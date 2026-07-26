#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum PipelineError {
    Prepare(String),
    Validation(String),
    Request(String),
    Timeout(String),
    Network(String),
    Rule(String),
    Internal(String),
    Cancelled,
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prepare(e) => write!(f, "Prepare: {e}"),
            Self::Validation(e) => write!(f, "Validation: {e}"),
            Self::Request(e) => write!(f, "Request: {e}"),
            Self::Timeout(e) => write!(f, "Timeout: {e}"),
            Self::Network(e) => write!(f, "Network: {e}"),
            Self::Rule(e) => write!(f, "Rule: {e}"),
            Self::Internal(e) => write!(f, "Internal: {e}"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}
impl std::error::Error for PipelineError {}
