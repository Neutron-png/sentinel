#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub actions: Vec<RecordedAction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AuthWorkflow {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            actions: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedAction {
    pub action_type: String,
    pub selector: Option<String>,
    pub value: Option<String>,
    pub url: Option<String>,
    pub delay_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl RecordedAction {
    pub fn navigate(url: &str) -> Self {
        Self {
            action_type: "navigate".into(),
            selector: None,
            value: None,
            url: Some(url.to_string()),
            delay_ms: 0,
            timestamp: Utc::now(),
        }
    }
    pub fn click(selector: &str) -> Self {
        Self {
            action_type: "click".into(),
            selector: Some(selector.to_string()),
            value: None,
            url: None,
            delay_ms: 0,
            timestamp: Utc::now(),
        }
    }
    pub fn type_text(selector: &str, text: &str) -> Self {
        Self {
            action_type: "type".into(),
            selector: Some(selector.to_string()),
            value: Some(text.to_string()),
            url: None,
            delay_ms: 0,
            timestamp: Utc::now(),
        }
    }
    pub fn submit(selector: &str) -> Self {
        Self {
            action_type: "submit".into(),
            selector: Some(selector.to_string()),
            value: None,
            url: None,
            delay_ms: 0,
            timestamp: Utc::now(),
        }
    }
    pub fn wait(ms: u64) -> Self {
        Self {
            action_type: "wait".into(),
            selector: None,
            value: None,
            url: None,
            delay_ms: ms,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthState {
    Unknown,
    Authenticated,
    Expired,
    LoggedOut,
    Failed,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub cookies: Vec<String>,
    pub local_storage: Vec<(String, String)>,
    pub session_storage: Vec<(String, String)>,
    pub extracted_at: DateTime<Utc>,
}
