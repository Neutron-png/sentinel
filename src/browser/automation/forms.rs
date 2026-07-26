#![allow(dead_code)]

use crate::browser::automation::models::AutomationResult;

pub struct FormEngine;

impl FormEngine {
    pub fn fill_form(_form_selector: &str, _fields: &[(String, String)]) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn submit_form(_selector: &str) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn reset_form(_selector: &str) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn extract_validation_errors(_selector: &str) -> Vec<String> {
        vec![]
    }
}
