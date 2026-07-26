#![allow(dead_code)]

#[derive(Debug)]
pub enum PolicyError {
    Load(String),
    Validate(String),
    Apply(String),
    Export(String),
}
impl std::fmt::Display for PolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(e) => write!(f, "Load: {e}"),
            Self::Validate(e) => write!(f, "Validate: {e}"),
            Self::Apply(e) => write!(f, "Apply: {e}"),
            Self::Export(e) => write!(f, "Export: {e}"),
        }
    }
}
impl std::error::Error for PolicyError {}
