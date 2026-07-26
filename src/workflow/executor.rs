#![allow(dead_code)]

use crate::browser::automation::engine::BrowserAutomationEngine;
use crate::browser::automation::models::AutomationAction;
use crate::workflow::errors::WorkflowError;
use crate::workflow::models::{ExecutionState, WorkflowDefinition, WorkflowStep};
use crate::workflow::variables::VariableStore;

pub struct WorkflowExecutor {
    state: ExecutionState,
    current_step: usize,
    variables: VariableStore,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        Self {
            state: ExecutionState::Idle,
            current_step: 0,
            variables: VariableStore::new(),
        }
    }

    pub fn start(&mut self) {
        self.state = ExecutionState::Running;
        self.current_step = 0;
    }
    pub fn pause(&mut self) {
        self.state = ExecutionState::Paused;
    }
    pub fn resume(&mut self) {
        self.state = ExecutionState::Running;
    }
    pub fn cancel(&mut self) {
        self.state = ExecutionState::Cancelled;
    }

    pub fn state(&self) -> ExecutionState {
        self.state
    }

    pub async fn execute_step(
        &mut self,
        step: &WorkflowStep,
        engine: &mut BrowserAutomationEngine,
    ) -> Result<bool, WorkflowError> {
        let action = match step.action.as_str() {
            "navigate" => AutomationAction::Navigate {
                url: step.url.clone().unwrap_or_default(),
            },
            "click" => AutomationAction::Click {
                selector: step.selector.clone().unwrap_or_default(),
            },
            "double_click" => AutomationAction::DoubleClick {
                selector: step.selector.clone().unwrap_or_default(),
            },
            "hover" => AutomationAction::Hover {
                selector: step.selector.clone().unwrap_or_default(),
            },
            "type_text" => AutomationAction::TypeText {
                selector: step.selector.clone().unwrap_or_default(),
                text: self
                    .variables
                    .replace_in(&step.value.clone().unwrap_or_default()),
            },
            "select" => AutomationAction::SelectOption {
                selector: step.selector.clone().unwrap_or_default(),
                value: step.value.clone().unwrap_or_default(),
            },
            "wait" => AutomationAction::Wait { ms: step.delay_ms },
            "submit" => AutomationAction::SubmitForm {
                selector: step.selector.clone().unwrap_or_default(),
            },
            "set_variable" => {
                self.variables.set(
                    &step.selector.clone().unwrap_or_default(),
                    &step.value.clone().unwrap_or_default(),
                );
                return Ok(true);
            }
            _ => {
                return Err(WorkflowError::Step(format!(
                    "Unknown action: {}",
                    step.action
                )))
            }
        };
        match engine.execute(&action).await {
            Ok(r) => Ok(r.success),
            Err(_) => Ok(false),
        }
    }

    pub async fn execute_workflow(
        &mut self,
        wf: &WorkflowDefinition,
        engine: &mut BrowserAutomationEngine,
    ) -> Result<Vec<bool>, WorkflowError> {
        self.start();
        let mut results = Vec::new();
        for step in &wf.steps {
            if self.state == ExecutionState::Paused {
                break;
            }
            if self.state == ExecutionState::Cancelled {
                break;
            }
            match self.execute_step(step, engine).await {
                Ok(r) => results.push(r),
                Err(_) => results.push(false),
            }
            self.current_step += 1;
        }
        self.state = ExecutionState::Completed;
        Ok(results)
    }

    pub fn variables(&self) -> &VariableStore {
        &self.variables
    }
}
