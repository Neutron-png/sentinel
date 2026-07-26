#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Host {
    pub id: Uuid,
    pub hostname: String,
    pub scheme: String,
    pub port: u16,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub endpoints: Vec<Endpoint>,
}

impl Host {
    pub fn new(hostname: &str, scheme: &str, port: u16) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            hostname: hostname.to_string(),
            scheme: scheme.to_string(),
            port,
            first_seen: now,
            last_seen: now,
            endpoints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: Uuid,
    pub host_id: Uuid,
    pub path: String,
    pub normalized_path: String,
    pub methods: HashSet<String>,
    pub parameters: HashSet<String>,
    pub content_types: HashSet<String>,
    pub status_codes: Vec<u16>,
    pub request_count: u64,
    pub response_count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub transactions: Vec<TransactionReference>,
}

impl Endpoint {
    pub fn new(host_id: Uuid, path: &str, normalized: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            host_id,
            path: path.to_string(),
            normalized_path: normalized.to_string(),
            methods: HashSet::new(),
            parameters: HashSet::new(),
            content_types: HashSet::new(),
            status_codes: Vec::new(),
            request_count: 0,
            response_count: 0,
            first_seen: now,
            last_seen: now,
            transactions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransactionReference {
    pub transaction_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub status_code: u16,
}
