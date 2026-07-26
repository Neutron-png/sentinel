#![allow(dead_code)]

use std::time::Duration;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum AutomationEvent {
    Started,
    Finished { action_count: usize },
    ActionStarted { action: String },
    ActionFinished { action: String, duration: Duration },
    ActionFailed { action: String, error: String },
}

pub struct AutomationEventBus {
    tx: broadcast::Sender<AutomationEvent>,
}
impl AutomationEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<AutomationEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<AutomationEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: AutomationEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for AutomationEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
