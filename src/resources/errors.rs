#![allow(dead_code)]

#[derive(Debug)]
pub enum ResourceError {
    RateLimit(String),
    Concurrency(String),
    Timeout(String),
    CircuitOpen(String),
}
impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RateLimit(e) => write!(f, "Rate limit: {e}"),
            Self::Concurrency(e) => write!(f, "Concurrency: {e}"),
            Self::Timeout(e) => write!(f, "Timeout: {e}"),
            Self::CircuitOpen(e) => write!(f, "Circuit open: {e}"),
        }
    }
}
impl std::error::Error for ResourceError {}
