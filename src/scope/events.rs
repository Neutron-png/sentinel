#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ScopeEvent {
    Loaded {
        assessment_id: Uuid,
        rule_count: usize,
    },
    Updated {
        assessment_id: Uuid,
    },
    Matched {
        url: String,
        verdict: super::models::ScopeVerdict,
    },
    Rejected {
        url: String,
    },
}

pub struct ScopeEventBus {
    tx: broadcast::Sender<ScopeEvent>,
}
impl ScopeEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ScopeEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ScopeEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ScopeEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ScopeEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
