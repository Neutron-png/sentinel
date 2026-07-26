#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum InterceptEvent {
    RequestIntercepted {
        id: Uuid,
        url: String,
        method: String,
    },
    ResponseIntercepted {
        id: Uuid,
        url: String,
        status: u16,
    },
    RequestModified {
        id: Uuid,
    },
    ResponseModified {
        id: Uuid,
    },
    RequestForwarded {
        id: Uuid,
    },
    ResponseForwarded {
        id: Uuid,
    },
    RequestDropped {
        id: Uuid,
    },
    ResponseDropped {
        id: Uuid,
    },
}

pub struct InterceptEventBus {
    tx: broadcast::Sender<InterceptEvent>,
}

impl InterceptEventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<InterceptEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<InterceptEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: InterceptEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for InterceptEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
