#![allow(dead_code)]

use std::sync::Arc;

use crate::network::client::HttpClient;
use crate::network::models::HttpRequest;
use crate::scope::engine::ScopeEngine;

pub struct ScanContext {
    pub http_client: Arc<HttpClient>,
    pub scope: Option<Arc<ScopeEngine>>,
    pub target: String,
    pub assessment_id: uuid::Uuid,
}

impl ScanContext {
    pub fn new(http_client: Arc<HttpClient>, assessment_id: uuid::Uuid, target: &str) -> Self {
        Self {
            http_client,
            scope: None,
            target: target.to_string(),
            assessment_id,
        }
    }

    pub fn with_scope(mut self, scope: Arc<ScopeEngine>) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn build_request(&self, method: &str, path: &str) -> HttpRequest {
        let url = format!("{}{}", self.target.trim_end_matches('/'), path);
        HttpRequest {
            method: method.to_string(),
            url,
            headers: vec![],
            cookies: vec![],
            body: super::super::super::network::models::HttpBody::Empty,
            query_params: vec![],
        }
    }

    pub fn is_in_scope(&self, url: &str) -> bool {
        if let Some(ref s) = self.scope {
            s.is_in_scope("", 443, "https", url)
        } else {
            true
        }
    }
}
