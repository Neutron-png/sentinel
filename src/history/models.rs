#![allow(dead_code)]

use crate::network::models::{HttpRequest, HttpResponse};
use crate::network::protocol::{HttpVersion, NegotiatedProtocol};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Enhanced history entry with full protocol metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: String,
    pub url: String,
    pub protocol: String,
    pub status_code: u16,
    pub request_size: u64,
    pub response_size: u64,
    pub duration_ms: u64,
    pub tls_enabled: bool,
    pub source: String,
    pub tags: String,
    pub request_body: String,
    pub response_body: String,
    pub request_headers: String,
    pub response_headers: String,
    // Protocol-enriched fields
    pub connection_id: Option<Uuid>,
    pub stream_id: Option<u32>,
    pub frame_metadata_json: Option<String>,
    pub negotiated_alpn: Option<String>,
    pub protocol_version: String,
}

impl HistoryEntry {
    pub fn protocol_version_enum(&self) -> HttpVersion {
        HttpVersion::from_protocol_str(&self.protocol_version)
    }

    pub fn from_network_transaction(
        request: &HttpRequest,
        response: &HttpResponse,
        source: &str,
        negotiated: Option<&NegotiatedProtocol>,
    ) -> Self {
        let parsed = url::Url::parse(&request.url)
            .unwrap_or_else(|_| url::Url::parse("http://unknown/").unwrap());
        let protocol_version = HttpVersion::from_protocol_str(&response.protocol);
        let (connection_id, stream_id, frame_meta, alpn) = if let Some(neg) = negotiated {
            let conn_id = Some(neg.connection_id);
            let stream = neg.streams.last().map(|s| s.stream_id);
            let frames_json = if neg.frame_log.is_empty() {
                None
            } else {
                serde_json::to_string(&neg.frame_log).ok()
            };
            let alpn_str = neg.alpn.as_ref().and_then(|a| a.selected.clone());
            (conn_id, stream, frames_json, alpn_str)
        } else {
            (None, None, None, None)
        };

        Self {
            id: Uuid::new_v4(),
            transaction_id: None,
            timestamp: Utc::now(),
            method: request.method.clone(),
            scheme: parsed.scheme().to_string(),
            host: parsed.host_str().unwrap_or("").to_string(),
            port: parsed.port().unwrap_or(443),
            path: parsed.path().to_string(),
            query: parsed.query().unwrap_or("").to_string(),
            url: request.url.clone(),
            protocol: protocol_version.label().to_string(),
            status_code: response.status_code,
            request_size: request.body.len() as u64,
            response_size: response.body.len() as u64,
            duration_ms: response.timing.total_duration.as_millis() as u64,
            tls_enabled: parsed.scheme() == "https",
            source: source.to_string(),
            tags: String::new(),
            request_body: String::new(),
            response_body: String::new(),
            request_headers: String::new(),
            response_headers: String::new(),
            connection_id,
            stream_id,
            frame_metadata_json: frame_meta,
            negotiated_alpn: alpn,
            protocol_version: protocol_version.label().to_string(),
        }
    }

    pub fn http_version(&self) -> HttpVersion {
        HttpVersion::from_protocol_str(&self.protocol_version)
    }

    pub fn is_http2_or_higher(&self) -> bool {
        self.http_version().is_http2_or_higher()
    }
}
