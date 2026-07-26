#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum WorkflowEvent {
    Created { workflow_id: Uuid, name: String },
    Updated { workflow_id: Uuid },
    Deleted { workflow_id: Uuid },
    Started { workflow_id: Uuid },
    Paused { workflow_id: Uuid },
    Resumed { workflow_id: Uuid },
    Completed { workflow_id: Uuid },
    Failed { workflow_id: Uuid, error: String },
    StepCompleted { workflow_id: Uuid, step_id: Uuid },
}

pub struct WorkflowEventBus {
    tx: broadcast::Sender<WorkflowEvent>,
}
impl WorkflowEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<WorkflowEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<WorkflowEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: WorkflowEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for WorkflowEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
