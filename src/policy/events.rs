#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum PolicyEvent {
    Loaded { policy_id: Uuid, name: String },
    Applied { policy_id: Uuid },
    Updated { policy_id: Uuid },
    Exported { policy_id: Uuid },
}

pub struct PolicyEventBus {
    tx: broadcast::Sender<PolicyEvent>,
}
impl PolicyEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<PolicyEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<PolicyEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: PolicyEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for PolicyEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
