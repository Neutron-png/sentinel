#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum EvidenceEvent {
    Created { id: Uuid, evidence_type: String },
    Linked { evidence_id: Uuid, finding_id: Uuid },
    Exported { evidence_id: Uuid },
    Verified { evidence_id: Uuid, hash_valid: bool },
}

pub struct EvidenceEventBus {
    tx: broadcast::Sender<EvidenceEvent>,
}
impl EvidenceEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<EvidenceEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<EvidenceEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: EvidenceEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for EvidenceEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
