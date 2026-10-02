#![allow(dead_code)]

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum ProxyEvent {
    ClientConnected {
        client_addr: String,
    },
    ClientDisconnected {
        client_addr: String,
    },
    RequestReceived {
        method: String,
        url: String,
        host: String,
    },
    ResponseReceived {
        status: u16,
        url: String,
    },
    TransactionCaptured {
        method: String,
        url: String,
        host: String,
        port: u16,
        status: u16,
        protocol: String,
        tls_enabled: bool,
        request_size: u64,
        response_size: u64,
        duration_ms: u64,
        request_body: String,
        response_body: String,
        via_upstream: String,
    },
    TlsEstablished {
        host: String,
    },
    ConnectionClosed {
        reason: String,
    },
    Error {
        message: String,
    },
}

pub struct EventBus {
    tx: broadcast::Sender<ProxyEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn sender(&self) -> broadcast::Sender<ProxyEvent> {
        self.tx.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ProxyEvent> {
        self.tx.subscribe()
    }

    pub fn emit(&self, event: ProxyEvent) {
        let _ = self.tx.send(event);
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
