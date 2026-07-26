#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ReportEvent {
    Started {
        assessment_id: Uuid,
        report_type: String,
    },
    Generated {
        assessment_id: Uuid,
    },
    Exported {
        assessment_id: Uuid,
        format: String,
    },
    Failed {
        error: String,
    },
}

pub struct ReportEventBus {
    tx: broadcast::Sender<ReportEvent>,
}
impl ReportEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ReportEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ReportEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ReportEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ReportEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
