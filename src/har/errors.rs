#![allow(dead_code)]

#[derive(Debug)]
pub enum HarError {
    Export(String),
    Import(String),
    Parse(String),
    InvalidFormat(String),
}
impl std::fmt::Display for HarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Export(e) => write!(f, "Export: {e}"),
            Self::Import(e) => write!(f, "Import: {e}"),
            Self::Parse(e) => write!(f, "Parse: {e}"),
            Self::InvalidFormat(e) => write!(f, "Invalid format: {e}"),
        }
    }
}
impl std::error::Error for HarError {}
