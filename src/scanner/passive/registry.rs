#![allow(dead_code)]

use std::sync::Arc;

use crate::scanner::passive::rule::ScanRule;

pub struct RuleRegistry {
    rules: Vec<Arc<dyn ScanRule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Arc<dyn ScanRule>) {
        self.rules.push(rule);
    }

    pub fn unregister(&mut self, rule_id: &str) {
        self.rules.retain(|r| r.id() != rule_id);
    }

    pub fn rules(&self) -> &[Arc<dyn ScanRule>] {
        &self.rules
    }
    pub fn count(&self) -> usize {
        self.rules.len()
    }
    pub fn find(&self, rule_id: &str) -> Option<&Arc<dyn ScanRule>> {
        self.rules.iter().find(|r| r.id() == rule_id)
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
