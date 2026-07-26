#![allow(dead_code)]

use crate::openapi::discovery::SpecDiscovery;
use crate::openapi::errors::OpenApiError;
use crate::openapi::generator;
use crate::openapi::models::{ApiEndpoint, AuthScheme, OpenApiSpec};
use crate::openapi::parser;
use crate::openapi::schema;
use crate::openapi::security;
use crate::openapi::validator;

pub struct OpenApiEngine {
    spec: Option<OpenApiSpec>,
    endpoints: Vec<ApiEndpoint>,
    auth_schemes: Vec<AuthScheme>,
    base_url: String,
}

impl OpenApiEngine {
    pub fn new(base_url: &str) -> Self {
        Self {
            spec: None,
            endpoints: Vec::new(),
            auth_schemes: Vec::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn import(&mut self, content: &str, is_yaml: bool) -> Result<(), OpenApiError> {
        let spec = parser::parse_spec(content, is_yaml)?;
        validator::validate(&spec)?;
        self.endpoints = parser::extract_endpoints(&spec, &self.base_url);
        self.auth_schemes = security::extract_security(&spec);
        self.spec = Some(spec);
        Ok(())
    }

    pub fn discover_paths(&self) -> Vec<&'static str> {
        SpecDiscovery::common_paths()
    }
    pub fn is_spec_response(&self, body: &str) -> bool {
        SpecDiscovery::is_spec_response(body)
    }

    pub fn endpoints(&self) -> &[ApiEndpoint] {
        &self.endpoints
    }
    pub fn auth_schemes(&self) -> &[AuthScheme] {
        &self.auth_schemes
    }
    pub fn spec(&self) -> Option<&OpenApiSpec> {
        self.spec.as_ref()
    }

    pub fn generate_requests(&self) -> Vec<crate::network::models::HttpRequest> {
        generator::generate_all(&self.endpoints, &self.base_url)
    }

    pub fn injectable_params(&self) -> Vec<(String, String, String)> {
        crate::openapi::integration::ScannerIntegration::extract_injectable(&self.endpoints)
    }

    pub fn parse_schema(
        &self,
        name: &str,
        spec: &serde_json::Value,
    ) -> Option<crate::openapi::models::SchemaObject> {
        schema::parse_schema(name, spec)
    }

    pub fn endpoint_count(&self) -> usize {
        self.endpoints.len()
    }
}
