#![allow(dead_code)]

use uuid::Uuid;

use crate::orchestrator::errors::OrchestratorError;
use crate::orchestrator::events::{OrchestratorEvent, OrchestratorEventBus};
use crate::orchestrator::models::{AssessmentPhase, ExecutionMode, OrchestratorState};
use crate::orchestrator::pipeline::PhasePipeline;
use crate::orchestrator::scheduler::TaskScheduler;

pub struct OrchestratorEngine {
    mode: ExecutionMode,
    state: OrchestratorState,
    current_phase: Option<AssessmentPhase>,
    scheduler: TaskScheduler,
    event_bus: OrchestratorEventBus,
    assessment_id: Option<Uuid>,
}

impl OrchestratorEngine {
    pub fn new() -> Self {
        Self {
            mode: ExecutionMode::FullAssessment,
            state: OrchestratorState::Idle,
            current_phase: None,
            scheduler: TaskScheduler::new(),
            event_bus: OrchestratorEventBus::new(512),
            assessment_id: None,
        }
    }

    pub fn event_bus(&self) -> OrchestratorEventBus {
        self.event_bus.clone()
    }

    pub fn start(&mut self, assessment_id: Uuid, mode: ExecutionMode) {
        self.assessment_id = Some(assessment_id);
        self.mode = mode;
        self.state = OrchestratorState::Running;
        self.event_bus
            .emit(OrchestratorEvent::AssessmentStarted { assessment_id });
    }

    pub fn pause(&mut self) {
        self.state = OrchestratorState::Paused;
        self.event_bus.emit(OrchestratorEvent::AssessmentPaused);
    }

    pub fn resume(&mut self) {
        self.state = OrchestratorState::Running;
        self.event_bus.emit(OrchestratorEvent::AssessmentResumed);
    }

    pub fn cancel(&mut self) {
        self.state = OrchestratorState::Cancelled;
        self.event_bus.emit(OrchestratorEvent::AssessmentCancelled);
    }

    pub fn complete(&mut self) {
        self.state = OrchestratorState::Completed;
        if let Some(id) = self.assessment_id {
            self.event_bus
                .emit(OrchestratorEvent::AssessmentCompleted { assessment_id: id });
        }
    }

    pub fn state(&self) -> OrchestratorState {
        self.state
    }

    pub fn run_phase(&mut self, phase: AssessmentPhase) -> Result<(), OrchestratorError> {
        if self.state != OrchestratorState::Running {
            return Err(OrchestratorError::Abort("Not running".into()));
        }
        self.current_phase = Some(phase);
        let task = self.scheduler.schedule_phase(phase);
        self.event_bus
            .emit(OrchestratorEvent::PhaseStarted { phase });
        self.scheduler.start_task(task.id);
        self.event_bus.emit(OrchestratorEvent::ProgressUpdated {
            phase,
            progress: 0.0,
        });
        Ok(())
    }

    pub fn complete_phase(&mut self, phase: AssessmentPhase) {
        let task_id = self
            .scheduler
            .tasks
            .iter()
            .find(|t| t.phase == phase)
            .map(|t| t.id);
        if let Some(id) = task_id {
            self.scheduler.complete_task(id);
        }
        self.event_bus
            .emit(OrchestratorEvent::PhaseCompleted { phase });
        self.event_bus.emit(OrchestratorEvent::ProgressUpdated {
            phase,
            progress: self.scheduler.overall_progress(),
        });
    }

    pub fn fail_phase(&mut self, phase: AssessmentPhase, error: &str) {
        self.event_bus.emit(OrchestratorEvent::PhaseFailed {
            phase,
            error: error.to_string(),
        });
    }

    pub async fn run_full_assessment(
        &mut self,
        assessment_id: Uuid,
        mode: ExecutionMode,
    ) -> Result<(), OrchestratorError> {
        self.start(assessment_id, mode);
        let phases = PhasePipeline::phases_for(mode);
        for phase in phases {
            if self.state == OrchestratorState::Cancelled {
                break;
            }
            self.run_phase(phase)?;
            self.complete_phase(phase);
        }
        self.complete();
        Ok(())
    }

    pub fn progress(&self) -> f64 {
        self.scheduler.overall_progress()
    }
    pub fn current_phase(&self) -> Option<AssessmentPhase> {
        self.current_phase
    }
}
