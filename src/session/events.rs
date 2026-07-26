#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum SessionEvent {
    SessionCreated {
        session_id: Uuid,
        assessment_id: Uuid,
    },
    SessionLoaded {
        session_id: Uuid,
    },
    SessionSaved {
        session_id: Uuid,
    },
    SessionExpired {
        session_id: Uuid,
    },
    SessionRefreshed {
        session_id: Uuid,
    },
    SessionDestroyed {
        session_id: Uuid,
    },
    AuthenticationRestored {
        session_id: Uuid,
    },
}

pub struct SessionEventBus {
    tx: broadcast::Sender<SessionEvent>,
}
impl SessionEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<SessionEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: SessionEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for SessionEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
