#![allow(dead_code)]

#[derive(Debug)]
pub enum Http3Error {
    Transport(String),
    Stream(String),
    Qpack(String),
    Connection(String),
    ConnectionNotFound(String),
    VersionNegotiation(String),
    ZeroRtt(String),
    Migration(String),
    Other(String),
}

impl std::fmt::Display for Http3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(e) => write!(f, "HTTP/3 transport error: {e}"),
            Self::Stream(e) => write!(f, "HTTP/3 stream error: {e}"),
            Self::Qpack(e) => write!(f, "HTTP/3 QPACK error: {e}"),
            Self::Connection(e) => write!(f, "HTTP/3 connection error: {e}"),
            Self::ConnectionNotFound(e) => write!(f, "HTTP/3 connection not found: {e}"),
            Self::VersionNegotiation(e) => write!(f, "HTTP/3 version negotiation error: {e}"),
            Self::ZeroRtt(e) => write!(f, "HTTP/3 0-RTT error: {e}"),
            Self::Migration(e) => write!(f, "HTTP/3 migration error: {e}"),
            Self::Other(e) => write!(f, "HTTP/3 error: {e}"),
        }
    }
}

impl std::error::Error for Http3Error {}
