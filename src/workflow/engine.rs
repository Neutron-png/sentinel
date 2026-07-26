#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::automation::engine::BrowserAutomationEngine;
use crate::workflow::errors::WorkflowError;
use crate::workflow::events::{WorkflowEvent, WorkflowEventBus};
use crate::workflow::executor::WorkflowExecutor;
use crate::workflow::models::{
    ExecutionState, WorkflowDefinition, WorkflowStep, WorkflowType, WorkflowVariable,
};
use crate::workflow::recorder::WorkflowRecorder;

pub struct WorkflowEngine {
    workflows: Vec<WorkflowDefinition>,
    executor: WorkflowExecutor,
    recorder: WorkflowRecorder,
    event_bus: WorkflowEventBus,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            workflows: Vec::new(),
            executor: WorkflowExecutor::new(),
            recorder: WorkflowRecorder::new(),
            event_bus: WorkflowEventBus::new(256),
        }
    }

    pub fn event_bus(&self) -> WorkflowEventBus {
        self.event_bus.clone()
    }

    // ── CRUD ──
    pub fn create_workflow(&mut self, name: &str, wf_type: WorkflowType) -> &WorkflowDefinition {
        let wf = WorkflowDefinition::new(name, wf_type);
        self.workflows.push(wf);
        let w = self.workflows.last().unwrap();
        self.event_bus.emit(WorkflowEvent::Created {
            workflow_id: w.id,
            name: name.to_string(),
        });
        w
    }

    pub fn add_step(&mut self, workflow_id: Uuid, step: WorkflowStep) -> Result<(), WorkflowError> {
        let wf = self
            .workflows
            .iter_mut()
            .find(|w| w.id == workflow_id)
            .ok_or_else(|| WorkflowError::Step("Workflow not found".into()))?;
        wf.steps.push(step);
        wf.updated_at = chrono::Utc::now();
        self.event_bus.emit(WorkflowEvent::Updated { workflow_id });
        Ok(())
    }

    pub fn add_variable(
        &mut self,
        workflow_id: Uuid,
        var: WorkflowVariable,
    ) -> Result<(), WorkflowError> {
        let wf = self
            .workflows
            .iter_mut()
            .find(|w| w.id == workflow_id)
            .ok_or_else(|| WorkflowError::Variable("Workflow not found".into()))?;
        wf.variables.push(var);
        Ok(())
    }

    pub fn delete_workflow(&mut self, workflow_id: Uuid) {
        self.workflows.retain(|w| w.id != workflow_id);
        self.event_bus.emit(WorkflowEvent::Deleted { workflow_id });
    }

    pub fn get_workflow(&self, id: Uuid) -> Option<&WorkflowDefinition> {
        self.workflows.iter().find(|w| w.id == id)
    }

    // ── Recording ──
    pub fn start_recording(&mut self) {
        self.recorder.start();
    }
    pub fn stop_recording(&mut self) -> Vec<WorkflowStep> {
        self.recorder.stop()
    }
    pub fn record_step(&mut self, step: WorkflowStep) {
        self.recorder.record_step(step);
    }

    // ── Execution ──
    pub async fn execute(
        &mut self,
        workflow_id: Uuid,
        engine: &mut BrowserAutomationEngine,
    ) -> Result<Vec<bool>, WorkflowError> {
        let wf = self
            .workflows
            .iter()
            .find(|w| w.id == workflow_id)
            .cloned()
            .ok_or_else(|| WorkflowError::Execution("Workflow not found".into()))?;
        self.event_bus.emit(WorkflowEvent::Started { workflow_id });
        let results = self.executor.execute_workflow(&wf, engine).await;
        match self.executor.state() {
            ExecutionState::Completed => self
                .event_bus
                .emit(WorkflowEvent::Completed { workflow_id }),
            ExecutionState::Failed => self.event_bus.emit(WorkflowEvent::Failed {
                workflow_id,
                error: "Execution failed".into(),
            }),
            _ => {}
        }
        results
    }

    pub fn pause(&mut self) {
        self.executor.pause();
    }
    pub fn resume(&mut self) {
        self.executor.resume();
    }
    pub fn cancel(&mut self) {
        self.executor.cancel();
    }

    pub fn workflows(&self) -> &[WorkflowDefinition] {
        &self.workflows
    }
}
