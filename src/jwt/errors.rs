use std::fmt;

#[derive(Debug)]
pub enum JwtError {
    Parse(String),
    Base64(String),
    Json(String),
    Validation(String),
    Detection(String),
    Editing(String),
    NotFound(String),
    UnsupportedAlgorithm(String),
    MalformedToken(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::Parse(msg) => write!(f, "JWT parse error: {}", msg),
            JwtError::Base64(msg) => write!(f, "JWT base64 error: {}", msg),
            JwtError::Json(msg) => write!(f, "JWT JSON error: {}", msg),
            JwtError::Validation(msg) => write!(f, "JWT validation error: {}", msg),
            JwtError::Detection(msg) => write!(f, "JWT detection error: {}", msg),
            JwtError::Editing(msg) => write!(f, "JWT editing error: {}", msg),
            JwtError::NotFound(msg) => write!(f, "JWT not found: {}", msg),
            JwtError::UnsupportedAlgorithm(msg) => write!(f, "Unsupported JWT algorithm: {}", msg),
            JwtError::MalformedToken(msg) => write!(f, "Malformed JWT token: {}", msg),
        }
    }
}

impl std::error::Error for JwtError {}

impl From<serde_json::Error> for JwtError {
    fn from(e: serde_json::Error) -> Self {
        JwtError::Json(e.to_string())
    }
}
