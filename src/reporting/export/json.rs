#![allow(dead_code)]

use crate::reporting::export::ReportExporter;
use crate::reporting::models::{OutputFormat, ReportData};

pub struct JsonExporter;

impl ReportExporter for JsonExporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Json
    }
    fn file_extension(&self) -> &'static str {
        "json"
    }

    fn export(&self, data: &ReportData) -> String {
        serde_json::to_string_pretty(&serde_json::json!({
            "assessment": { "name": data.assessment_name, "target": data.assessment_target },
            "total_findings": data.total_findings,
            "severity_counts": {
                "critical": data.severity_counts.critical,
                "high": data.severity_counts.high,
                "medium": data.severity_counts.medium,
                "low": data.severity_counts.low,
                "informational": data.severity_counts.informational,
            },
            "findings": data.findings.iter().map(|f| serde_json::json!({
                "title": f.title, "severity": f.severity, "confidence": f.confidence,
                "description": f.description, "impact": f.impact, "recommendation": f.recommendation,
                "evidence_count": f.evidence.len(),
            })).collect::<Vec<_>>(),
        })).unwrap_or_default()
    }
}
