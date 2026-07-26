#![allow(dead_code)]

#[derive(Debug)]
pub enum ProxyError {
    Bind(String),
    Tls(String),
    Cert(String),
    Connection(String),
    Io(String),
    Shutdown(String),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bind(e) => write!(f, "Bind error: {e}"),
            Self::Tls(e) => write!(f, "TLS error: {e}"),
            Self::Cert(e) => write!(f, "Certificate error: {e}"),
            Self::Connection(e) => write!(f, "Connection error: {e}"),
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Shutdown(e) => write!(f, "Shutdown error: {e}"),
        }
    }
}

impl std::error::Error for ProxyError {}

impl From<std::io::Error> for ProxyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
