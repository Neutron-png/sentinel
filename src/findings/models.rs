#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingLifecycle {
    New,
    Updated,
    Verified,
    Duplicate,
    Ignored,
    Resolved,
}

impl FindingLifecycle {
    pub fn label(&self) -> &'static str {
        match self {
            Self::New => "New",
            Self::Updated => "Updated",
            Self::Verified => "Verified",
            Self::Duplicate => "Duplicate",
            Self::Ignored => "Ignored",
            Self::Resolved => "Resolved",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorrelatedFinding {
    pub id: Uuid,
    pub rule_id: String,
    pub title: String,
    pub description: String,
    pub severity: FindingSeverity,
    pub confidence: FindingConfidence,
    pub target_url: String,
    pub endpoint: String,
    pub parameter: String,
    pub evidence: Vec<String>,
    pub duplicate_of: Option<Uuid>,
    pub lifecycle: FindingLifecycle,
    pub source_rules: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingSeverity {
    Informational = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl FindingSeverity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Informational => "Informational",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Self::Critical,
            "high" => Self::High,
            "medium" => Self::Medium,
            "low" => Self::Low,
            _ => Self::Informational,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingConfidence {
    Low,
    Medium,
    High,
    Confirmed,
}

impl FindingConfidence {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Confirmed => "Confirmed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DuplicateResult {
    pub is_duplicate: bool,
    pub matched_with: Option<Uuid>,
    pub similarity: f64,
}
