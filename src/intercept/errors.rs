#![allow(dead_code)]

#[derive(Debug)]
pub enum InterceptError {
    NotFound(String),
    AlreadyProcessed(String),
    QueueFull(String),
    EditFailed(String),
}

impl std::fmt::Display for InterceptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::AlreadyProcessed(e) => write!(f, "Already processed: {e}"),
            Self::QueueFull(e) => write!(f, "Queue full: {e}"),
            Self::EditFailed(e) => write!(f, "Edit failed: {e}"),
        }
    }
}
impl std::error::Error for InterceptError {}
