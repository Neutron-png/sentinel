#![allow(dead_code)]

#[derive(Debug)]
pub enum ProxyIntegrationError {
    Attach(String),
    Detach(String),
    Route(String),
    Config(String),
    Session(String),
    Tls(String),
}
impl std::fmt::Display for ProxyIntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attach(e) => write!(f, "Attach: {e}"),
            Self::Detach(e) => write!(f, "Detach: {e}"),
            Self::Route(e) => write!(f, "Route: {e}"),
            Self::Config(e) => write!(f, "Config: {e}"),
            Self::Session(e) => write!(f, "Session: {e}"),
            Self::Tls(e) => write!(f, "TLS: {e}"),
        }
    }
}
impl std::error::Error for ProxyIntegrationError {}
