#![allow(dead_code)]

use uuid::Uuid;

use crate::policy::errors::PolicyError;
use crate::policy::events::{PolicyEvent, PolicyEventBus};
use crate::policy::limits::LimitEnforcer;
use crate::policy::loader;
use crate::policy::models::ScanPolicy;
use crate::policy::payloads::PayloadPolicy;
use crate::policy::rules;
use crate::policy::storage::PolicyStorage;
use crate::policy::validator;

pub struct PolicyEngine {
    storage: PolicyStorage,
    active_policy_id: Option<Uuid>,
    event_bus: PolicyEventBus,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            storage: PolicyStorage::new(),
            active_policy_id: None,
            event_bus: PolicyEventBus::new(128),
        }
    }

    pub fn event_bus(&self) -> PolicyEventBus {
        self.event_bus.clone()
    }

    pub fn load_file(&mut self, path: &str) -> Result<&ScanPolicy, PolicyError> {
        let policy = loader::load_from_file(path)?;
        validator::validate(&policy)?;
        let id = policy.id;
        self.storage.store(policy);
        self.event_bus.emit(PolicyEvent::Loaded {
            policy_id: id,
            name: self.storage.get(id).unwrap().name.clone(),
        });
        self.storage
            .get(id)
            .ok_or(PolicyError::Load("Storage error".into()))
    }

    pub fn apply(&mut self, policy_id: Uuid) -> Result<(), PolicyError> {
        self.storage
            .get(policy_id)
            .ok_or(PolicyError::Apply("Policy not found".into()))?;
        self.active_policy_id = Some(policy_id);
        self.event_bus.emit(PolicyEvent::Applied { policy_id });
        Ok(())
    }

    pub fn active_policy(&self) -> Option<&ScanPolicy> {
        self.active_policy_id.and_then(|id| self.storage.get(id))
    }

    pub fn is_rule_enabled(&self, rule_id: &str) -> bool {
        self.active_policy()
            .map(|p| rules::is_rule_enabled(&p.rules, rule_id))
            .unwrap_or(true)
    }

    pub fn should_stop(&self, requests: usize, depth: usize, elapsed: u64) -> bool {
        self.active_policy()
            .map(|p| LimitEnforcer::should_stop(&p.limits, requests, depth, elapsed))
            .unwrap_or(false)
    }

    pub fn allowed_category(&self, category: &str) -> bool {
        self.active_policy()
            .map(|p| PayloadPolicy::allowed_category(&p.payloads, category))
            .unwrap_or(true)
    }

    pub fn export_policy(&self, id: Uuid) -> Result<String, PolicyError> {
        let json = self.storage.export_json(id)?;
        self.event_bus.emit(PolicyEvent::Exported { policy_id: id });
        Ok(json)
    }

    pub fn all_policies(&self) -> Vec<&ScanPolicy> {
        self.storage.all()
    }
}
