#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Document,
    Script,
    Stylesheet,
    Image,
    Font,
    Media,
    Manifest,
    Fetch,
    Xhr,
    Beacon,
    Other,
}

impl ResourceType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Document => "Document",
            Self::Script => "Script",
            Self::Stylesheet => "Stylesheet",
            Self::Image => "Image",
            Self::Font => "Font",
            Self::Media => "Media",
            Self::Manifest => "Manifest",
            Self::Fetch => "Fetch",
            Self::Xhr => "XHR",
            Self::Beacon => "Beacon",
            Self::Other => "Other",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrowserNetworkRequest {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: String,
    pub headers: String,
    pub cookies: String,
    pub body: String,
    pub resource_type: ResourceType,
    pub initiator: String,
    pub frame_id: Option<Uuid>,
    pub tab_id: Uuid,
    pub browser_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct BrowserNetworkResponse {
    pub request_id: Uuid,
    pub status_code: u16,
    pub headers: String,
    pub cookies: String,
    pub mime_type: Option<String>,
    pub content_length: Option<u64>,
    pub encoding: Option<String>,
    pub timing_ms: u64,
    pub redirect_chain: Vec<String>,
}
