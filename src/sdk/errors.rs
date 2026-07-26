#![allow(dead_code)]

#[derive(Debug)]
pub enum SdkError {
    Registry(String),
    Lifecycle(String),
    Permission(String),
    Dependency(String),
}
impl std::fmt::Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(e) => write!(f, "Registry: {e}"),
            Self::Lifecycle(e) => write!(f, "Lifecycle: {e}"),
            Self::Permission(e) => write!(f, "Permission: {e}"),
            Self::Dependency(e) => write!(f, "Dependency: {e}"),
        }
    }
}
impl std::error::Error for SdkError {}
