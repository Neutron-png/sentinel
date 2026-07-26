#![allow(dead_code)]

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum HarEvent {
    RecordingStarted,
    RecordingStopped { entries: usize },
    EntryRecorded { url: String, method: String },
    ExportStarted,
    ExportFinished { path: String },
    ImportStarted { path: String },
    ImportFinished { entries: usize },
}

pub struct HarEventBus {
    tx: broadcast::Sender<HarEvent>,
}
impl HarEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<HarEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<HarEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: HarEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for HarEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
