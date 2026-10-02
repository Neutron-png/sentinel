#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TokenClass {
    Session,
    Csrf,
    PasswordReset,
    Generic,
}

impl TokenClass {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Session => "session token",
            Self::Csrf => "CSRF token",
            Self::PasswordReset => "password-reset token",
            Self::Generic => "generic token",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LengthStats {
    pub min: usize,
    pub max: usize,
    pub fixed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PositionStats {
    pub position: usize,
    pub samples: usize,
    pub distinct_chars: usize,
    pub entropy_bits: f64,
    pub max_entropy_bits: f64,
    pub most_common: Option<(char, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ObservationKind {
    LowEntropyPosition,
    RepeatedValue,
    Sequential,
    FixedPrefix,
    FixedSuffix,
    SmallAlphabet,
    VariableLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ObservationSeverity {
    Informational,
    Low,
    Medium,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Observation {
    pub kind: ObservationKind,
    pub severity: ObservationSeverity,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TokenAnalysis {
    pub class: TokenClass,
    pub sample_count: usize,
    pub unique_count: usize,
    pub length: LengthStats,
    pub alphabet_size: usize,
    pub shannon_entropy_bits_per_char: f64,
    pub ideal_entropy_bits_per_char: f64,
    pub total_entropy_bits: f64,
    pub per_position: Vec<PositionStats>,
    pub observations: Vec<Observation>,
    pub methodology: String,
    pub confidence: u8,
}
