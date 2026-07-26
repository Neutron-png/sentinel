#![allow(dead_code)]

use std::sync::Arc;
use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::events::ScanEventBus;
use crate::scanner::passive::models::ScanResult;
use crate::scanner::passive::registry::RuleRegistry;
use crate::scanner::passive::rule::ScanRule;
use crate::scanner::passive::runner::RuleRunner;

pub struct PassiveScannerEngine {
    registry: RuleRegistry,
    event_bus: ScanEventBus,
    results: Vec<ScanResult>,
    results_by_transaction: std::collections::HashMap<Uuid, Vec<ScanResult>>,
}

impl PassiveScannerEngine {
    pub fn new() -> Self {
        Self {
            registry: RuleRegistry::new(),
            event_bus: ScanEventBus::new(1024),
            results: Vec::new(),
            results_by_transaction: std::collections::HashMap::new(),
        }
    }

    pub fn register_rule(&mut self, rule: Arc<dyn ScanRule>) {
        self.registry.register(rule);
    }

    pub fn unregister_rule(&mut self, rule_id: &str) {
        self.registry.unregister(rule_id);
    }

    pub fn rule_count(&self) -> usize {
        self.registry.count()
    }
    pub fn event_bus(&self) -> ScanEventBus {
        self.event_bus.clone()
    }

    pub fn scan(
        &mut self,
        transaction_id: Uuid,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<ScanResult> {
        let runner = RuleRunner::new_with_rules(&self.registry, self.event_bus.clone());
        let results = runner.run(transaction_id, request, response);
        if !results.is_empty() {
            self.results.extend(results.clone());
            self.results_by_transaction
                .insert(transaction_id, results.clone());
        }
        results
    }

    pub fn results(&self) -> &[ScanResult] {
        &self.results
    }
    pub fn results_for(&self, transaction_id: Uuid) -> Option<&[ScanResult]> {
        self.results_by_transaction
            .get(&transaction_id)
            .map(|v| v.as_slice())
    }
    pub fn total_results(&self) -> usize {
        self.results.len()
    }
    pub fn clear(&mut self) {
        self.results.clear();
        self.results_by_transaction.clear();
    }
}
