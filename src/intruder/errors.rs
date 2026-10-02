use std::fmt;

#[derive(Debug)]
pub enum IntruderError {
    InvalidTemplate(String),
    NoPositions,
    PayloadCountExceeded { planned: u64, cap: u64 },
    InvalidPayloadSet(String),
    InvalidEncoderInput(String),
    Io(String),
    Client(String),
    Cancelled,
    NotRunning,
}

impl fmt::Display for IntruderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTemplate(m) => write!(f, "invalid request template: {m}"),
            Self::NoPositions => write!(f, "request template contains no payload positions"),
            Self::PayloadCountExceeded { planned, cap } => {
                write!(f, "payload plan has {planned} requests, exceeding the configured cap of {cap}")
            }
            Self::InvalidPayloadSet(m) => write!(f, "invalid payload set: {m}"),
            Self::InvalidEncoderInput(m) => write!(f, "cannot encode payload: {m}"),
            Self::Io(m) => write!(f, "payload source error: {m}"),
            Self::Client(m) => write!(f, "http client error: {m}"),
            Self::Cancelled => write!(f, "intruder run cancelled"),
            Self::NotRunning => write!(f, "no intruder run in progress"),
        }
    }
}

impl std::error::Error for IntruderError {}

impl From<crate::network::errors::NetworkError> for IntruderError {
    fn from(e: crate::network::errors::NetworkError) -> Self {
        Self::Client(e.to_string())
    }
}
