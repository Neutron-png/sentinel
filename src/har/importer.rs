#![allow(dead_code)]

use crate::har::errors::HarError;
use crate::har::events::{HarEvent, HarEventBus};
use crate::har::models::HarLog;
use crate::network::models::{HttpBody, HttpRequest, HttpResponse};

pub struct HarImporter {
    event_bus: HarEventBus,
}

impl HarImporter {
    pub fn new() -> Self {
        Self {
            event_bus: HarEventBus::new(256),
        }
    }

    pub fn event_bus(&self) -> HarEventBus {
        self.event_bus.clone()
    }

    pub fn import_file(&self, path: &str) -> Result<HarLog, HarError> {
        self.event_bus.emit(HarEvent::ImportStarted {
            path: path.to_string(),
        });
        let content = std::fs::read_to_string(path).map_err(|e| HarError::Import(e.to_string()))?;
        let log: HarLog =
            serde_json::from_str(&content).map_err(|e| HarError::Parse(e.to_string()))?;
        if log.version != "1.2" {
            return Err(HarError::InvalidFormat(format!(
                "Version {} not supported",
                log.version
            )));
        }
        self.event_bus.emit(HarEvent::ImportFinished {
            entries: log.entries.len(),
        });
        Ok(log)
    }

    pub fn entries_to_requests(log: &HarLog) -> Vec<(HttpRequest, HttpResponse)> {
        log.entries
            .iter()
            .map(|entry| {
                let _h: Vec<(String, String)> = entry
                    .request
                    .headers
                    .iter()
                    .map(|h| (h.name.clone(), h.value.clone()))
                    .collect();
                let query: Vec<(String, String)> = entry
                    .request
                    .query_string
                    .iter()
                    .map(|q| (q.name.clone(), q.value.clone()))
                    .collect();
                let request = HttpRequest {
                    method: entry.request.method.clone(),
                    url: entry.request.url.clone(),
                    headers: vec![],
                    cookies: vec![],
                    body: HttpBody::Empty,
                    query_params: query,
                };
                let resp_headers = entry
                    .response
                    .headers
                    .iter()
                    .map(|h| crate::network::models::HttpHeader {
                        name: h.name.clone(),
                        value: h.value.clone(),
                    })
                    .collect();
                let response = HttpResponse {
                    status_code: entry.response.status,
                    status_text: entry.response.status_text.clone(),
                    headers: resp_headers,
                    cookies: vec![],
                    body: HttpBody::Empty,
                    protocol: entry.request.http_version.clone(),
                    content_type: Some(entry.response.content.mime_type.clone()),
                    content_length: Some(entry.response.content.size as u64),
                    timing: Default::default(),
                    url: entry.request.url.clone(),
                };
                (request, response)
            })
            .collect()
    }
}
