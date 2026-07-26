#![allow(dead_code)]

#[derive(Debug)]
pub enum HistoryError {
    NotFound(String),
    Storage(String),
    Query(String),
}

impl std::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Storage(e) => write!(f, "Storage: {e}"),
            Self::Query(e) => write!(f, "Query: {e}"),
        }
    }
}
impl std::error::Error for HistoryError {}
