#![allow(dead_code)]

use crate::findings::collector::FindingCollector;
use crate::findings::confidence::calculate_confidence;
use crate::findings::correlator::CrossRuleCorrelator;
use crate::findings::deduplicator::Deduplicator;
use crate::findings::events::{CorrelationEvent, CorrelationEventBus};
use crate::findings::models::{CorrelatedFinding, FindingLifecycle, FindingSeverity};
use crate::findings::normalizer::Normalizer;
use crate::findings::severity::{resolve_severity, SeverityStrategy};

pub struct CorrelationEngine {
    collector: FindingCollector,
    deduplicator: Deduplicator,
    cross_correlator: CrossRuleCorrelator,
    event_bus: CorrelationEventBus,
    severity_strategy: SeverityStrategy,
}

impl CorrelationEngine {
    pub fn new() -> Self {
        Self {
            collector: FindingCollector::new(),
            deduplicator: Deduplicator::new(0.5),
            cross_correlator: CrossRuleCorrelator::new(),
            event_bus: CorrelationEventBus::new(256),
            severity_strategy: SeverityStrategy::Highest,
        }
    }

    pub fn event_bus(&self) -> CorrelationEventBus {
        self.event_bus.clone()
    }
    pub fn set_severity_strategy(&mut self, s: SeverityStrategy) {
        self.severity_strategy = s;
    }

    pub fn add_finding(&mut self, finding: CorrelatedFinding) {
        let url = Normalizer::normalize_url(&finding.target_url);
        let mut f = finding;
        f.target_url = url;
        self.event_bus.emit(CorrelationEvent::Created {
            id: f.id,
            title: f.title.clone(),
        });
        self.collector.add(f);
    }

    pub fn deduplicate_and_merge(&mut self) -> Vec<CorrelatedFinding> {
        let raw = self.collector.all().to_vec();
        let merged = self.deduplicator.deduplicate_with_merge(raw);
        let mut result = Vec::new();
        for mut f in merged {
            let source_count = f.source_rules.len();
            let ev_count = f.evidence.len();
            f.confidence = calculate_confidence(&f, source_count, ev_count);
            f.lifecycle = FindingLifecycle::Verified;
            result.push(f);
        }
        result
    }

    pub fn correlate_cross_rules(&self, findings: &[CorrelatedFinding]) -> Vec<String> {
        self.cross_correlator.correlate(findings)
    }

    pub fn resolve_severity(&self, findings: &[CorrelatedFinding]) -> FindingSeverity {
        let sevs: Vec<FindingSeverity> = findings.iter().map(|f| f.severity).collect();
        resolve_severity(&sevs, self.severity_strategy, None)
    }

    pub fn all_findings(&self) -> &[CorrelatedFinding] {
        self.collector.all()
    }
    pub fn count(&self) -> usize {
        self.collector.count()
    }
}
