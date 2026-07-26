#![allow(dead_code)]

use crate::network::models::{HttpBody, HttpRequest};
use crate::openapi::models::ApiEndpoint;

pub fn generate_request(endpoint: &ApiEndpoint, base_url: &str) -> HttpRequest {
    let url = format!("{}{}", base_url.trim_end_matches('/'), endpoint.path);
    let query_params: Vec<(String, String)> = endpoint
        .parameters
        .iter()
        .filter(|p| p.location == "query")
        .map(|p| (p.name.clone(), p.example.clone().unwrap_or_default()))
        .collect();
    let body = match &endpoint.request_body {
        Some(_) => HttpBody::Text("{}".into()),
        None => HttpBody::Empty,
    };
    HttpRequest {
        method: endpoint.method.clone(),
        url,
        headers: vec![],
        cookies: vec![],
        body,
        query_params,
    }
}

pub fn generate_all(endpoints: &[ApiEndpoint], base_url: &str) -> Vec<HttpRequest> {
    endpoints
        .iter()
        .map(|e| generate_request(e, base_url))
        .collect()
}

pub fn extract_injectable_params(endpoint: &ApiEndpoint) -> Vec<(String, String)> {
    endpoint
        .parameters
        .iter()
        .filter(|p| p.location == "query" || p.location == "path" || p.location == "header")
        .map(|p| (p.location.clone(), p.name.clone()))
        .collect()
}
