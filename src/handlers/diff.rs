use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info};

use crate::{
    models::tool_schema::{
        CommitDiffRequest, CommitDiffResponse, CommitInfo, FileChange,
        SummarizeDiffRequest, SummarizeDiffResponse,
    },
    utils::git_helpers,
};

/// Get the full diff for a specific commit
///
/// Returns the complete diff and metadata for a specific commit
#[utoipa::path(
    post,
    path = "/tools/get_commit_diff",
    request_body = CommitDiffRequest,
    responses(
        (status = 200, description = "Commit diff retrieved successfully", body = CommitDiffResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_commit_diff(Json(request): Json<CommitDiffRequest>) -> Response {
    info!("Processing get_commit_diff request for SHA: {}", request.sha);
    
    match git_helpers::get_commit_diff(&request.sha) {
        Ok(diff_data) => {
            let response = CommitDiffResponse {
                commit: CommitInfo {
                    sha: diff_data.commit_sha,
                    message: diff_data.commit_message,
                    author: diff_data.author,
                    email: diff_data.email,
                    timestamp: diff_data.timestamp,
                },
                changes: diff_data
                    .changes
                    .into_iter()
                    .map(|change| FileChange {
                        path: change.path,
                        change_type: change.change_type,
                        diff: change.diff,
                        additions: change.additions,
                        deletions: change.deletions,
                    })
                    .collect(),
            };

            Json(response).into_response()
        }
        Err(e) => {
            error!("Error getting commit diff: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error getting commit diff: {}", e),
            )
                .into_response()
        }
    }
}

/// Summarize the differences between two commits
///
/// Compares two commits and produces a human-readable summary of the changes
#[utoipa::path(
    post,
    path = "/tools/summarize_diff",
    request_body = SummarizeDiffRequest,
    responses(
        (status = 200, description = "Diff summary generated successfully", body = SummarizeDiffResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn summarize_diff(Json(request): Json<SummarizeDiffRequest>) -> Response {
    info!(
        "Processing summarize_diff request for base: {} and head: {}",
        request.base, request.head
    );
    
    match git_helpers::summarize_diff(&request.base, &request.head) {
        Ok(summary_data) => {
            let response = SummarizeDiffResponse {
                base_commit: CommitInfo {
                    sha: summary_data.base_commit.sha,
                    message: summary_data.base_commit.message,
                    author: summary_data.base_commit.author,
                    email: summary_data.base_commit.email,
                    timestamp: summary_data.base_commit.timestamp,
                },
                head_commit: CommitInfo {
                    sha: summary_data.head_commit.sha,
                    message: summary_data.head_commit.message,
                    author: summary_data.head_commit.author,
                    email: summary_data.head_commit.email,
                    timestamp: summary_data.head_commit.timestamp,
                },
                summary: summary_data.summary,
                changes: summary_data
                    .changes
                    .into_iter()
                    .map(|change| FileChange {
                        path: change.path,
                        change_type: change.change_type,
                        diff: change.diff,
                        additions: change.additions,
                        deletions: change.deletions,
                    })
                    .collect(),
            };

            Json(response).into_response()
        }
        Err(e) => {
            error!("Error summarizing diff: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error summarizing diff: {}", e),
            )
                .into_response()
        }
    }
}
