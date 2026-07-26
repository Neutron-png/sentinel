#![allow(dead_code)]

use uuid::Uuid;

use crate::evidence::errors::EvidenceError;
use crate::evidence::events::{EvidenceEvent, EvidenceEventBus};
use crate::evidence::export;
use crate::evidence::hash;
use crate::evidence::linker;
use crate::evidence::models::{EvidenceItem, EvidenceType};
use crate::evidence::storage::EvidenceStorage;

pub struct EvidenceEngine {
    storage: EvidenceStorage,
    event_bus: EvidenceEventBus,
}

impl EvidenceEngine {
    pub fn new() -> Self {
        Self {
            storage: EvidenceStorage::new(),
            event_bus: EvidenceEventBus::new(512),
        }
    }

    pub fn event_bus(&self) -> EvidenceEventBus {
        self.event_bus.clone()
    }

    // ── CRUD ──

    pub fn create(
        &mut self,
        assessment_id: Uuid,
        ev_type: EvidenceType,
        content: Vec<u8>,
    ) -> &EvidenceItem {
        let mut item = EvidenceItem::new(assessment_id, ev_type);
        item.sha256_hash = hash::sha256(&content);
        item.content = content;
        let id = item.id;
        let type_label = ev_type.label().to_string();
        let item = self.storage.store(item);
        self.event_bus.emit(EvidenceEvent::Created {
            id,
            evidence_type: type_label,
        });
        item
    }

    pub fn get(&self, id: Uuid) -> Option<&EvidenceItem> {
        self.storage.get(id)
    }
    pub fn get_by_assessment(&self, assessment_id: Uuid) -> Vec<&EvidenceItem> {
        self.storage.get_by_assessment(assessment_id)
    }
    pub fn get_by_finding(&self, finding_id: Uuid) -> Vec<&EvidenceItem> {
        self.storage.get_by_finding(finding_id)
    }
    pub fn all(&self) -> Vec<&EvidenceItem> {
        self.storage.all()
    }

    // ── Link ──

    pub fn link_to_finding(
        &mut self,
        evidence_id: Uuid,
        finding_id: Uuid,
    ) -> Result<(), EvidenceError> {
        let items = &mut self.storage.items;
        if let Some(item) = items.get_mut(&evidence_id) {
            linker::link_to_finding(item, finding_id);
            self.event_bus.emit(EvidenceEvent::Linked {
                evidence_id,
                finding_id,
            });
            Ok(())
        } else {
            Err(EvidenceError::NotFound(evidence_id.to_string()))
        }
    }

    // ── Verify ──

    pub fn verify(&self, id: Uuid) -> Result<bool, EvidenceError> {
        let item = self
            .storage
            .get(id)
            .ok_or_else(|| EvidenceError::NotFound(id.to_string()))?;
        let valid = hash::verify(item);
        self.event_bus.emit(EvidenceEvent::Verified {
            evidence_id: id,
            hash_valid: valid,
        });
        Ok(valid)
    }

    // ── Search ──

    pub fn search_by_tag(&self, tag: &str) -> Vec<&EvidenceItem> {
        self.storage
            .items
            .values()
            .filter(|e| e.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect()
    }
    pub fn search_by_type(&self, t: EvidenceType) -> Vec<&EvidenceItem> {
        self.storage
            .items
            .values()
            .filter(|e| e.evidence_type == t)
            .collect()
    }
    pub fn search_by_keyword(&self, kw: &str) -> Vec<&EvidenceItem> {
        let k = kw.to_lowercase();
        self.storage
            .items
            .values()
            .filter(|e| {
                e.description.to_lowercase().contains(&k)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&k))
            })
            .collect()
    }

    // ── Export ──

    pub fn export_item(&self, id: Uuid) -> Option<String> {
        self.storage.get(id).map(export::export_item)
    }
    pub fn export_by_finding(&self, finding_id: Uuid) -> String {
        export::export_by_finding(&self.storage, finding_id)
    }
    pub fn export_by_assessment(&self, assessment_id: Uuid) -> String {
        export::export_by_assessment(&self.storage, assessment_id)
    }
}
