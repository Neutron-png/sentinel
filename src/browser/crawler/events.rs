#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CrawlEvent {
    CrawlStarted {
        browser_id: Uuid,
    },
    PageDiscovered {
        url: String,
        title: String,
    },
    RouteDiscovered {
        url: String,
    },
    EndpointDiscovered {
        url: String,
        method: String,
    },
    FormDiscovered {
        url: String,
        action: String,
        fields: usize,
    },
    CrawlFinished {
        browser_id: Uuid,
        pages: usize,
    },
    CrawlError {
        error: String,
    },
}

pub struct CrawlEventBus {
    tx: broadcast::Sender<CrawlEvent>,
}
impl CrawlEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<CrawlEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<CrawlEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: CrawlEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for CrawlEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
