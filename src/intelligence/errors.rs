#![allow(dead_code)]

#[derive(Debug)]
pub enum IntelligenceError {
    NotFound(String),
    Enrichment(String),
    Reference(String),
}
impl std::fmt::Display for IntelligenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Enrichment(e) => write!(f, "Enrichment: {e}"),
            Self::Reference(e) => write!(f, "Reference: {e}"),
        }
    }
}
impl std::error::Error for IntelligenceError {}
