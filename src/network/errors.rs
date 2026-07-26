#[allow(dead_code)]
#[derive(Debug)]
pub enum NetworkError {
    Build(String),
    Request(String),
    Timeout(String),
    Tls(String),
    Connection(String),
    TooManyRedirects(String),
    InvalidUrl(String),
    BodyRead(String),
    Other(String),
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Build(e) => write!(f, "Build error: {e}"),
            Self::Request(e) => write!(f, "Request error: {e}"),
            Self::Timeout(e) => write!(f, "Timeout: {e}"),
            Self::Tls(e) => write!(f, "TLS error: {e}"),
            Self::Connection(e) => write!(f, "Connection error: {e}"),
            Self::TooManyRedirects(e) => write!(f, "Too many redirects: {e}"),
            Self::InvalidUrl(e) => write!(f, "Invalid URL: {e}"),
            Self::BodyRead(e) => write!(f, "Body read error: {e}"),
            Self::Other(e) => write!(f, "Network error: {e}"),
        }
    }
}

impl std::error::Error for NetworkError {}

impl From<reqwest::Error> for NetworkError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Self::Timeout(e.to_string())
        } else if e.is_connect() {
            Self::Connection(e.to_string())
        } else if e.is_builder() {
            Self::Build(e.to_string())
        } else if e.is_request() {
            Self::Request(e.to_string())
        } else {
            Self::Other(e.to_string())
        }
    }
}
