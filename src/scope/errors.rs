#![allow(dead_code)]

#[derive(Debug)]
pub enum ScopeError {
    NotFound(String),
    InvalidRule(String),
    Storage(String),
}
impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::InvalidRule(e) => write!(f, "Invalid rule: {e}"),
            Self::Storage(e) => write!(f, "Storage: {e}"),
        }
    }
}
impl std::error::Error for ScopeError {}
