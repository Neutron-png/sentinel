#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: Option<String>,
    pub swagger: Option<String>,
    pub info: Option<SpecInfo>,
    pub servers: Option<Vec<SpecServer>>,
    pub paths: Option<serde_json::Map<String, serde_json::Value>>,
    pub components: Option<serde_json::Value>,
    pub security: Option<Vec<serde_json::Value>>,
    pub tags: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecInfo {
    pub title: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecServer {
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    pub method: String,
    pub path: String,
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub parameters: Vec<ApiParameter>,
    pub request_body: Option<RequestBodySchema>,
    pub responses: Vec<ApiResponse>,
    pub security: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ApiParameter {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub param_type: String,
    pub description: Option<String>,
    pub example: Option<String>,
    pub schema_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestBodySchema {
    pub content_type: String,
    pub schema_ref: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status_code: String,
    pub description: String,
    pub schema_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthScheme {
    pub scheme_type: String,
    pub scheme_name: String,
    pub location: String,
    pub scheme_format: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub name: String,
    pub schema_type: String,
    pub properties: Vec<(String, String)>,
    pub required_fields: Vec<String>,
    pub enum_values: Option<Vec<String>>,
}
