#![allow(dead_code)]

use crate::browser::automation::models::AutomationResult;

pub struct WaitEngine;

impl WaitEngine {
    pub fn wait(ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: ms,
        }
    }
    pub fn wait_for_selector(_selector: &str, _timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn wait_for_text(_text: &str, _timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn until_visible(_selector: &str, _timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn until_hidden(_selector: &str, _timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn until_enabled(_selector: &str, _timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
}
