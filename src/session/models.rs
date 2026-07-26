#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Unknown,
    Authenticating,
    Authenticated,
    Expired,
    LoggedOut,
    Failed,
}

impl SessionState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Authenticating => "Authenticating",
            Self::Authenticated => "Authenticated",
            Self::Expired => "Expired",
            Self::LoggedOut => "Logged Out",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub browser_session_id: Option<Uuid>,
    pub auth_workflow_id: Option<Uuid>,
    pub cookies: Vec<String>,
    pub local_storage: Vec<(String, String)>,
    pub session_storage: Vec<(String, String)>,
    pub state: SessionState,
    pub last_activity: DateTime<Utc>,
    pub expiration_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SessionInfo {
    pub fn new(assessment_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            assessment_id,
            browser_session_id: None,
            auth_workflow_id: None,
            cookies: vec![],
            local_storage: vec![],
            session_storage: vec![],
            state: SessionState::Unknown,
            last_activity: now,
            expiration_time: None,
            created_at: now,
            updated_at: now,
        }
    }
}
