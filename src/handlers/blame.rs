use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info};

use crate::{
    models::tool_schema::{BlameRequest, BlameResponse, BlameLine, CommitInfo},
    utils::git_helpers,
};

/// Get git blame information for a file
///
/// Returns line-by-line blame metadata showing who last modified each line
#[utoipa::path(
    post,
    path = "/tools/get_git_blame",
    request_body = BlameRequest,
    responses(
        (status = 200, description = "Blame information retrieved successfully", body = BlameResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_git_blame(Json(request): Json<BlameRequest>) -> Response {
    info!("Processing get_git_blame request for file: {}", request.file);
    
    match git_helpers::get_file_blame(&request.file) {
        Ok(blame_data) => {
            let blame_lines = blame_data
                .into_iter()
                .map(|line| BlameLine {
                    line_number: line.line_number,
                    content: line.content,
                    commit: CommitInfo {
                        sha: line.commit_sha,
                        message: line.commit_message,
                        author: line.author,
                        email: line.email,
                        timestamp: line.timestamp,
                    },
                })
                .collect();

            let response = BlameResponse {
                file: request.file,
                lines: blame_lines,
            };

            Json(response).into_response()
        }
        Err(e) => {
            error!("Error getting git blame: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error getting git blame: {}", e),
            )
                .into_response()
        }
    }
}
