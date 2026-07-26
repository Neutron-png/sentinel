#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BrowserEvent {
    BrowserCreated {
        id: Uuid,
    },
    BrowserClosed {
        id: Uuid,
    },
    TabCreated {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    TabClosed {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    NavigationStarted {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
    },
    NavigationFinished {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
    },
    PageLoaded {
        browser_id: Uuid,
        tab_id: Uuid,
    },
    LoadFailed {
        browser_id: Uuid,
        tab_id: Uuid,
        error: String,
    },
    TitleChanged {
        browser_id: Uuid,
        tab_id: Uuid,
        title: String,
    },
}

pub struct BrowserEventBus {
    tx: broadcast::Sender<BrowserEvent>,
}
impl BrowserEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<BrowserEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<BrowserEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: BrowserEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for BrowserEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
