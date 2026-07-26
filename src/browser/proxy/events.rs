#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BrowserProxyEvent {
    BrowserAttached {
        browser_id: Uuid,
    },
    BrowserDetached {
        browser_id: Uuid,
    },
    ProxyConnected {
        browser_id: Uuid,
        proxy_port: u16,
    },
    ProxyDisconnected {
        browser_id: Uuid,
    },
    BrowserRequestStarted {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
        method: String,
    },
    BrowserRequestFinished {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
        status: u16,
    },
    BrowserNavigationStarted {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
    },
    BrowserNavigationFinished {
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
    },
}

pub struct BrowserProxyEventBus {
    tx: broadcast::Sender<BrowserProxyEvent>,
}
impl BrowserProxyEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<BrowserProxyEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<BrowserProxyEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: BrowserProxyEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for BrowserProxyEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
