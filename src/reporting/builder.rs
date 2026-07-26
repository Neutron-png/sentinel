#![allow(dead_code)]

use crate::reporting::models::{FindingSummary, ReportData, SeverityCounts};

pub struct ReportBuilder;

impl ReportBuilder {
    pub fn build(assessment_name: &str, target: &str, findings: &[FindingSummary]) -> ReportData {
        let mut counts = SeverityCounts::default();
        for f in findings {
            match f.severity.to_lowercase().as_str() {
                "critical" => counts.critical += 1,
                "high" => counts.high += 1,
                "medium" => counts.medium += 1,
                "low" => counts.low += 1,
                _ => counts.informational += 1,
            }
        }
        ReportData {
            assessment_name: assessment_name.to_string(),
            assessment_target: target.to_string(),
            scope: String::new(),
            duration: String::new(),
            total_findings: findings.len(),
            findings: findings.to_vec(),
            severity_counts: counts,
            generated_at: chrono::Utc::now(),
        }
    }
}
