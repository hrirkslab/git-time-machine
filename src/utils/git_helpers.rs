use chrono::{DateTime, TimeZone, Utc};
use git2::{
    BlameOptions, Commit, DiffOptions, ObjectType, Repository, Signature,
};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::{debug, error};
use crate::models::tool_schema::ChangeType;

/// Custom error type for Git operations
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid commit: {0}")]
    InvalidCommit(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

/// Simplified commit information
pub struct SimpleCommit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub timestamp: String,
}

/// Information for a file's blame
pub struct BlameLineInfo {
    pub line_number: usize,
    pub content: String,
    pub commit_sha: String,
    pub commit_message: String,
    pub author: String,
    pub email: String,
    pub timestamp: String,
}

/// Information about file changes in a diff
pub struct DiffInfo {
    pub commit_sha: String,
    pub commit_message: String,
    pub author: String,
    pub email: String,
    pub timestamp: String,
    pub changes: Vec<FileChangeInfo>,
}

/// Information about a file change
pub struct FileChangeInfo {
    pub path: String,
    pub change_type: ChangeType,
    pub diff: Option<String>,
    pub additions: usize,
    pub deletions: usize,
}

/// Information for a diff summary
pub struct DiffSummary {
    pub base_commit: SimpleCommit,
    pub head_commit: SimpleCommit,
    pub summary: String,
    pub changes: Vec<FileChangeInfo>,
}

/// Information for a file at a specific commit
pub struct FileAtCommitInfo {
    pub commit: SimpleCommit,
    pub content: String,
}

/// Get the repository path, defaulting to the current directory
fn get_repo_path() -> PathBuf {
    // In a real application, this might come from configuration
    // or be passed as a parameter
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Open the git repository
fn open_repo() -> Result<Repository, GitError> {
    let repo_path = get_repo_path();
    debug!("Opening repository at path: {:?}", repo_path);
    let repo = Repository::open(repo_path)?;
    Ok(repo)
}

/// Convert a git signature to author name, email, and timestamp
fn signature_to_info(sig: &Signature) -> (String, String, String) {
    let name = sig.name().unwrap_or("Unknown").to_string();
    let email = sig.email().unwrap_or("unknown@example.com").to_string();
    
    // Convert git timestamp to ISO format
    let time = sig.when();
    let dt = match Utc.timestamp_opt(time.seconds(), 0) {
        chrono::offset::LocalResult::Single(dt) => dt,
        _ => DateTime::default(),
    };
    
    let timestamp = dt.to_rfc3339();
    
    (name, email, timestamp)
}

/// Convert a git commit to our simplified commit format
fn commit_to_simple(commit: &Commit) -> SimpleCommit {
    let message = commit.message().unwrap_or("").to_string();
    let sha = commit.id().to_string();
    
    let (author, email, timestamp) = signature_to_info(&commit.author());
    
    SimpleCommit {
        sha,
        message,
        author,
        email,
        timestamp,
    }
}

/// Get blame information for a file
pub fn get_file_blame(file_path: &str) -> Result<Vec<BlameLineInfo>, GitError> {
    let repo = open_repo()?;
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(GitError::FileNotFound(file_path.to_string()));
    }
    
    let mut blame_opts = BlameOptions::new();
    let blame = repo.blame_file(path, Some(&mut blame_opts))?;
    
    let mut result = Vec::new();
    let file_content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = file_content.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        let hunk = match blame.get_line(i + 1) {
            Some(hunk) => hunk,
            None => continue,
        };
        
        let commit_id = hunk.final_commit_id();
        let commit = repo.find_commit(commit_id)?;
        let simple_commit = commit_to_simple(&commit);
        
        result.push(BlameLineInfo {
            line_number: i + 1,
            content: line.to_string(),
            commit_sha: simple_commit.sha,
            commit_message: simple_commit.message,
            author: simple_commit.author,
            email: simple_commit.email,
            timestamp: simple_commit.timestamp,
        });
    }
    
    Ok(result)
}

/// Get the diff for a specific commit
pub fn get_commit_diff(sha: &str) -> Result<DiffInfo, GitError> {
    let repo = open_repo()?;
    
    // Find the commit by SHA
    let oid = git2::Oid::from_str(sha)
        .map_err(|_| GitError::InvalidCommit(sha.to_string()))?;
    let commit = repo.find_commit(oid)?;
    
    // Get the commit's parent
    let parent = match commit.parent(0) {
        Ok(parent) => parent,
        Err(_) => {
            // For initial commits with no parent
            let empty_tree = repo.find_tree(repo.treebuilder(None)?.write()?)?;
            let mut diff_opts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(Some(&empty_tree), Some(&commit.tree()?), Some(&mut diff_opts))?;
            
            let simple_commit = commit_to_simple(&commit);
            let changes = process_diff(&diff)?;
            
            return Ok(DiffInfo {
                commit_sha: simple_commit.sha,
                commit_message: simple_commit.message,
                author: simple_commit.author,
                email: simple_commit.email,
                timestamp: simple_commit.timestamp,
                changes,
            });
        }
    };
    
    // Get the diff between the commit and its parent
    let parent_tree = parent.tree()?;
    let commit_tree = commit.tree()?;
    let mut diff_opts = DiffOptions::new();
    diff_opts.show_binary(true).context_lines(3);
    
    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(&mut diff_opts))?;
    
    let simple_commit = commit_to_simple(&commit);
    let changes = process_diff(&diff)?;
    
    Ok(DiffInfo {
        commit_sha: simple_commit.sha,
        commit_message: simple_commit.message,
        author: simple_commit.author,
        email: simple_commit.email,
        timestamp: simple_commit.timestamp,
        changes,
    })
}

/// Process a git diff into our format
fn process_diff(diff: &git2::Diff) -> Result<Vec<FileChangeInfo>, GitError> {
    // First, collect file paths and change types
    let mut changes = Vec::new();
    let mut file_diffs = std::collections::HashMap::new();
    
    // First pass: collect file paths and change types
    diff.foreach(
        &mut |delta, _| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            let change_type = match delta.status() {
                git2::Delta::Added => ChangeType::Added,
                git2::Delta::Deleted => ChangeType::Deleted,
                git2::Delta::Renamed => ChangeType::Renamed,
                _ => ChangeType::Modified,
            };
            
            changes.push(FileChangeInfo {
                path: path.clone(),
                change_type,
                diff: Some(String::new()), // Initialize with empty string instead of None
                additions: 0,
                deletions: 0,
            });
            
            // Initialize an entry in our hashmap for this file
            file_diffs.insert(path, (changes.len() - 1, String::new()));
            
            true
        },
        None,
        None,
        None
    )?;
    
    // Second pass: collect line changes
    diff.foreach(
        &mut |_, _| { true }, // File callback (required)
        None,
        None,
        Some(&mut |delta, hunk, line| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            if let Some((idx, diff_content)) = file_diffs.get_mut(&path) {
                // Add hunk header if it exists and we're at the start of a hunk
                if let Some(h) = hunk {
                    if diff_content.is_empty() ||
                        !diff_content.contains(&format!("@@ -{},{} +{},{} @@",
                            h.old_start(), h.old_lines(), h.new_start(), h.new_lines())) {
                        let hunk_str = format!("@@ -{},{} +{},{} @@\n", 
                            h.old_start(), h.old_lines(), h.new_start(), h.new_lines());
                        diff_content.push_str(&hunk_str);
                    }
                }
                
                // Add the line with its prefix
                let prefix = match line.origin() {
                    '+' => {
                        // Update additions counter in the actual changes vector
                        changes[*idx].additions += 1;
                        "+"
                    },
                    '-' => {
                        // Update deletions counter in the actual changes vector
                        changes[*idx].deletions += 1;
                        "-"
                    },
                    _ => " ",
                };
                
                let content = std::str::from_utf8(line.content()).unwrap_or("");
                diff_content.push_str(&format!("{}{}", prefix, content));
            }
            
            true
        }),
    )?;
    
    // Now update all the changes with the collected diff content
    for (path, (idx, diff_content)) in file_diffs {
        if let Some(change) = changes.get_mut(idx) {
            change.diff = Some(diff_content);
        }
    }
    
    Ok(changes)
}

/// Generate a summary of changes between two commits
pub fn summarize_diff(base_sha: &str, head_sha: &str) -> Result<DiffSummary, GitError> {
    let repo = open_repo()?;
    
    // Find the base and head commits
    let base_oid = git2::Oid::from_str(base_sha)
        .map_err(|_| GitError::InvalidCommit(base_sha.to_string()))?;
    let head_oid = git2::Oid::from_str(head_sha)
        .map_err(|_| GitError::InvalidCommit(head_sha.to_string()))?;
    
    let base_commit = repo.find_commit(base_oid)?;
    let head_commit = repo.find_commit(head_oid)?;
    
    // Get the trees for diffing
    let base_tree = base_commit.tree()?;
    let head_tree = head_commit.tree()?;
    
    // Compute the diff
    let mut diff_opts = DiffOptions::new();
    diff_opts.show_binary(true).context_lines(3);
    
    let diff = repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut diff_opts))?;
    let changes = process_diff(&diff)?;
    
    // In a real application, this would call an LLM API to generate a summary
    // For now, we'll create a simple summary manually
    let simple_base_commit = commit_to_simple(&base_commit);
    let simple_head_commit = commit_to_simple(&head_commit);
    
    let stats = diff.stats()?;
    let summary = format!(
        "Changes between {} and {}: {} files changed, {} insertions(+), {} deletions(-)",
        &base_sha[0..7],
        &head_sha[0..7],
        stats.files_changed(),
        stats.insertions(),
        stats.deletions()
    );
    
    Ok(DiffSummary {
        base_commit: simple_base_commit,
        head_commit: simple_head_commit,
        summary,
        changes,
    })
}

/// Get a list of commits that modified a file
pub fn get_commits_affecting_file(
    file_path: &str,
    limit: Option<usize>,
) -> Result<Vec<SimpleCommit>, GitError> {
    let repo = open_repo()?;
    
    // Create a revwalk to iterate through commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    
    let limit = limit.unwrap_or(50); // Default limit to avoid excessive results
    let mut result = Vec::new();
    // The path variable is used in the diff_opts.pathspec call below
    let _path = Path::new(file_path);
    
    // Find commits that modified this file
    for oid in revwalk {
        if result.len() >= limit {
            break;
        }
        
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        
        // Skip merge commits for simplicity
        if commit.parent_count() > 1 {
            continue;
        }
        
        let parent = if commit.parent_count() > 0 {
            Some(commit.parent(0)?)
        } else {
            None
        };
        
        let commit_tree = commit.tree()?;
        let parent_tree = match &parent {
            Some(parent) => Some(parent.tree()?),
            None => None,
        };
        
        // Compare the trees to see if this file changed
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(file_path);
        
        let diff = match parent_tree {
            Some(parent_tree) => repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(&mut diff_opts))?,
            None => {
                // For the initial commit
                let empty_tree = repo.find_tree(repo.treebuilder(None)?.write()?)?;
                repo.diff_tree_to_tree(Some(&empty_tree), Some(&commit_tree), Some(&mut diff_opts))?
            }
        };
        
        // If the diff has deltas, this commit modified the file
        if diff.deltas().count() > 0 {
            result.push(commit_to_simple(&commit));
        }
    }
    
    Ok(result)
}

/// Get a file as it existed at a specific commit
pub fn get_file_at_commit(file_path: &str, sha: &str) -> Result<FileAtCommitInfo, GitError> {
    let repo = open_repo()?;
    
    // Find the commit
    let oid = git2::Oid::from_str(sha)
        .map_err(|_| GitError::InvalidCommit(sha.to_string()))?;
    let commit = repo.find_commit(oid)?;
    
    // Get the tree for this commit
    let tree = commit.tree()?;
    
    // Find the file in the tree
    let entry = tree.get_path(Path::new(file_path))
        .map_err(|_| GitError::FileNotFound(format!("{} at commit {}", file_path, sha)))?;
    
    // Get the object and read its content
    let object = repo.find_object(entry.id(), Some(ObjectType::Blob))?;
    let blob = object.as_blob().ok_or_else(|| GitError::Other("Not a blob".to_string()))?;
    
    let content = match std::str::from_utf8(blob.content()) {
        Ok(content) => content.to_string(),
        Err(_) => {
            error!("File content is not valid UTF-8");
            return Err(GitError::Other("File content is not valid UTF-8".to_string()));
        }
    };
    
    Ok(FileAtCommitInfo {
        commit: commit_to_simple(&commit),
        content,
    })
}
