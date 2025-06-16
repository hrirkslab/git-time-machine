use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info};

use crate::{
    models::tool_schema::{
        CommitInfo, CommitsAffectingRequest, CommitsAffectingResponse, FileAtCommitRequest,
        FileAtCommitResponse,
    },
    utils::git_helpers,
};

/// Get a list of commits that modified a file
///
/// Returns metadata for commits that affected the given file
#[utoipa::path(
    post,
    path = "/tools/get_commits_affecting",
    request_body = CommitsAffectingRequest,
    responses(
        (status = 200, description = "Commits retrieved successfully", body = CommitsAffectingResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_commits_affecting(Json(request): Json<CommitsAffectingRequest>) -> Response {
    info!(
        "Processing get_commits_affecting request for file: {}",
        request.file
    );
    
    match git_helpers::get_commits_affecting_file(&request.file, request.limit) {
        Ok(commits_data) => {
            let commits = commits_data
                .into_iter()
                .map(|commit| CommitInfo {
                    sha: commit.sha,
                    message: commit.message,
                    author: commit.author,
                    email: commit.email,
                    timestamp: commit.timestamp,
                })
                .collect();

            let response = CommitsAffectingResponse {
                file: request.file,
                commits,
            };

            Json(response).into_response()
        }
        Err(e) => {
            error!("Error getting commits: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error getting commits: {}", e),
            )
                .into_response()
        }
    }
}

/// Get a file as it existed at a specific commit
///
/// Returns the file content at the given commit
#[utoipa::path(
    post,
    path = "/tools/get_file_at_commit",
    request_body = FileAtCommitRequest,
    responses(
        (status = 200, description = "File retrieved successfully", body = FileAtCommitResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_file_at_commit(Json(request): Json<FileAtCommitRequest>) -> Response {
    info!(
        "Processing get_file_at_commit request for file: {} at commit: {}",
        request.file, request.sha
    );
    
    match git_helpers::get_file_at_commit(&request.file, &request.sha) {
        Ok(file_data) => {
            let response = FileAtCommitResponse {
                file: request.file,
                commit: CommitInfo {
                    sha: file_data.commit.sha,
                    message: file_data.commit.message,
                    author: file_data.commit.author,
                    email: file_data.commit.email,
                    timestamp: file_data.commit.timestamp,
                },
                content: file_data.content,
            };

            Json(response).into_response()
        }
        Err(e) => {
            error!("Error getting file: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error getting file: {}", e),
            )
                .into_response()
        }
    }
}
