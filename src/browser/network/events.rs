#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

use crate::browser::network::models::{
    BrowserNetworkRequest, BrowserNetworkResponse, ResourceType,
};

#[derive(Debug, Clone)]
pub enum ObserverEvent {
    RequestStarted {
        request: BrowserNetworkRequest,
    },
    RequestSent {
        request_id: Uuid,
    },
    ResponseStarted {
        request_id: Uuid,
        status_code: u16,
    },
    ResponseReceived {
        request_id: Uuid,
        response: BrowserNetworkResponse,
    },
    RequestFinished {
        request_id: Uuid,
        timing_ms: u64,
    },
    RequestFailed {
        request_id: Uuid,
        error: String,
    },
    RedirectOccurred {
        request_id: Uuid,
        from_url: String,
        to_url: String,
    },
    ResourceDiscovered {
        request_id: Uuid,
        resource_type: ResourceType,
        url: String,
    },
}

pub struct ObserverEventBus {
    tx: broadcast::Sender<ObserverEvent>,
}
impl ObserverEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ObserverEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ObserverEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ObserverEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ObserverEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
