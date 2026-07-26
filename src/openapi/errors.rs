#![allow(dead_code)]

#[derive(Debug)]
pub enum OpenApiError {
    Parse(String),
    Validate(String),
    Import(String),
    Generate(String),
}
impl std::fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "Parse: {e}"),
            Self::Validate(e) => write!(f, "Validate: {e}"),
            Self::Import(e) => write!(f, "Import: {e}"),
            Self::Generate(e) => write!(f, "Generate: {e}"),
        }
    }
}
impl std::error::Error for OpenApiError {}
