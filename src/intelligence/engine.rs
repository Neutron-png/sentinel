#![allow(dead_code)]

use uuid::Uuid;

use crate::intelligence::errors::IntelligenceError;
use crate::intelligence::events::{IntelligenceEvent, IntelligenceEventBus};
use crate::intelligence::knowledge::KnowledgeRepository;
use crate::intelligence::models::{EnrichmentResult, KnowledgeEntry, TechnologyStack};
use crate::intelligence::remediation::RemediationGenerator;
use crate::intelligence::resolver::ReferenceResolver;

pub struct IntelligenceEngine {
    repository: KnowledgeRepository,
    event_bus: IntelligenceEventBus,
}

impl IntelligenceEngine {
    pub fn new() -> Self {
        Self {
            repository: KnowledgeRepository::new(),
            event_bus: IntelligenceEventBus::new(256),
        }
    }

    pub fn event_bus(&self) -> IntelligenceEventBus {
        self.event_bus.clone()
    }

    pub fn enrich(
        &self,
        finding_id: Uuid,
        cwe: &str,
        tech: &TechnologyStack,
    ) -> Result<EnrichmentResult, IntelligenceError> {
        let knowledge = self.repository.entries.get(cwe).cloned();
        let resolved = ReferenceResolver::resolve_all(&self.repository.references, cwe);
        let remediation = knowledge
            .as_ref()
            .and_then(|_| RemediationGenerator::generate_for(cwe, tech));

        let result = EnrichmentResult {
            finding_id,
            knowledge,
            related_cwe: resolved.capec,
            related_findings: vec![],
            remediation,
        };

        if let Some(ref k) = result.knowledge {
            self.event_bus.emit(IntelligenceEvent::FindingEnriched {
                finding_id,
                cwe: k.cwe_id.clone(),
            });
            self.event_bus.emit(IntelligenceEvent::ReferenceResolved {
                from: cwe.to_string(),
                to: k.owasp_category.clone(),
            });
        }
        Ok(result)
    }

    pub fn resolve_references(&self, cwe: &str) -> crate::intelligence::resolver::ResolvedRefs {
        ReferenceResolver::resolve_all(&self.repository.references, cwe)
    }

    pub fn search_by_cwe(&self, cwe: &str) -> Vec<&KnowledgeEntry> {
        self.repository
            .entries
            .values()
            .filter(|e| e.cwe_id.eq_ignore_ascii_case(cwe))
            .collect()
    }

    pub fn search_by_owasp(&self, category: &str) -> Vec<&KnowledgeEntry> {
        let cat = category.to_lowercase();
        self.repository
            .entries
            .values()
            .filter(|e| e.owasp_category.to_lowercase().contains(&cat))
            .collect()
    }

    pub fn search_by_keyword(&self, kw: &str) -> Vec<&KnowledgeEntry> {
        let k = kw.to_lowercase();
        self.repository
            .entries
            .values()
            .filter(|e| {
                e.description.to_lowercase().contains(&k) || e.cwe_name.to_lowercase().contains(&k)
            })
            .collect()
    }

    pub fn cwe_count(&self) -> usize {
        self.repository.entries.len()
    }
}
