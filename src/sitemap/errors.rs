#![allow(dead_code)]

#[derive(Debug)]
pub enum SiteMapError {
    NotFound(String),
    Duplicate(String),
}
impl std::fmt::Display for SiteMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::Duplicate(e) => write!(f, "Duplicate: {e}"),
        }
    }
}
impl std::error::Error for SiteMapError {}
