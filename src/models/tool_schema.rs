use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request model for git blame operations
#[derive(Debug, Deserialize, ToSchema)]
pub struct BlameRequest {
    /// Path to the file to analyze
    pub file: String,
}

/// Response model for git blame operations
#[derive(Debug, Serialize, ToSchema)]
pub struct BlameResponse {
    /// The file path that was analyzed
    pub file: String,
    /// Line by line blame information
    pub lines: Vec<BlameLine>,
}

/// Information about a single line in a file
#[derive(Debug, Serialize, ToSchema)]
pub struct BlameLine {
    /// Line number (1-indexed)
    pub line_number: usize,
    /// Content of the line
    pub content: String,
    /// Commit information for this line
    pub commit: CommitInfo,
}

/// Request model for commit diff operations
#[derive(Debug, Deserialize, ToSchema)]
pub struct CommitDiffRequest {
    /// Git commit SHA
    pub sha: String,
}

/// Response model for commit diff operations
#[derive(Debug, Serialize, ToSchema)]
pub struct CommitDiffResponse {
    /// Commit information
    pub commit: CommitInfo,
    /// List of file changes in this commit
    pub changes: Vec<FileChange>,
}

/// Request model for summarizing diffs between commits
#[derive(Debug, Deserialize, ToSchema)]
pub struct SummarizeDiffRequest {
    /// Base commit SHA
    pub base: String,
    /// Head commit SHA
    pub head: String,
}

/// Response model for summarized diffs
#[derive(Debug, Serialize, ToSchema)]
pub struct SummarizeDiffResponse {
    /// Base commit information
    pub base_commit: CommitInfo,
    /// Head commit information
    pub head_commit: CommitInfo,
    /// Human-readable summary of the changes
    pub summary: String,
    /// List of file changes between these commits
    pub changes: Vec<FileChange>,
}

/// Request model for getting commits affecting a file
#[derive(Debug, Deserialize, ToSchema)]
pub struct CommitsAffectingRequest {
    /// Path to the file to analyze
    pub file: String,
    /// Optional limit on the number of commits to return
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Response model for commits affecting a file
#[derive(Debug, Serialize, ToSchema)]
pub struct CommitsAffectingResponse {
    /// Path to the file that was analyzed
    pub file: String,
    /// List of commits that modified this file
    pub commits: Vec<CommitInfo>,
}

/// Request model for getting a file at a specific commit
#[derive(Debug, Deserialize, ToSchema)]
pub struct FileAtCommitRequest {
    /// Path to the file
    pub file: String,
    /// Commit SHA
    pub sha: String,
}

/// Response model for file at commit
#[derive(Debug, Serialize, ToSchema)]
pub struct FileAtCommitResponse {
    /// Path to the file
    pub file: String,
    /// Commit information
    pub commit: CommitInfo,
    /// File content at the specified commit
    pub content: String,
}

/// Information about a Git commit
#[derive(Debug, Serialize, ToSchema)]
pub struct CommitInfo {
    /// Commit SHA
    pub sha: String,
    /// Commit message
    pub message: String,
    /// Author name
    pub author: String,
    /// Author email
    pub email: String,
    /// Timestamp of the commit (ISO format)
    pub timestamp: String,
}

/// Information about file changes in a commit
#[derive(Debug, Serialize, ToSchema)]
pub struct FileChange {
    /// Path to the file
    pub path: String,
    /// Type of change
    pub change_type: ChangeType,
    /// Diff for this file (if applicable)
    pub diff: Option<String>,
    /// Number of lines added
    pub additions: usize,
    /// Number of lines removed
    pub deletions: usize,
}

/// Types of changes that can happen to a file
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}
