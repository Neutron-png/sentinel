#![allow(dead_code)]

use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::events::ScanEventBus;
use crate::scanner::passive::models::ScanResult;
use crate::scanner::passive::registry::RuleRegistry;

pub struct RuleRunner<'a> {
    registry: &'a RuleRegistry,
    event_bus: ScanEventBus,
}

impl<'a> RuleRunner<'a> {
    pub fn new_with_rules(registry: &'a RuleRegistry, event_bus: ScanEventBus) -> Self {
        Self {
            registry,
            event_bus,
        }
    }

    pub fn run(
        &self,
        transaction_id: Uuid,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<ScanResult> {
        self.event_bus
            .emit(super::events::ScanEvent::ScanStarted { transaction_id });
        let mut results = Vec::new();

        for rule in self.registry.rules() {
            if !rule.applies_to(request, response) {
                continue;
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                rule.analyze(transaction_id, request, response)
            })) {
                Ok(rule_results) => {
                    if !rule_results.is_empty() {
                        self.event_bus.emit(super::events::ScanEvent::RuleMatched {
                            rule_id: rule.id().to_string(),
                            transaction_id,
                        });
                        results.extend(rule_results);
                    }
                }
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "unknown error".into()
                    };
                    self.event_bus.emit(super::events::ScanEvent::ScanError {
                        transaction_id,
                        error: format!("Rule '{}' panicked: {msg}", rule.id()),
                    });
                }
            }
        }

        self.event_bus
            .emit(super::events::ScanEvent::ScanCompleted {
                transaction_id,
                result_count: results.len(),
            });
        results
    }

    pub fn event_bus(&self) -> ScanEventBus {
        self.event_bus.clone()
    }
}
