#![allow(dead_code)]

use uuid::Uuid;

use crate::auth::engine::AuthenticationEngine;
use crate::browser::automation::engine::BrowserAutomationEngine;
use crate::session::detector::ExpirationDetector;
use crate::session::errors::SessionError;
use crate::session::events::{SessionEvent, SessionEventBus};
use crate::session::models::{SessionInfo, SessionState};
use crate::session::refresh::SessionRefresher;
use crate::session::store::SessionStore;

pub struct SessionManager {
    store: SessionStore,
    detector: ExpirationDetector,
    event_bus: SessionEventBus,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            store: SessionStore::new(),
            detector: ExpirationDetector::new(),
            event_bus: SessionEventBus::new(256),
        }
    }

    pub fn event_bus(&self) -> SessionEventBus {
        self.event_bus.clone()
    }

    // ── Session lifecycle ──

    pub fn create_session(&mut self, assessment_id: Uuid) -> &SessionInfo {
        let session = SessionInfo::new(assessment_id);
        self.store.create(session);
        let s = self.store.get_by_assessment(assessment_id).unwrap();
        self.event_bus.emit(SessionEvent::SessionCreated {
            session_id: s.id,
            assessment_id,
        });
        s
    }

    pub fn get_session(&self, id: Uuid) -> Option<&SessionInfo> {
        self.store.get(id)
    }
    pub fn get_by_assessment(&self, assessment_id: Uuid) -> Option<&SessionInfo> {
        self.store.get_by_assessment(assessment_id)
    }

    pub fn destroy_session(&mut self, id: Uuid) -> Result<(), SessionError> {
        self.store
            .remove(id)
            .ok_or_else(|| SessionError::NotFound(id.to_string()))?;
        self.event_bus
            .emit(SessionEvent::SessionDestroyed { session_id: id });
        Ok(())
    }

    // ── Authentication state ──

    pub fn update_state(&mut self, id: Uuid, state: SessionState) {
        if self.store.update_state(id, state) {
            match state {
                SessionState::Expired => self
                    .event_bus
                    .emit(SessionEvent::SessionExpired { session_id: id }),
                SessionState::Authenticated => self
                    .event_bus
                    .emit(SessionEvent::SessionRefreshed { session_id: id }),
                _ => {}
            }
        }
    }

    pub fn detect_expiration(
        &self,
        id: Uuid,
        status_code: u16,
        body: &str,
        current_url: &str,
        has_cookie: bool,
    ) -> SessionState {
        let state = self
            .detector
            .detect_expiration(status_code, body, current_url, has_cookie);
        if state == SessionState::Expired && self.store.get(id).is_some() {
            let _ = state;
        }
        state
    }

    // ── Refresh ──

    pub async fn refresh_session(
        &mut self,
        session_id: Uuid,
        auth_engine: &mut AuthenticationEngine,
        automation_engine: &mut BrowserAutomationEngine,
    ) -> Result<bool, SessionError> {
        let session = self
            .store
            .get(session_id)
            .ok_or_else(|| SessionError::NotFound(session_id.to_string()))?;
        let workflow_id = session
            .auth_workflow_id
            .ok_or_else(|| SessionError::Refresh("No workflow".into()))?;
        let result = SessionRefresher::refresh(auth_engine, automation_engine, workflow_id).await?;
        if result {
            self.store
                .update_state(session_id, SessionState::Authenticated);
            self.event_bus
                .emit(SessionEvent::AuthenticationRestored { session_id });
        }
        Ok(result)
    }

    // ── Cookies ──

    pub fn update_cookies(&mut self, id: Uuid, cookies: Vec<String>) -> Result<(), SessionError> {
        if self.store.update_cookies(id, cookies) {
            Ok(())
        } else {
            Err(SessionError::NotFound(id.to_string()))
        }
    }

    pub fn sessions(&self) -> Vec<&SessionInfo> {
        self.store.all()
    }
}
