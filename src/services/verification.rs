use crate::models::finding::Finding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    Incomplete,
    ReadyForReview,
    Verified,
}

impl VerificationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Incomplete => "Incomplete",
            Self::ReadyForReview => "Ready for Review",
            Self::Verified => "Verified",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Incomplete => "\u{25CB}",
            Self::ReadyForReview => "\u{25CF}",
            Self::Verified => "\u{2713}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub status: VerificationStatus,
    pub missing: Vec<String>,
}

pub fn verify(finding: &Finding, linked_evidence_count: usize) -> VerificationResult {
    let mut missing = Vec::new();

    if linked_evidence_count == 0 {
        missing.push("Missing Evidence".into());
    }
    if finding.description.trim().is_empty() {
        missing.push("Missing Description".into());
    }
    if finding.impact.trim().is_empty() {
        missing.push("Missing Impact".into());
    }
    if finding.recommendation.trim().is_empty() {
        missing.push("Missing Recommendation".into());
    }
    if finding.title.trim().is_empty() {
        missing.push("Missing Title".into());
    }

    let has_severity = true;
    let has_confidence = true;
    let _ = (has_severity, has_confidence);

    let status = if missing.is_empty() {
        match finding.status {
            crate::models::finding::FindingStatus::Verified => VerificationStatus::Verified,
            _ => VerificationStatus::ReadyForReview,
        }
    } else {
        VerificationStatus::Incomplete
    };

    VerificationResult { status, missing }
}
