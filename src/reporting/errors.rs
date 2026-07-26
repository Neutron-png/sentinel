#![allow(dead_code)]

#[derive(Debug)]
pub enum ReportError {
    Build(String),
    Export(String),
    Template(String),
}
impl std::fmt::Display for ReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Build(e) => write!(f, "Build: {e}"),
            Self::Export(e) => write!(f, "Export: {e}"),
            Self::Template(e) => write!(f, "Template: {e}"),
        }
    }
}
impl std::error::Error for ReportError {}
