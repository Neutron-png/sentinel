#![allow(dead_code)]

use std::sync::Arc;

use crate::scanner::active::context::ScanContext;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub rule_id: String,
    pub endpoint: String,
    pub finding: String,
    pub evidence: String,
    pub severity: String,
}

pub trait ActiveScanRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, context: &ScanContext, task: &super::models::ScanTask) -> Vec<ScanResult>;
}

pub struct ActiveRuleRegistry {
    rules: Vec<Arc<dyn ActiveScanRule>>,
}

impl ActiveRuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn register(&mut self, rule: Arc<dyn ActiveScanRule>) {
        self.rules.push(rule);
    }
    pub fn get(&self, id: &str) -> Option<&Arc<dyn ActiveScanRule>> {
        self.rules.iter().find(|r| r.id() == id)
    }
    pub fn count(&self) -> usize {
        self.rules.len()
    }
}
impl Default for ActiveRuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
