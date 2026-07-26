#![allow(dead_code)]

use std::time::Instant;

use crate::browser::automation::actions::ActionEngine;
use crate::browser::automation::errors::AutomationError;
use crate::browser::automation::events::{AutomationEvent, AutomationEventBus};
use crate::browser::automation::forms::FormEngine;
use crate::browser::automation::models::{AutomationAction, AutomationResult};
use crate::browser::automation::navigation::NavigationEngine;
use crate::browser::automation::screenshots::ScreenshotEngine;
use crate::browser::automation::wait::WaitEngine;

pub struct BrowserAutomationEngine {
    event_bus: AutomationEventBus,
    action_count: usize,
}

impl BrowserAutomationEngine {
    pub fn new() -> Self {
        Self {
            event_bus: AutomationEventBus::new(256),
            action_count: 0,
        }
    }

    pub fn event_bus(&self) -> AutomationEventBus {
        self.event_bus.clone()
    }

    pub async fn execute(
        &mut self,
        action: &AutomationAction,
    ) -> Result<AutomationResult, AutomationError> {
        let started = Instant::now();
        self.event_bus.emit(AutomationEvent::ActionStarted {
            action: format!("{:?}", action),
        });
        self.action_count += 1;

        let result = match action {
            AutomationAction::Navigate { url } => NavigationEngine::go_to(url),
            AutomationAction::Click { selector } => ActionEngine::click(selector),
            AutomationAction::DoubleClick { selector } => ActionEngine::double_click(selector),
            AutomationAction::RightClick { selector } => ActionEngine::double_click(selector),
            AutomationAction::Hover { selector } => ActionEngine::hover(selector),
            AutomationAction::Focus { selector } => ActionEngine::focus(selector),
            AutomationAction::Blur { selector } => ActionEngine::blur(selector),
            AutomationAction::Scroll { x, y } => ActionEngine::scroll(*x, *y),
            AutomationAction::TypeText { selector, text } => {
                ActionEngine::type_text(selector, text)
            }
            AutomationAction::Clear { selector } => ActionEngine::clear(selector),
            AutomationAction::AppendText { selector, text } => {
                ActionEngine::type_text(selector, text)
            }
            AutomationAction::UploadFile { selector, path } => {
                ActionEngine::upload_file(selector, path)
            }
            AutomationAction::SelectOption { selector, value } => {
                ActionEngine::select_option(selector, value)
            }
            AutomationAction::Check { selector } => ActionEngine::check(selector),
            AutomationAction::Uncheck { selector } => ActionEngine::uncheck(selector),
            AutomationAction::Wait { ms } => WaitEngine::wait(*ms),
            AutomationAction::WaitForSelector {
                selector,
                timeout_ms,
            } => WaitEngine::wait_for_selector(selector, *timeout_ms),
            AutomationAction::WaitForText { text, timeout_ms } => {
                WaitEngine::wait_for_text(text, *timeout_ms)
            }
            AutomationAction::WaitUntilVisible {
                selector,
                timeout_ms,
            } => WaitEngine::until_visible(selector, *timeout_ms),
            AutomationAction::WaitUntilHidden {
                selector,
                timeout_ms,
            } => WaitEngine::until_hidden(selector, *timeout_ms),
            AutomationAction::FillForm {
                form_selector,
                fields,
            } => FormEngine::fill_form(form_selector, fields),
            AutomationAction::SubmitForm { selector } => FormEngine::submit_form(selector),
            AutomationAction::GoBack => NavigationEngine::back(),
            AutomationAction::GoForward => NavigationEngine::forward(),
            AutomationAction::Reload => NavigationEngine::reload(),
            AutomationAction::Screenshot { .. } => {
                let _ = ScreenshotEngine::full_page();
                AutomationResult {
                    success: true,
                    error: None,
                    duration_ms: 0,
                }
            }
        };

        let elapsed = started.elapsed();
        if result.success {
            self.event_bus.emit(AutomationEvent::ActionFinished {
                action: format!("{:?}", action),
                duration: elapsed,
            });
        } else if let Some(ref err) = result.error {
            self.event_bus.emit(AutomationEvent::ActionFailed {
                action: format!("{:?}", action),
                error: err.clone(),
            });
        }
        Ok(result)
    }

    pub fn start(&self) {
        self.event_bus.emit(AutomationEvent::Started);
    }
    pub fn finish(&self) {
        self.event_bus.emit(AutomationEvent::Finished {
            action_count: self.action_count,
        });
    }
}
