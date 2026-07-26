#![allow(dead_code)]

use std::collections::HashMap;
use uuid::Uuid;

use crate::policy::errors::PolicyError;
use crate::policy::models::ScanPolicy;

pub struct PolicyStorage {
    policies: HashMap<Uuid, ScanPolicy>,
}

impl PolicyStorage {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }

    pub fn store(&mut self, policy: ScanPolicy) {
        self.policies.insert(policy.id, policy);
    }
    pub fn get(&self, id: Uuid) -> Option<&ScanPolicy> {
        self.policies.get(&id)
    }
    pub fn all(&self) -> Vec<&ScanPolicy> {
        self.policies.values().collect()
    }
    pub fn delete(&mut self, id: Uuid) -> Result<(), PolicyError> {
        self.policies
            .remove(&id)
            .ok_or(PolicyError::Load("Not found".into()))?;
        Ok(())
    }
    pub fn export_json(&self, id: Uuid) -> Result<String, PolicyError> {
        let p = self
            .policies
            .get(&id)
            .ok_or(PolicyError::Export("Not found".into()))?;
        serde_json::to_string_pretty(p).map_err(|e| PolicyError::Export(e.to_string()))
    }
}
