use serde::Serialize;
use utoipa::ToSchema;

/// MCP Plugin Manifest
#[derive(Serialize, Debug)]
pub struct PluginManifest {
    pub schema_version: String,
    pub name_for_human: String,
    pub name_for_model: String,
    pub description_for_human: String,
    pub description_for_model: String,
    pub auth: AuthType,
    pub api: ApiInfo,
    pub logo_url: String,
    pub contact_email: String,
    pub legal_info_url: String,
}

/// Auth type for MCP
#[derive(Serialize, Debug)]
pub struct AuthType {
    #[serde(rename = "type")]
    pub auth_type: String,
}

/// API info for MCP
#[derive(Serialize, Debug)]
pub struct ApiInfo {
    #[serde(rename = "type")]
    pub api_type: String,
    pub url: String,
}

/// MCP Metadata
#[derive(Serialize, Debug, ToSchema)]
pub struct McpMetadata {
    pub name: String,
    pub description: String,
    pub tools_json_schema: String,
    pub tools_url: String,
    pub version: String,
}

/// Error response for API errors
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}
