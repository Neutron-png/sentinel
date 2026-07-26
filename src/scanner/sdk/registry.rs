#![allow(dead_code)]

use std::sync::Arc;

use crate::scanner::sdk::errors::SdkError;
use crate::scanner::sdk::metadata::RuleMetadata;
use crate::scanner::sdk::rule::ScanRule;

pub struct ScanRuleRegistry {
    rules: Vec<ScanRuleEntry>,
}

struct ScanRuleEntry {
    rule: Arc<dyn ScanRule>,
    enabled: bool,
}

impl ScanRuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Arc<dyn ScanRule>) -> Result<(), SdkError> {
        if self
            .rules
            .iter()
            .any(|e| e.rule.metadata().id == rule.metadata().id)
        {
            return Err(SdkError::Registry(format!(
                "Rule '{}' already registered",
                rule.metadata().id
            )));
        }
        self.rules.push(ScanRuleEntry {
            rule,
            enabled: true,
        });
        Ok(())
    }

    pub fn unregister(&mut self, rule_id: &str) -> Option<Arc<dyn ScanRule>> {
        if let Some(pos) = self
            .rules
            .iter()
            .position(|e| e.rule.metadata().id == rule_id)
        {
            let entry = self.rules.remove(pos);
            Some(entry.rule)
        } else {
            None
        }
    }

    pub fn enable(&mut self, rule_id: &str) -> Result<(), SdkError> {
        self.rules
            .iter_mut()
            .find(|e| e.rule.metadata().id == rule_id)
            .map(|e| e.enabled = true)
            .ok_or_else(|| SdkError::Registry(format!("Rule '{rule_id}' not found")))
    }

    pub fn disable(&mut self, rule_id: &str) -> Result<(), SdkError> {
        self.rules
            .iter_mut()
            .find(|e| e.rule.metadata().id == rule_id)
            .map(|e| e.enabled = false)
            .ok_or_else(|| SdkError::Registry(format!("Rule '{rule_id}' not found")))
    }

    pub fn find(&self, rule_id: &str) -> Option<&Arc<dyn ScanRule>> {
        self.rules
            .iter()
            .find(|e| e.rule.metadata().id == rule_id)
            .map(|e| &e.rule)
    }

    pub fn list(&self) -> Vec<&RuleMetadata> {
        self.rules.iter().map(|e| e.rule.metadata()).collect()
    }

    pub fn enabled_rules(&self) -> Vec<&Arc<dyn ScanRule>> {
        self.rules
            .iter()
            .filter(|e| e.enabled)
            .map(|e| &e.rule)
            .collect()
    }

    pub fn all_rules(&self) -> Vec<&Arc<dyn ScanRule>> {
        self.rules.iter().map(|e| &e.rule).collect()
    }

    pub fn count(&self) -> usize {
        self.rules.len()
    }
    pub fn enabled_count(&self) -> usize {
        self.rules.iter().filter(|e| e.enabled).count()
    }

    pub fn find_by_category(&self, category: &str) -> Vec<&Arc<dyn ScanRule>> {
        self.rules
            .iter()
            .filter(|e| e.rule.metadata().category == category && e.enabled)
            .map(|e| &e.rule)
            .collect()
    }

    pub fn find_by_tag(&self, tag: &str) -> Vec<&Arc<dyn ScanRule>> {
        self.rules
            .iter()
            .filter(|e| e.rule.metadata().tags.contains(&tag.to_string()) && e.enabled)
            .map(|e| &e.rule)
            .collect()
    }
}

impl Default for ScanRuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
