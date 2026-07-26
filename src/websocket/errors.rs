#![allow(dead_code)]

#[derive(Debug)]
pub enum WsError {
    Parse(String),
    Frame(String),
    Connection(String),
    Replay(String),
}
impl std::fmt::Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "Parse: {e}"),
            Self::Frame(e) => write!(f, "Frame: {e}"),
            Self::Connection(e) => write!(f, "Connection: {e}"),
            Self::Replay(e) => write!(f, "Replay: {e}"),
        }
    }
}
impl std::error::Error for WsError {}
