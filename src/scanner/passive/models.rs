#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub id: Uuid,
    pub rule_id: String,
    pub transaction_id: Uuid,
    pub assessment_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub severity: ScanSeverity,
    pub confidence: ScanConfidence,
    pub evidence: String,
    pub references: String,
    pub timestamp: DateTime<Utc>,
}

impl ScanResult {
    pub fn new(
        rule_id: &str,
        transaction_id: Uuid,
        title: &str,
        severity: ScanSeverity,
        confidence: ScanConfidence,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id: rule_id.to_string(),
            transaction_id,
            assessment_id: None,
            title: title.to_string(),
            description: String::new(),
            severity,
            confidence,
            evidence: String::new(),
            references: String::new(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl ScanSeverity {
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
pub enum ScanConfidence {
    Confirmed,
    High,
    Medium,
    Low,
    Tentative,
}

impl ScanConfidence {
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
