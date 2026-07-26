#![allow(dead_code)]

use crate::auth::models::AuthWorkflow;
use crate::browser::automation::engine::BrowserAutomationEngine;
use crate::browser::automation::models::AutomationAction;

pub struct AuthReplayer;

impl AuthReplayer {
    pub async fn replay(
        workflow: &AuthWorkflow,
        engine: &mut BrowserAutomationEngine,
    ) -> Vec<bool> {
        let mut results = Vec::new();
        engine.start();
        for action in &workflow.actions {
            let autom_action = match action.action_type.as_str() {
                "navigate" => AutomationAction::Navigate {
                    url: action.url.clone().unwrap_or_default(),
                },
                "click" => AutomationAction::Click {
                    selector: action.selector.clone().unwrap_or_default(),
                },
                "type" => AutomationAction::TypeText {
                    selector: action.selector.clone().unwrap_or_default(),
                    text: action.value.clone().unwrap_or_default(),
                },
                "submit" => AutomationAction::SubmitForm {
                    selector: action.selector.clone().unwrap_or_default(),
                },
                "wait" => AutomationAction::Wait {
                    ms: action.delay_ms,
                },
                _ => continue,
            };
            match engine.execute(&autom_action).await {
                Ok(r) => results.push(r.success),
                Err(_) => results.push(false),
            }
        }
        engine.finish();
        results
    }
}
