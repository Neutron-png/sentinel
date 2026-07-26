#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CorrelationInfo {
    pub request_id: Uuid,
    pub browser_id: Uuid,
    pub tab_id: Uuid,
    pub history_entry_id: Option<Uuid>,
    pub sitemap_endpoint_id: Option<Uuid>,
    pub proxy_connection_id: Option<String>,
}

pub struct RequestCorrelator {
    correlations: Vec<CorrelationInfo>,
}

impl RequestCorrelator {
    pub fn new() -> Self {
        Self {
            correlations: Vec::new(),
        }
    }

    pub fn add_correlation(&mut self, info: CorrelationInfo) {
        self.correlations.push(info);
    }

    pub fn find_by_request(&self, request_id: Uuid) -> Option<&CorrelationInfo> {
        self.correlations
            .iter()
            .find(|c| c.request_id == request_id)
    }

    pub fn find_by_browser(&self, browser_id: Uuid) -> Vec<&CorrelationInfo> {
        self.correlations
            .iter()
            .filter(|c| c.browser_id == browser_id)
            .collect()
    }

    pub fn link_history(&mut self, request_id: Uuid, history_id: Uuid) {
        if let Some(c) = self
            .correlations
            .iter_mut()
            .find(|c| c.request_id == request_id)
        {
            c.history_entry_id = Some(history_id);
        }
    }

    pub fn link_sitemap(&mut self, request_id: Uuid, endpoint_id: Uuid) {
        if let Some(c) = self
            .correlations
            .iter_mut()
            .find(|c| c.request_id == request_id)
        {
            c.sitemap_endpoint_id = Some(endpoint_id);
        }
    }
}
