#![allow(dead_code)]

#[derive(Debug)]
pub enum ObserverError {
    Capture(String),
    Classify(String),
    Correlate(String),
}
impl std::fmt::Display for ObserverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capture(e) => write!(f, "Capture: {e}"),
            Self::Classify(e) => write!(f, "Classify: {e}"),
            Self::Correlate(e) => write!(f, "Correlate: {e}"),
        }
    }
}
impl std::error::Error for ObserverError {}
