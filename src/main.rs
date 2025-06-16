mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    info!("Starting Git Time Machine MCP server");

    // CORS middleware to allow requests from anywhere
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the application router
    let app = Router::new()
        // MCP metadata endpoint
        .route("/.well-known/ai-plugin.json", get(handlers::metadata::get_plugin_manifest))
        .route("/metadata", get(handlers::metadata::get_metadata))
        
        // Tool endpoints
        .route("/tools/get_git_blame", post(handlers::blame::get_git_blame))
        .route("/tools/get_commit_diff", post(handlers::diff::get_commit_diff))
        .route("/tools/summarize_diff", post(handlers::diff::summarize_diff))
        .route("/tools/get_commits_affecting", post(handlers::commits::get_commits_affecting))
        .route("/tools/get_file_at_commit", post(handlers::commits::get_file_at_commit))
        
        // OpenAPI schema for tools
        .route("/openapi.json", get(handlers::metadata::get_openapi_schema))
        
        // Add middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Set up the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on {}", addr);
    
    // Start the server
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;
        
    Ok(())
}
