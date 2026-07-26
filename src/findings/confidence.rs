#![allow(dead_code)]

use crate::findings::models::{CorrelatedFinding, FindingConfidence};

pub fn calculate_confidence(
    finding: &CorrelatedFinding,
    independent_sources: usize,
    evidence_count: usize,
) -> FindingConfidence {
    if evidence_count >= 3 && independent_sources >= 2 {
        return FindingConfidence::Confirmed;
    }
    if independent_sources >= 2 || evidence_count >= 2 {
        return FindingConfidence::High;
    }
    if evidence_count >= 1 {
        return FindingConfidence::Medium;
    }
    finding.confidence
}
