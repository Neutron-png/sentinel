#![allow(dead_code)]

use crate::workflow::models::WorkflowStep;
use crate::workflow::variables::VariableStore;

pub struct WorkflowRecorder {
    recording: bool,
    steps: Vec<WorkflowStep>,
    variables: VariableStore,
}

impl WorkflowRecorder {
    pub fn new() -> Self {
        Self {
            recording: false,
            steps: vec![],
            variables: VariableStore::new(),
        }
    }

    pub fn start(&mut self) {
        self.recording = true;
        self.steps.clear();
    }
    pub fn stop(&mut self) -> Vec<WorkflowStep> {
        self.recording = false;
        self.steps.clone()
    }

    pub fn record_step(&mut self, step: WorkflowStep) {
        if self.recording {
            self.steps.push(step);
        }
    }
    pub fn set_variable(&mut self, name: &str, value: &str) {
        self.variables.set(name, value);
    }
    pub fn variables(&self) -> &VariableStore {
        &self.variables
    }
}
