#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AuthEvent {
    RecordingStarted {
        workflow_id: Uuid,
    },
    RecordingStopped {
        workflow_id: Uuid,
        action_count: usize,
    },
    AuthenticationStarted {
        workflow_id: Uuid,
    },
    AuthenticationSucceeded {
        workflow_id: Uuid,
    },
    AuthenticationFailed {
        workflow_id: Uuid,
        error: String,
    },
    AuthenticationExpired {
        workflow_id: Uuid,
    },
    WorkflowRecorded {
        workflow_id: Uuid,
        name: String,
    },
    WorkflowReplayed {
        workflow_id: Uuid,
    },
    SessionExtracted {
        cookie_count: usize,
        storage_entries: usize,
    },
}

pub struct AuthEventBus {
    tx: broadcast::Sender<AuthEvent>,
}
impl AuthEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<AuthEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<AuthEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: AuthEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for AuthEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
