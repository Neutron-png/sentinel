#![allow(dead_code)]

use std::sync::Arc;
use tokio::sync::watch;

use crate::network::client::HttpClient;

pub struct PipelineContext {
    pub http_client: Arc<HttpClient>,
    pub target: String,
    pub rule_id: String,
    pub cancellation: watch::Receiver<bool>,
    pub request_count: usize,
    pub response_count: usize,
    pub error_count: usize,
}

impl PipelineContext {
    pub fn new(
        http_client: Arc<HttpClient>,
        target: &str,
        rule_id: &str,
        cancel: watch::Receiver<bool>,
    ) -> Self {
        Self {
            http_client,
            target: target.to_string(),
            rule_id: rule_id.to_string(),
            cancellation: cancel,
            request_count: 0,
            response_count: 0,
            error_count: 0,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        *self.cancellation.borrow()
    }
}
