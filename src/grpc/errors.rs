use std::fmt;

#[derive(Debug)]
pub enum GrpcError {
    Detection(String),
    Parse(String),
    Reflection(String),
    Proto(String),
    Message(String),
    Analysis(String),
    Integration(String),
    Http(String),
}

impl fmt::Display for GrpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrpcError::Detection(m) => write!(f, "gRPC detection error: {}", m),
            GrpcError::Parse(m) => write!(f, "gRPC parse error: {}", m),
            GrpcError::Reflection(m) => write!(f, "gRPC reflection error: {}", m),
            GrpcError::Proto(m) => write!(f, "gRPC proto error: {}", m),
            GrpcError::Message(m) => write!(f, "gRPC message error: {}", m),
            GrpcError::Analysis(m) => write!(f, "gRPC analysis error: {}", m),
            GrpcError::Integration(m) => write!(f, "gRPC integration error: {}", m),
            GrpcError::Http(m) => write!(f, "gRPC HTTP error: {}", m),
        }
    }
}

impl std::error::Error for GrpcError {}

impl From<serde_json::Error> for GrpcError {
    fn from(e: serde_json::Error) -> Self {
        GrpcError::Parse(e.to_string())
    }
}
