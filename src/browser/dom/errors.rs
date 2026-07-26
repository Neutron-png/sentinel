#![allow(dead_code)]

#[derive(Debug)]
pub enum DomError {
    Query(String),
    Script(String),
    ElementNotFound(String),
    Storage(String),
    Timeout(String),
}
impl std::fmt::Display for DomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query(e) => write!(f, "Query: {e}"),
            Self::Script(e) => write!(f, "Script: {e}"),
            Self::ElementNotFound(e) => write!(f, "Element not found: {e}"),
            Self::Storage(e) => write!(f, "Storage: {e}"),
            Self::Timeout(e) => write!(f, "Timeout: {e}"),
        }
    }
}
impl std::error::Error for DomError {}
