#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum WsEvent {
    ConnectionOpened {
        connection_id: Uuid,
        url: String,
    },
    ConnectionClosed {
        connection_id: Uuid,
    },
    FrameReceived {
        connection_id: Uuid,
        frame_id: Uuid,
        opcode: String,
        direction: super::models::WsDirection,
    },
    FrameSent {
        connection_id: Uuid,
        frame_id: Uuid,
    },
    FrameModified {
        connection_id: Uuid,
        frame_id: Uuid,
    },
    FrameDropped {
        connection_id: Uuid,
        frame_id: Uuid,
    },
    FrameReplayed {
        connection_id: Uuid,
        frame_id: Uuid,
    },
}

pub struct WsEventBus {
    tx: broadcast::Sender<WsEvent>,
}
impl WsEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<WsEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<WsEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: WsEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for WsEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
