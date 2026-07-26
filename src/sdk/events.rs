#![allow(dead_code)]

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum SdkEvent {
    Loaded { plugin_id: String },
    Initialized { plugin_id: String },
    Enabled { plugin_id: String },
    Disabled { plugin_id: String },
    Reloaded { plugin_id: String },
    Unloaded { plugin_id: String },
    Failed { plugin_id: String, error: String },
}

pub struct SdkEventBus {
    tx: broadcast::Sender<SdkEvent>,
}
impl SdkEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<SdkEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<SdkEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: SdkEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for SdkEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
