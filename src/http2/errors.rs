#![allow(dead_code)]

#[derive(Debug)]
pub enum Http2Error {
    Protocol(String),
    Stream(String),
    Frame(String),
    Compression(String),
    Connection(String),
    ConnectionNotFound(String),
    FlowControl(String),
    Settings(String),
    GoAway(String),
    Other(String),
}

impl std::fmt::Display for Http2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Protocol(e) => write!(f, "HTTP/2 protocol error: {e}"),
            Self::Stream(e) => write!(f, "HTTP/2 stream error: {e}"),
            Self::Frame(e) => write!(f, "HTTP/2 frame error: {e}"),
            Self::Compression(e) => write!(f, "HTTP/2 compression error: {e}"),
            Self::Connection(e) => write!(f, "HTTP/2 connection error: {e}"),
            Self::ConnectionNotFound(e) => write!(f, "HTTP/2 connection not found: {e}"),
            Self::FlowControl(e) => write!(f, "HTTP/2 flow control error: {e}"),
            Self::Settings(e) => write!(f, "HTTP/2 settings error: {e}"),
            Self::GoAway(e) => write!(f, "HTTP/2 GOAWAY error: {e}"),
            Self::Other(e) => write!(f, "HTTP/2 error: {e}"),
        }
    }
}

impl std::error::Error for Http2Error {}
