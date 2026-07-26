#![allow(dead_code)]

#[derive(Debug)]
pub enum EvidenceError {
    Storage(String),
    Hash(String),
    NotFound(String),
    Immutable,
}
impl std::fmt::Display for EvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(e) => write!(f, "Storage: {e}"),
            Self::Hash(e) => write!(f, "Hash: {e}"),
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Immutable => write!(f, "Evidence is immutable"),
        }
    }
}
impl std::error::Error for EvidenceError {}
