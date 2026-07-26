#![allow(dead_code)]

#[derive(Debug)]
pub enum AuthError {
    Record(String),
    Replay(String),
    Detection(String),
    Session(String),
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Record(e) => write!(f, "Record: {e}"),
            Self::Replay(e) => write!(f, "Replay: {e}"),
            Self::Detection(e) => write!(f, "Detection: {e}"),
            Self::Session(e) => write!(f, "Session: {e}"),
        }
    }
}
impl std::error::Error for AuthError {}
