#![allow(dead_code)]

#[derive(Debug)]
pub enum SessionError {
    NotFound(String),
    Storage(String),
    Refresh(String),
    Isolation(String),
}
impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Storage(e) => write!(f, "Storage: {e}"),
            Self::Refresh(e) => write!(f, "Refresh: {e}"),
            Self::Isolation(e) => write!(f, "Isolation: {e}"),
        }
    }
}
impl std::error::Error for SessionError {}
