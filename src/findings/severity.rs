#![allow(dead_code)]

use crate::findings::models::FindingSeverity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityStrategy {
    Highest,
    Average,
    ManualOverride,
}

pub fn resolve_severity(
    severities: &[FindingSeverity],
    strategy: SeverityStrategy,
    manual: Option<FindingSeverity>,
) -> FindingSeverity {
    if let Some(m) = manual {
        return m;
    }
    match strategy {
        SeverityStrategy::Highest => *severities
            .iter()
            .max()
            .unwrap_or(&FindingSeverity::Informational),
        SeverityStrategy::Average => {
            let total: i32 = severities.iter().map(|s| *s as i32).sum();
            let avg = total as f64 / severities.len().max(1) as f64;
            match avg.round() as i32 {
                4 => FindingSeverity::Critical,
                3 => FindingSeverity::High,
                2 => FindingSeverity::Medium,
                1 => FindingSeverity::Low,
                _ => FindingSeverity::Informational,
            }
        }
        SeverityStrategy::ManualOverride => manual.unwrap_or(FindingSeverity::Medium),
    }
}
