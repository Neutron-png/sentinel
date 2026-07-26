#![allow(dead_code)]

use crate::auth::models::{AuthWorkflow, RecordedAction};

pub struct AuthRecorder {
    recording: bool,
    workflow: Option<AuthWorkflow>,
}

impl AuthRecorder {
    pub fn new() -> Self {
        Self {
            recording: false,
            workflow: None,
        }
    }

    pub fn start(&mut self, name: &str) -> &AuthWorkflow {
        self.recording = true;
        self.workflow = Some(AuthWorkflow::new(name));
        self.workflow.as_ref().unwrap()
    }

    pub fn stop(&mut self) -> Option<AuthWorkflow> {
        self.recording = false;
        self.workflow.take()
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn record_action(&mut self, action: RecordedAction) {
        if let Some(ref mut wf) = self.workflow {
            wf.actions.push(action);
            wf.updated_at = chrono::Utc::now();
        }
    }

    pub fn record_navigate(&mut self, url: &str) {
        self.record_action(RecordedAction::navigate(url));
    }
    pub fn record_click(&mut self, selector: &str) {
        self.record_action(RecordedAction::click(selector));
    }
    pub fn record_type(&mut self, selector: &str, text: &str) {
        self.record_action(RecordedAction::type_text(selector, text));
    }
    pub fn record_submit(&mut self, selector: &str) {
        self.record_action(RecordedAction::submit(selector));
    }
    pub fn record_wait(&mut self, ms: u64) {
        self.record_action(RecordedAction::wait(ms));
    }
}
