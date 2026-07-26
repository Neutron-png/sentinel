#![allow(dead_code)]

use tokio::sync::broadcast;
use uuid::Uuid;

use crate::orchestrator::models::AssessmentPhase;

#[derive(Debug, Clone)]
pub enum OrchestratorEvent {
    AssessmentStarted {
        assessment_id: Uuid,
    },
    AssessmentPaused,
    AssessmentResumed,
    AssessmentCancelled,
    AssessmentCompleted {
        assessment_id: Uuid,
    },
    PhaseStarted {
        phase: AssessmentPhase,
    },
    PhaseCompleted {
        phase: AssessmentPhase,
    },
    PhaseFailed {
        phase: AssessmentPhase,
        error: String,
    },
    ProgressUpdated {
        phase: AssessmentPhase,
        progress: f64,
    },
    TaskCompleted {
        module: String,
    },
    TaskFailed {
        module: String,
        error: String,
    },
}

pub struct OrchestratorEventBus {
    tx: broadcast::Sender<OrchestratorEvent>,
}
impl OrchestratorEventBus {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        Self { tx }
    }
    pub fn sender(&self) -> broadcast::Sender<OrchestratorEvent> {
        self.tx.clone()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<OrchestratorEvent> {
        self.tx.subscribe()
    }
    pub fn emit(&self, event: OrchestratorEvent) {
        let _ = self.tx.send(event);
    }
}
impl Clone for OrchestratorEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
