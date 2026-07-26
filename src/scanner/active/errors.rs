#![allow(dead_code)]

#[derive(Debug)]
pub enum ScanError {
    NotFound(String),
    JobLimit(String),
    TaskFailed(String),
    Cancelled,
}
impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::JobLimit(e) => write!(f, "Job limit: {e}"),
            Self::TaskFailed(e) => write!(f, "Task failed: {e}"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}
impl std::error::Error for ScanError {}
