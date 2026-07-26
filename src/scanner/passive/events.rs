#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    ScanStarted {
        transaction_id: Uuid,
    },
    RuleMatched {
        rule_id: String,
        transaction_id: Uuid,
    },
    ScanCompleted {
        transaction_id: Uuid,
        result_count: usize,
    },
    ScanError {
        transaction_id: Uuid,
        error: String,
    },
}

pub struct ScanEventBus {
    tx: broadcast::Sender<ScanEvent>,
}
impl ScanEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ScanEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ScanEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ScanEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ScanEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
