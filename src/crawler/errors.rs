#![allow(dead_code)]

#[derive(Debug)]
pub enum CrawlerError {
    QueueFull(String),
    Parse(String),
    Request(String),
    Scope(String),
}
impl std::fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull(e) => write!(f, "Queue full: {e}"),
            Self::Parse(e) => write!(f, "Parse: {e}"),
            Self::Request(e) => write!(f, "Request: {e}"),
            Self::Scope(e) => write!(f, "Scope: {e}"),
        }
    }
}
impl std::error::Error for CrawlerError {}
