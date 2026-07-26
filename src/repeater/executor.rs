#![allow(dead_code)]

use std::sync::Arc;

use crate::network::client::HttpClient;
use crate::repeater::errors::RepeaterError;
use crate::repeater::history;
use crate::repeater::models::{RepeaterTab, ResponseEntry};

pub struct RequestExecutor {
    client: Arc<HttpClient>,
}

impl RequestExecutor {
    pub fn new(client: Arc<HttpClient>) -> Self {
        Self { client }
    }

    pub async fn send_request(
        &self,
        tab: &mut RepeaterTab,
    ) -> Result<ResponseEntry, RepeaterError> {
        let start = std::time::Instant::now();
        let response = self
            .client
            .execute(tab.request.clone())
            .await
            .map_err(|e| RepeaterError::Send(e.to_string()))?;
        let duration = start.elapsed().as_millis() as u64;
        history::record_response(tab, response, duration);
        Ok(tab.latest_response.clone().unwrap())
    }

    pub async fn send_multiple(
        &self,
        tab: &mut RepeaterTab,
        count: usize,
    ) -> Result<Vec<ResponseEntry>, RepeaterError> {
        let mut results = Vec::new();
        for _ in 0..count {
            let entry = self.send_request(tab).await?;
            results.push(entry);
        }
        Ok(results)
    }

    pub async fn cancel_request(&self, _tab: &RepeaterTab) {
        // Async cancellation would require future abort handling
        // For now, this is a no-op placeholder
    }
}
