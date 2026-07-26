#![allow(dead_code)]

use std::collections::HashMap;

pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (k, v) in &self.variables {
            result = result.replace(&format!("{{{{{}}}}}", k), v);
        }
        result
    }

    pub fn render_executive(&self, data: &super::models::ReportData) -> String {
        let mut t = String::new();
        t.push_str(&format!(
            "# Executive Summary: {}\n\n",
            data.assessment_name
        ));
        t.push_str(&format!("**Target**: {}\n\n", data.assessment_target));
        t.push_str(&format!("**Total Findings**: {}\n\n", data.total_findings));
        t.push_str("## Severity Distribution\n\n");
        t.push_str(&format!("- Critical: {}\n", data.severity_counts.critical));
        t.push_str(&format!("- High: {}\n", data.severity_counts.high));
        t.push_str(&format!("- Medium: {}\n", data.severity_counts.medium));
        t.push_str(&format!("- Low: {}\n", data.severity_counts.low));
        t.push_str(&format!(
            "- Informational: {}\n",
            data.severity_counts.informational
        ));
        t
    }

    pub fn render_technical(&self, data: &super::models::ReportData) -> String {
        let mut t = format!("# Technical Report: {}\n\n", data.assessment_name);
        for (i, f) in data.findings.iter().enumerate() {
            t.push_str(&format!("## Finding {}: {}\n\n", i + 1, f.title));
            t.push_str(&format!("- **Severity**: {}\n", f.severity));
            t.push_str(&format!("- **Confidence**: {}\n", f.confidence));
            t.push_str(&format!("- **Description**: {}\n", f.description));
            t.push_str(&format!("- **Impact**: {}\n", f.impact));
            t.push_str(&format!("- **Recommendation**: {}\n", f.recommendation));
            if !f.evidence.is_empty() {
                t.push_str(&format!("- **Evidence**: {} items\n", f.evidence.len()));
            }
            t.push_str("\n---\n\n");
        }
        t
    }
}
