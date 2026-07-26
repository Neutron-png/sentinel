#![allow(dead_code)]

use crate::findings::models::CorrelatedFinding;

pub struct CorrelationRule {
    pub name: String,
    pub rule_ids: Vec<String>,
    pub min_matches: usize,
}

impl CorrelationRule {
    pub fn new(name: &str, rule_ids: Vec<&str>, min_matches: usize) -> Self {
        Self {
            name: name.to_string(),
            rule_ids: rule_ids.into_iter().map(|s| s.to_string()).collect(),
            min_matches,
        }
    }
}

pub struct CrossRuleCorrelator {
    rules: Vec<CorrelationRule>,
}

impl CrossRuleCorrelator {
    pub fn new() -> Self {
        let mut s = Self { rules: Vec::new() };
        s.add_defaults();
        s
    }

    fn add_defaults(&mut self) {
        self.rules.push(CorrelationRule::new(
            "Missing CSP + XSS",
            vec!["PASSIVE-SEC-001", "ACTIVE-XSS-001"],
            2,
        ));
        self.rules.push(CorrelationRule::new(
            "Missing HSTS + Weak TLS",
            vec!["PASSIVE-SEC-002", "WSTG-CRYP-01"],
            2,
        ));
        self.rules.push(CorrelationRule::new(
            "Dir Listing + Path Traversal",
            vec!["ACTIVE-PT-001"],
            1,
        ));
    }

    pub fn add_rule(&mut self, rule: CorrelationRule) {
        self.rules.push(rule);
    }

    pub fn correlate(&self, findings: &[CorrelatedFinding]) -> Vec<String> {
        let mut results = Vec::new();
        for rule in &self.rules {
            let matches: Vec<&CorrelatedFinding> = findings
                .iter()
                .filter(|f| rule.rule_ids.contains(&f.rule_id))
                .collect();
            if matches.len() >= rule.min_matches {
                let names: Vec<&str> = matches.iter().map(|f| f.title.as_str()).collect();
                results.push(format!("{}: {}", rule.name, names.join(", ")));
            }
        }
        results
    }
}
