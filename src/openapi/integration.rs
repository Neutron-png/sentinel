#![allow(dead_code)]

use crate::openapi::models::ApiEndpoint;

pub struct ScannerIntegration;

impl ScannerIntegration {
    pub fn extract_injectable(endpoints: &[ApiEndpoint]) -> Vec<(String, String, String)> {
        endpoints
            .iter()
            .flat_map(|ep| {
                ep.parameters
                    .iter()
                    .filter(|p| !p.required)
                    .map(|p| (ep.path.clone(), p.location.clone(), p.name.clone()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn is_idempotent(method: &str) -> bool {
        matches!(method.to_uppercase().as_str(), "GET" | "HEAD" | "OPTIONS")
    }
}
