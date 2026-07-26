#![allow(dead_code)]

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum ResourceEvent {
    RateLimited { reason: String },
    ConcurrencyThrottled { current: usize, max: usize },
    CircuitOpened { reason: String },
    CircuitClosed,
    RetryScheduled { attempt: u32, delay_ms: u64 },
}

pub struct ResourceEventBus {
    tx: broadcast::Sender<ResourceEvent>,
}
impl ResourceEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ResourceEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ResourceEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ResourceEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ResourceEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
