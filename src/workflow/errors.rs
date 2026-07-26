#![allow(dead_code)]

#[derive(Debug)]
pub enum WorkflowError {
    Step(String),
    Variable(String),
    Condition(String),
    Execution(String),
    Record(String),
}
impl std::fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Step(e) => write!(f, "Step: {e}"),
            Self::Variable(e) => write!(f, "Variable: {e}"),
            Self::Condition(e) => write!(f, "Condition: {e}"),
            Self::Execution(e) => write!(f, "Execution: {e}"),
            Self::Record(e) => write!(f, "Record: {e}"),
        }
    }
}
impl std::error::Error for WorkflowError {}
