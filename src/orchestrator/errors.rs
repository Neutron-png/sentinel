#![allow(dead_code)]

#[derive(Debug)]
pub enum OrchestratorError {
    Phase(String),
    Scheduling(String),
    Module(String),
    Abort(String),
}
impl std::fmt::Display for OrchestratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(e) => write!(f, "Phase: {e}"),
            Self::Scheduling(e) => write!(f, "Schedule: {e}"),
            Self::Module(e) => write!(f, "Module: {e}"),
            Self::Abort(e) => write!(f, "Abort: {e}"),
        }
    }
}
impl std::error::Error for OrchestratorError {}
