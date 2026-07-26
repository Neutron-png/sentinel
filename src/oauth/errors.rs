use std::fmt;

#[derive(Debug)]
pub enum OAuthError {
    Detection(String),
    Flow(String),
    Oidc(String),
    Token(String),
    Pkce(String),
    Session(String),
    Analysis(String),
    Integration(String),
    Http(String),
    Json(String),
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthError::Detection(m) => write!(f, "OAuth detection error: {}", m),
            OAuthError::Flow(m) => write!(f, "OAuth flow error: {}", m),
            OAuthError::Oidc(m) => write!(f, "OIDC error: {}", m),
            OAuthError::Token(m) => write!(f, "OAuth token error: {}", m),
            OAuthError::Pkce(m) => write!(f, "PKCE error: {}", m),
            OAuthError::Session(m) => write!(f, "OAuth session error: {}", m),
            OAuthError::Analysis(m) => write!(f, "OAuth analysis error: {}", m),
            OAuthError::Integration(m) => write!(f, "OAuth integration error: {}", m),
            OAuthError::Http(m) => write!(f, "OAuth HTTP error: {}", m),
            OAuthError::Json(m) => write!(f, "OAuth JSON error: {}", m),
        }
    }
}

impl std::error::Error for OAuthError {}

impl From<serde_json::Error> for OAuthError {
    fn from(e: serde_json::Error) -> Self {
        OAuthError::Json(e.to_string())
    }
}
