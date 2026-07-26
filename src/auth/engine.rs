#![allow(dead_code)]

use uuid::Uuid;

use crate::auth::detector::AuthDetector;
use crate::auth::errors::AuthError;
use crate::auth::events::{AuthEvent, AuthEventBus};
use crate::auth::models::{AuthSession, AuthState, AuthWorkflow};
use crate::auth::recorder::AuthRecorder;
use crate::auth::replayer::AuthReplayer;
use crate::auth::session::SessionExtractor;
use crate::browser::automation::engine::BrowserAutomationEngine;

pub struct AuthenticationEngine {
    recorder: AuthRecorder,
    detector: AuthDetector,
    event_bus: AuthEventBus,
    workflows: Vec<AuthWorkflow>,
    sessions: Vec<AuthSession>,
}

impl AuthenticationEngine {
    pub fn new() -> Self {
        Self {
            recorder: AuthRecorder::new(),
            detector: AuthDetector::new(),
            event_bus: AuthEventBus::new(256),
            workflows: Vec::new(),
            sessions: Vec::new(),
        }
    }

    pub fn event_bus(&self) -> AuthEventBus {
        self.event_bus.clone()
    }

    // ── Recording ──
    pub fn start_recording(&mut self, name: &str) -> Uuid {
        let wf = self.recorder.start(name);
        let id = wf.id;
        self.event_bus
            .emit(AuthEvent::RecordingStarted { workflow_id: id });
        id
    }
    pub fn stop_recording(&mut self) -> Option<AuthWorkflow> {
        let wf = self.recorder.stop();
        if let Some(ref w) = wf {
            self.event_bus.emit(AuthEvent::RecordingStopped {
                workflow_id: w.id,
                action_count: w.actions.len(),
            });
            self.event_bus.emit(AuthEvent::WorkflowRecorded {
                workflow_id: w.id,
                name: w.name.clone(),
            });
            self.workflows.push(w.clone());
        }
        wf
    }
    pub fn record_navigate(&mut self, url: &str) {
        self.recorder.record_navigate(url);
    }
    pub fn record_click(&mut self, selector: &str) {
        self.recorder.record_click(selector);
    }
    pub fn record_type(&mut self, selector: &str, text: &str) {
        self.recorder.record_type(selector, text);
    }

    // ── Replay ──
    pub async fn replay(
        &mut self,
        workflow_id: Uuid,
        engine: &mut BrowserAutomationEngine,
    ) -> Result<bool, AuthError> {
        let wf = self
            .workflows
            .iter()
            .find(|w| w.id == workflow_id)
            .cloned()
            .ok_or_else(|| AuthError::Replay("Workflow not found".into()))?;
        self.event_bus
            .emit(AuthEvent::AuthenticationStarted { workflow_id });
        let results = AuthReplayer::replay(&wf, engine).await;
        let success = results.iter().all(|r| *r);
        if success {
            self.event_bus
                .emit(AuthEvent::AuthenticationSucceeded { workflow_id });
        } else {
            self.event_bus.emit(AuthEvent::AuthenticationFailed {
                workflow_id,
                error: "Replay failed".into(),
            });
        }
        self.event_bus
            .emit(AuthEvent::WorkflowReplayed { workflow_id });
        Ok(success)
    }

    // ── Detection ──
    pub fn detect_state(&self, url: &str, has_cookie: bool, has_login: bool) -> AuthState {
        self.detector.detect_state(url, has_cookie, has_login)
    }
    pub fn is_authenticated(&self, url: &str, has_cookie: bool) -> bool {
        self.detector.is_authenticated(url, has_cookie)
    }
    pub fn refresh_auth(&self, _url: &str) -> AuthState {
        AuthState::Unknown
    }

    // ── Session ──
    pub fn extract_session(&mut self, cookies: &str, local: &str, session: &str) {
        let s = SessionExtractor::build_session(cookies, local, session);
        self.event_bus.emit(AuthEvent::SessionExtracted {
            cookie_count: s.cookies.len(),
            storage_entries: s.local_storage.len() + s.session_storage.len(),
        });
        self.sessions.push(s);
    }

    pub fn workflows(&self) -> &[AuthWorkflow] {
        &self.workflows
    }
}
