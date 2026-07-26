#![allow(dead_code)]

use uuid::Uuid;

use crate::findings::models::{CorrelatedFinding, DuplicateResult};

pub struct Deduplicator {
    pub threshold: f64,
}

impl Deduplicator {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn check_duplicate(&self, a: &CorrelatedFinding, b: &CorrelatedFinding) -> DuplicateResult {
        let mut score = 0.0;
        if a.rule_id == b.rule_id {
            score += 0.4;
        }
        if a.target_url == b.target_url {
            score += 0.2;
        }
        if a.endpoint == b.endpoint {
            score += 0.2;
        }
        if a.parameter == b.parameter && !a.parameter.is_empty() {
            score += 0.2;
        }
        DuplicateResult {
            is_duplicate: score >= self.threshold,
            matched_with: if score >= self.threshold {
                Some(b.id)
            } else {
                None
            },
            similarity: score,
        }
    }

    pub fn deduplicate(&self, findings: &[CorrelatedFinding]) -> Vec<CorrelatedFinding> {
        let mut result: Vec<CorrelatedFinding> = Vec::new();
        let mut seen: Vec<Uuid> = Vec::new();
        for f in findings {
            let is_dup = seen.iter().any(|id| {
                if let Some(existing) = result.iter().find(|r| r.id == *id) {
                    self.check_duplicate(f, existing).is_duplicate
                } else {
                    false
                }
            });
            if !is_dup {
                seen.push(f.id);
                result.push(f.clone());
            }
        }
        result
    }

    pub fn deduplicate_with_merge(
        &self,
        findings: Vec<CorrelatedFinding>,
    ) -> Vec<CorrelatedFinding> {
        let mut result = self.deduplicate(&findings);
        for f in result.iter_mut() {
            let dups: Vec<&CorrelatedFinding> = findings
                .iter()
                .filter(|o| o.id != f.id && self.check_duplicate(f, o).is_duplicate)
                .collect();
            for d in dups {
                f.evidence.extend(d.evidence.clone());
                if !f.source_rules.contains(&d.rule_id) {
                    f.source_rules.push(d.rule_id.clone());
                }
            }
        }
        result
    }
}
