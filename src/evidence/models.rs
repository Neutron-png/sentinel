#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceType {
    HttpRequest,
    HttpResponse,
    Screenshot,
    DomSnapshot,
    HtmlSource,
    JavaScriptOutput,
    ConsoleLog,
    Cookie,
    LocalStorage,
    SessionStorage,
    WebSocketFrame,
    File,
    TextNote,
    Json,
    BinaryBlob,
}

impl EvidenceType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::HttpRequest => "HTTP Request",
            Self::HttpResponse => "HTTP Response",
            Self::Screenshot => "Screenshot",
            Self::DomSnapshot => "DOM Snapshot",
            Self::HtmlSource => "HTML Source",
            Self::JavaScriptOutput => "JS Output",
            Self::ConsoleLog => "Console Log",
            Self::Cookie => "Cookie",
            Self::LocalStorage => "Local Storage",
            Self::SessionStorage => "Session Storage",
            Self::WebSocketFrame => "WS Frame",
            Self::File => "File",
            Self::TextNote => "Text Note",
            Self::Json => "JSON",
            Self::BinaryBlob => "Binary Blob",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvidenceItem {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub finding_id: Option<Uuid>,
    pub source_module: String,
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub evidence_type: EvidenceType,
    pub mime_type: String,
    pub tags: Vec<String>,
    pub description: String,
    pub content: Vec<u8>,
    pub sha256_hash: String,
    pub linked_transactions: Vec<Uuid>,
    pub linked_findings: Vec<Uuid>,
    pub version: u32,
}

impl EvidenceItem {
    pub fn new(assessment_id: Uuid, evidence_type: EvidenceType) -> Self {
        Self {
            id: Uuid::new_v4(),
            assessment_id,
            finding_id: None,
            source_module: String::new(),
            timestamp: Utc::now(),
            author: String::new(),
            evidence_type,
            mime_type: String::new(),
            tags: vec![],
            description: String::new(),
            content: vec![],
            sha256_hash: String::new(),
            linked_transactions: vec![],
            linked_findings: vec![],
            version: 1,
        }
    }
}
