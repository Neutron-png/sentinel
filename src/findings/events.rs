#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CorrelationEvent {
    Created { id: Uuid, title: String },
    Updated { id: Uuid },
    Merged { id: Uuid, merged_from: Vec<Uuid> },
    Verified { id: Uuid },
    Resolved { id: Uuid },
}

pub struct CorrelationEventBus {
    tx: broadcast::Sender<CorrelationEvent>,
}
impl CorrelationEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<CorrelationEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<CorrelationEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: CorrelationEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for CorrelationEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
