#![allow(dead_code)]

#[derive(Debug)]
pub enum AnalyzerError {
    Comparison(String),
    Analysis(String),
}
impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comparison(e) => write!(f, "Comparison: {e}"),
            Self::Analysis(e) => write!(f, "Analysis: {e}"),
        }
    }
}
impl std::error::Error for AnalyzerError {}
