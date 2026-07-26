#![allow(dead_code)]

#[derive(Debug)]
pub enum CrawlerError {
    QueueFull,
    Navigation(String),
    Extraction(String),
    ScopeRejected(String),
}
impl std::fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "Queue full"),
            Self::Navigation(e) => write!(f, "Navigation: {e}"),
            Self::Extraction(e) => write!(f, "Extraction: {e}"),
            Self::ScopeRejected(e) => write!(f, "Scope rejected: {e}"),
        }
    }
}
impl std::error::Error for CrawlerError {}
