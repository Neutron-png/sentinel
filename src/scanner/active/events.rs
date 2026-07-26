#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    JobStarted { job_id: Uuid },
    JobProgress { job_id: Uuid, progress: f64 },
    JobPaused { job_id: Uuid },
    JobResumed { job_id: Uuid },
    JobCompleted { job_id: Uuid },
    JobCancelled { job_id: Uuid },
    JobFailed { job_id: Uuid, error: String },
    TaskStarted { task_id: Uuid, job_id: Uuid },
    TaskCompleted { task_id: Uuid, job_id: Uuid },
    TaskFailed { task_id: Uuid, error: String },
}

pub struct ScanEventBus {
    tx: broadcast::Sender<ScanEvent>,
}
impl ScanEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<ScanEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<ScanEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: ScanEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for ScanEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
