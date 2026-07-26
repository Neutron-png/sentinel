#![allow(dead_code)]

use std::collections::HashMap;
use uuid::Uuid;

use crate::evidence::models::EvidenceItem;

pub struct EvidenceStorage {
    pub items: HashMap<Uuid, EvidenceItem>,
}

impl EvidenceStorage {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn store(&mut self, item: EvidenceItem) -> &EvidenceItem {
        let id = item.id;
        self.items.insert(id, item);
        self.items.get(&id).unwrap()
    }

    pub fn get(&self, id: Uuid) -> Option<&EvidenceItem> {
        self.items.get(&id)
    }
    pub fn get_by_assessment(&self, assessment_id: Uuid) -> Vec<&EvidenceItem> {
        self.items
            .values()
            .filter(|e| e.assessment_id == assessment_id)
            .collect()
    }
    pub fn get_by_finding(&self, finding_id: Uuid) -> Vec<&EvidenceItem> {
        self.items
            .values()
            .filter(|e| e.linked_findings.contains(&finding_id))
            .collect()
    }
    pub fn all(&self) -> Vec<&EvidenceItem> {
        self.items.values().collect()
    }
    pub fn delete(&mut self, id: Uuid) -> Result<(), super::errors::EvidenceError> {
        self.items
            .remove(&id)
            .ok_or(super::errors::EvidenceError::NotFound(id.to_string()))?;
        Ok(())
    }
}
