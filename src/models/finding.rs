use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl Severity {
    pub const ALL: &'static [Severity] = &[
        Self::Critical,
        Self::High,
        Self::Medium,
        Self::Low,
        Self::Informational,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Critical",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Informational => "Informational",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Critical" => Some(Self::Critical),
            "High" => Some(Self::High),
            "Medium" => Some(Self::Medium),
            "Low" => Some(Self::Low),
            "Informational" => Some(Self::Informational),
            _ => None,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    Confirmed,
    High,
    Medium,
    Low,
}

impl Confidence {
    pub const ALL: &'static [Confidence] = &[Self::Confirmed, Self::High, Self::Medium, Self::Low];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Confirmed => "Confirmed",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Confirmed" => Some(Self::Confirmed),
            "High" => Some(Self::High),
            "Medium" => Some(Self::Medium),
            "Low" => Some(Self::Low),
            _ => None,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingStatus {
    Draft,
    Verified,
    #[serde(rename = "False Positive")]
    FalsePositive,
    #[serde(rename = "Accepted Risk")]
    AcceptedRisk,
}

impl FindingStatus {
    pub const ALL: &'static [FindingStatus] = &[
        Self::Draft,
        Self::Verified,
        Self::FalsePositive,
        Self::AcceptedRisk,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Verified => "Verified",
            Self::FalsePositive => "False Positive",
            Self::AcceptedRisk => "Accepted Risk",
        }
    }

    pub fn from_label_for_db(label: &str) -> Option<Self> {
        match label {
            "Draft" => Some(Self::Draft),
            "Verified" => Some(Self::Verified),
            "False Positive" => Some(Self::FalsePositive),
            "Accepted Risk" => Some(Self::AcceptedRisk),
            _ => None,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

impl std::fmt::Display for FindingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl FromStr for FindingStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_label_for_db(s).ok_or_else(|| format!("unknown finding status: {s}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub task_id: Uuid,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub status: FindingStatus,
    pub impact: String,
    pub recommendation: String,
    pub references: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
