#![allow(dead_code)]

use crate::browser::automation::models::AutomationResult;

pub struct NavigationEngine;

impl NavigationEngine {
    pub fn go_to(_url: &str) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn reload() -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn back() -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn forward() -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn wait_for_navigation(_timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn wait_for_dom_ready(_timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
    pub fn wait_for_network_idle(_timeout_ms: u64) -> AutomationResult {
        AutomationResult {
            success: true,
            error: None,
            duration_ms: 0,
        }
    }
}
