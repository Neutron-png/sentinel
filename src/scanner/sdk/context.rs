#![allow(dead_code)]

use std::sync::Arc;

use crate::network::client::HttpClient;
use crate::network::models::HttpRequest;
use crate::scope::engine::ScopeEngine;

pub struct ScanRuleContext {
    pub http_client: Arc<HttpClient>,
    pub scope: Option<Arc<ScopeEngine>>,
    pub assessment_id: Option<uuid::Uuid>,
    pub target: String,
    pub config: RuleConfig,
    pub logger: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RuleConfig {
    pub max_requests: usize,
    pub request_delay_ms: u64,
    pub timeout_secs: u64,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            max_requests: 10,
            request_delay_ms: 100,
            timeout_secs: 30,
        }
    }
}

impl ScanRuleContext {
    pub fn new(http_client: Arc<HttpClient>, target: &str) -> Self {
        Self {
            http_client,
            scope: None,
            assessment_id: None,
            target: target.to_string(),
            config: RuleConfig::default(),
            logger: Vec::new(),
        }
    }

    pub fn with_scope(mut self, scope: Arc<ScopeEngine>) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn with_assessment(mut self, id: uuid::Uuid) -> Self {
        self.assessment_id = Some(id);
        self
    }

    pub fn build_get(&self, path: &str) -> HttpRequest {
        let url = format!("{}{}", self.target.trim_end_matches('/'), path);
        HttpRequest {
            method: "GET".to_string(),
            url,
            headers: vec![],
            cookies: vec![],
            body: crate::network::models::HttpBody::Empty,
            query_params: vec![],
        }
    }

    pub fn build_post(&self, path: &str, body: &str) -> HttpRequest {
        let url = format!("{}{}", self.target.trim_end_matches('/'), path);
        HttpRequest {
            method: "POST".to_string(),
            url,
            headers: vec![],
            cookies: vec![],
            body: crate::network::models::HttpBody::Text(body.to_string()),
            query_params: vec![],
        }
    }

    pub fn log(&mut self, msg: &str) {
        self.logger.push(msg.to_string());
    }
    pub fn clear_log(&mut self) {
        self.logger.clear();
    }

    pub async fn send(
        &self,
        request: &HttpRequest,
    ) -> Result<crate::network::models::HttpResponse, crate::network::errors::NetworkError> {
        self.http_client.execute(request.clone()).await
    }
}
