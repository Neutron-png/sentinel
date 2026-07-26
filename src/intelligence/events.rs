#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum IntelligenceEvent {
    FindingEnriched { finding_id: Uuid, cwe: String },
    ReferenceResolved { from: String, to: String },
    RemediationGenerated { finding_id: Uuid },
}

pub struct IntelligenceEventBus {
    tx: broadcast::Sender<IntelligenceEvent>,
}
impl IntelligenceEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<IntelligenceEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<IntelligenceEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: IntelligenceEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for IntelligenceEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
