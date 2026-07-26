#![allow(dead_code)]

#[derive(Debug)]
pub enum SdkError {
    Registry(String),
    Execution(String),
    Validation(String),
}
impl std::fmt::Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(e) => write!(f, "Registry: {e}"),
            Self::Execution(e) => write!(f, "Execution: {e}"),
            Self::Validation(e) => write!(f, "Validation: {e}"),
        }
    }
}
impl std::error::Error for SdkError {}
