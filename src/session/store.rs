#![allow(dead_code)]

use std::collections::HashMap;
use uuid::Uuid;

use crate::session::models::SessionInfo;

pub struct SessionStore {
    sessions: HashMap<Uuid, SessionInfo>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create(&mut self, session: SessionInfo) {
        self.sessions.insert(session.id, session);
    }

    pub fn get(&self, id: Uuid) -> Option<&SessionInfo> {
        self.sessions.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut SessionInfo> {
        self.sessions.get_mut(&id)
    }

    pub fn get_by_assessment(&self, assessment_id: Uuid) -> Option<&SessionInfo> {
        self.sessions
            .values()
            .find(|s| s.assessment_id == assessment_id)
    }

    pub fn get_mut_by_assessment(&mut self, assessment_id: Uuid) -> Option<&mut SessionInfo> {
        self.sessions
            .values_mut()
            .find(|s| s.assessment_id == assessment_id)
    }

    pub fn remove(&mut self, id: Uuid) -> Option<SessionInfo> {
        self.sessions.remove(&id)
    }

    pub fn all(&self) -> Vec<&SessionInfo> {
        self.sessions.values().collect()
    }

    pub fn update_state(&mut self, id: Uuid, state: super::models::SessionState) -> bool {
        if let Some(s) = self.sessions.get_mut(&id) {
            s.state = state;
            s.updated_at = chrono::Utc::now();
            true
        } else {
            false
        }
    }

    pub fn update_cookies(&mut self, id: Uuid, cookies: Vec<String>) -> bool {
        if let Some(s) = self.sessions.get_mut(&id) {
            s.cookies = cookies;
            s.updated_at = chrono::Utc::now();
            true
        } else {
            false
        }
    }
}
