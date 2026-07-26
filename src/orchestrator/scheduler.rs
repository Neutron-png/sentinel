#![allow(dead_code)]

use uuid::Uuid;

use crate::orchestrator::models::{AssessmentPhase, OrchestratorState, OrchestratorTask};

pub struct TaskScheduler {
    pub tasks: Vec<OrchestratorTask>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn schedule_phase(&mut self, phase: AssessmentPhase) -> OrchestratorTask {
        let task = OrchestratorTask {
            id: Uuid::new_v4(),
            phase,
            name: phase.label().to_string(),
            state: OrchestratorState::Idle,
            progress: 0.0,
            started_at: None,
            finished_at: None,
        };
        self.tasks.push(task);
        self.tasks.last().unwrap().clone()
    }

    pub fn start_task(&mut self, id: Uuid) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.state = OrchestratorState::Running;
            t.started_at = Some(chrono::Utc::now());
        }
    }

    pub fn complete_task(&mut self, id: Uuid) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.state = OrchestratorState::Completed;
            t.progress = 1.0;
            t.finished_at = Some(chrono::Utc::now());
        }
    }

    pub fn fail_task(&mut self, id: Uuid) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.state = OrchestratorState::Failed;
            t.finished_at = Some(chrono::Utc::now());
        }
    }

    pub fn pending_tasks(&self) -> Vec<&OrchestratorTask> {
        self.tasks
            .iter()
            .filter(|t| t.state == OrchestratorState::Idle)
            .collect()
    }

    pub fn active_tasks(&self) -> Vec<&OrchestratorTask> {
        self.tasks
            .iter()
            .filter(|t| t.state == OrchestratorState::Running)
            .collect()
    }

    pub fn completed_tasks(&self) -> Vec<&OrchestratorTask> {
        self.tasks
            .iter()
            .filter(|t| t.state == OrchestratorState::Completed)
            .collect()
    }

    pub fn failed_tasks(&self) -> Vec<&OrchestratorTask> {
        self.tasks
            .iter()
            .filter(|t| t.state == OrchestratorState::Failed)
            .collect()
    }

    pub fn overall_progress(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let completed = self
            .tasks
            .iter()
            .filter(|t| t.state == OrchestratorState::Completed)
            .count() as f64;
        completed / self.tasks.len() as f64
    }
}
