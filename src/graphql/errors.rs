#![allow(dead_code)]

#[derive(Debug)]
pub enum GraphQlError {
    Detection(String),
    Introspection(String),
    Parse(String),
    Validate(String),
    Inject(String),
}
impl std::fmt::Display for GraphQlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Detection(e) => write!(f, "Detection: {e}"),
            Self::Introspection(e) => write!(f, "Introspection: {e}"),
            Self::Parse(e) => write!(f, "Parse: {e}"),
            Self::Validate(e) => write!(f, "Validate: {e}"),
            Self::Inject(e) => write!(f, "Inject: {e}"),
        }
    }
}
impl std::error::Error for GraphQlError {}
