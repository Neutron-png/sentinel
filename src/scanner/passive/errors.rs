#![allow(dead_code)]

#[derive(Debug)]
pub enum ScannerError {
    RuleError(String),
    ExecutionError(String),
}
impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuleError(e) => write!(f, "Rule error: {e}"),
            Self::ExecutionError(e) => write!(f, "Execution error: {e}"),
        }
    }
}
impl std::error::Error for ScannerError {}
