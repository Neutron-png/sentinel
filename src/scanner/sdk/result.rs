#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}
impl RuleSeverity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Critical",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Informational => "Informational",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleConfidence {
    Confirmed,
    High,
    Medium,
    Low,
    Tentative,
}
impl RuleConfidence {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Confirmed => "Confirmed",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Tentative => "Tentative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleStatus {
    Passed,
    Informational,
    Potential,
    Confirmed,
    Error,
    Skipped,
}
impl RuleStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Passed => "Passed",
            Self::Informational => "Informational",
            Self::Potential => "Potential",
            Self::Confirmed => "Confirmed",
            Self::Error => "Error",
            Self::Skipped => "Skipped",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleResult {
    pub rule_id: String,
    pub status: RuleStatus,
    pub severity: RuleSeverity,
    pub confidence: RuleConfidence,
    pub title: String,
    pub description: String,
    pub recommendation: String,
    pub references: String,
    pub evidence: String,
    pub transactions: Vec<uuid::Uuid>,
}

impl RuleResult {
    pub fn new(rule_id: &str, title: &str, severity: RuleSeverity) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            status: RuleStatus::Potential,
            severity,
            confidence: RuleConfidence::Medium,
            title: title.to_string(),
            description: String::new(),
            recommendation: String::new(),
            references: String::new(),
            evidence: String::new(),
            transactions: vec![],
        }
    }

    pub fn with_status(mut self, s: RuleStatus) -> Self {
        self.status = s;
        self
    }
    pub fn with_confidence(mut self, c: RuleConfidence) -> Self {
        self.confidence = c;
        self
    }
    pub fn with_description(mut self, d: &str) -> Self {
        self.description = d.into();
        self
    }
    pub fn with_evidence(mut self, e: &str) -> Self {
        self.evidence = e.into();
        self
    }
    pub fn with_transaction(mut self, id: uuid::Uuid) -> Self {
        self.transactions.push(id);
        self
    }
}
