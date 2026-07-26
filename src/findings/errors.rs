#![allow(dead_code)]

#[derive(Debug)]
pub enum CorrelationError {
    Collection(String),
    Dedup(String),
    Merge(String),
}
impl std::fmt::Display for CorrelationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Collection(e) => write!(f, "Collection: {e}"),
            Self::Dedup(e) => write!(f, "Dedup: {e}"),
            Self::Merge(e) => write!(f, "Merge: {e}"),
        }
    }
}
impl std::error::Error for CorrelationError {}
