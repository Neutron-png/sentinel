#![allow(dead_code)]

use crate::findings::models::CorrelatedFinding;

pub fn merge_evidence(target: &mut CorrelatedFinding, source: &CorrelatedFinding) {
    for ev in &source.evidence {
        if !target.evidence.contains(ev) {
            target.evidence.push(ev.clone());
        }
    }
    for rule in &source.source_rules {
        if !target.source_rules.contains(rule) {
            target.source_rules.push(rule.clone());
        }
    }
}
