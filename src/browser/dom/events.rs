#![allow(dead_code)]

use serde_json::Value;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum DomEvent {
    DomReady {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    DomChanged {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    FormDiscovered {
        browser_id: Uuid,
        tab_id: Uuid,
        form: super::models::FormInfo,
    },
    ResourceDiscovered {
        browser_id: Uuid,
        tab_id: Uuid,
        resource: super::models::ResourceInfo,
    },
    ScriptExecuted {
        browser_id: Uuid,
        tab_id: Uuid,
        result: Value,
    },
    NavigationCompleted {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    StorageChanged {
        browser_id: Uuid,
        tab_id: Uuid,
        storage_type: String,
    },
}

pub struct DomEventBus {
    tx: broadcast::Sender<DomEvent>,
}
impl DomEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<DomEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<DomEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: DomEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for DomEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
