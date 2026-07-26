#![allow(dead_code)]

use crate::findings::models::CorrelatedFinding;

pub struct FindingCollector {
    findings: Vec<CorrelatedFinding>,
}

impl FindingCollector {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
        }
    }

    pub fn add(&mut self, finding: CorrelatedFinding) {
        self.findings.push(finding);
    }

    pub fn add_all(&mut self, findings: Vec<CorrelatedFinding>) {
        self.findings.extend(findings);
    }

    pub fn all(&self) -> &[CorrelatedFinding] {
        &self.findings
    }
    pub fn count(&self) -> usize {
        self.findings.len()
    }

    pub fn collect_from_passive(
        &mut self,
        _results: &[crate::scanner::passive::models::ScanResult],
    ) {
    }
    pub fn collect_from_active(
        &mut self,
        _results: &[crate::scanner::active::registry::ScanResult],
    ) {
    }
}
