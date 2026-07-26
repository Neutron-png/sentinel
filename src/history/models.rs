#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
}
