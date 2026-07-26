#![allow(dead_code)]

use uuid::Uuid;

use crate::scope::errors::ScopeError;
use crate::scope::events::{ScopeEvent, ScopeEventBus};
use crate::scope::matcher;
use crate::scope::models::{ScopeRule, ScopeVerdict};

pub struct ScopeEngine {
    rules: Vec<ScopeRule>,
    assessment_id: Option<Uuid>,
    event_bus: ScopeEventBus,
}

impl ScopeEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            assessment_id: None,
            event_bus: ScopeEventBus::new(128),
        }
    }

    pub fn event_bus(&self) -> ScopeEventBus {
        self.event_bus.clone()
    }

    pub fn load_rules(&mut self, assessment_id: Uuid, rules: Vec<ScopeRule>) {
        self.assessment_id = Some(assessment_id);
        self.rules = rules;
        self.event_bus.emit(ScopeEvent::Loaded {
            assessment_id,
            rule_count: self.rules.len(),
        });
    }

    pub fn unload(&mut self) {
        self.rules.clear();
        self.assessment_id = None;
    }

    pub fn add_rule(&mut self, rule: ScopeRule) -> Result<(), ScopeError> {
        self.rules.push(rule);
        if let Some(aid) = self.assessment_id {
            self.event_bus
                .emit(ScopeEvent::Updated { assessment_id: aid });
        }
        Ok(())
    }

    pub fn remove_rule(&mut self, id: Uuid) -> Result<(), ScopeError> {
        self.rules.retain(|r| r.id != id);
        if let Some(aid) = self.assessment_id {
            self.event_bus
                .emit(ScopeEvent::Updated { assessment_id: aid });
        }
        Ok(())
    }

    pub fn update_rule(&mut self, rule: ScopeRule) -> Result<(), ScopeError> {
        if let Some(r) = self.rules.iter_mut().find(|r| r.id == rule.id) {
            *r = rule;
        }
        if let Some(aid) = self.assessment_id {
            self.event_bus
                .emit(ScopeEvent::Updated { assessment_id: aid });
        }
        Ok(())
    }

    pub fn evaluate(&self, host: &str, port: u16, scheme: &str, url: &str) -> ScopeVerdict {
        let verdict = matcher::evaluate(&self.rules, host, port, scheme, url);
        match verdict {
            ScopeVerdict::InScope => self.event_bus.emit(ScopeEvent::Matched {
                url: url.into(),
                verdict,
            }),
            ScopeVerdict::OutOfScope => self
                .event_bus
                .emit(ScopeEvent::Rejected { url: url.into() }),
            _ => {}
        }
        verdict
    }

    pub fn is_in_scope(&self, host: &str, port: u16, scheme: &str, url: &str) -> bool {
        self.evaluate(host, port, scheme, url) == ScopeVerdict::InScope
    }

    pub fn rules(&self) -> &[ScopeRule] {
        &self.rules
    }
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}
