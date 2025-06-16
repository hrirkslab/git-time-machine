use crate::models::git::{ApiInfo, AuthType, McpMetadata, PluginManifest};
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::info;
use utoipa::OpenApi;

// Define OpenAPI documentation for our tools
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::blame::get_git_blame,
        crate::handlers::diff::get_commit_diff,
        crate::handlers::diff::summarize_diff,
        crate::handlers::commits::get_commits_affecting,
        crate::handlers::commits::get_file_at_commit,
    ),
    components(
        schemas(
            crate::models::tool_schema::BlameRequest,
            crate::models::tool_schema::BlameResponse,
            crate::models::tool_schema::BlameLine,
            crate::models::tool_schema::CommitDiffRequest, 
            crate::models::tool_schema::CommitDiffResponse,
            crate::models::tool_schema::SummarizeDiffRequest, 
            crate::models::tool_schema::SummarizeDiffResponse,
            crate::models::tool_schema::CommitsAffectingRequest,
            crate::models::tool_schema::CommitsAffectingResponse,
            crate::models::tool_schema::FileAtCommitRequest,
            crate::models::tool_schema::FileAtCommitResponse,
            crate::models::tool_schema::CommitInfo,
            crate::models::tool_schema::FileChange,
            crate::models::tool_schema::ChangeType,
            crate::models::git::McpMetadata,
        )
    ),
    tags(
        (name = "Git Time Machine", description = "Git history exploration tools")
    )
)]
struct ApiDoc;

/// Get the AI plugin manifest for integration with AI assistants
pub async fn get_plugin_manifest() -> impl IntoResponse {
    info!("Serving plugin manifest");
    
    let manifest = PluginManifest {
        schema_version: "v1".to_string(),
        name_for_human: "Git Time Machine".to_string(),
        name_for_model: "git_time_machine".to_string(),
        description_for_human: "Explore Git history, diffs, and file evolution over time.".to_string(),
        description_for_model: "Use this plugin to explore Git history, diffs, and file evolution over time. You can retrieve blame information, commit diffs, file history, and more.".to_string(),
        auth: AuthType {
            auth_type: "none".to_string(),
        },
        api: ApiInfo {
            api_type: "openapi".to_string(),
            url: "/openapi.json".to_string(),
        },
        logo_url: "https://example.com/logo.png".to_string(),
        contact_email: "contact@example.com".to_string(),
        legal_info_url: "https://example.com/legal".to_string(),
    };

    Json(manifest)
}

/// Get the MCP metadata for this server
pub async fn get_metadata() -> impl IntoResponse {
    info!("Serving MCP metadata");

    let metadata = McpMetadata {
        name: "Git Time Machine".to_string(),
        description: "A tool for exploring Git history and tracking file evolution over time".to_string(),
        tools_json_schema: "/openapi.json".to_string(),
        tools_url: "/tools".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(metadata)
}

/// Get OpenAPI schema for this server's tools
pub async fn get_openapi_schema() -> impl IntoResponse {
    info!("Serving OpenAPI schema");
    
    let openapi = ApiDoc::openapi();
    let openapi_json = serde_json::to_string_pretty(&openapi).unwrap_or_default();
    
    (StatusCode::OK, openapi_json)
}
