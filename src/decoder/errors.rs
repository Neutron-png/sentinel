use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DecoderError {
    InvalidBase64(String),
    InvalidHex(String),
    InvalidUrl(String),
    InvalidUnicode(String),
    InvalidJson(String),
    InvalidJwt(String),
    NotEmbedded(String),
    Utf8(String),
    UnknownTransform(String),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase64(m) => write!(f, "invalid base64 input: {m}"),
            Self::InvalidHex(m) => write!(f, "invalid hex input: {m}"),
            Self::InvalidUrl(m) => write!(f, "invalid percent-encoding: {m}"),
            Self::InvalidUnicode(m) => write!(f, "invalid unicode escape: {m}"),
            Self::InvalidJson(m) => write!(f, "invalid json: {m}"),
            Self::InvalidJwt(m) => write!(f, "invalid jwt: {m}"),
            Self::NotEmbedded(m) => write!(f, "transform is not reversible: {m}"),
            Self::Utf8(m) => write!(f, "decoded bytes are not valid utf-8: {m}"),
            Self::UnknownTransform(m) => write!(f, "unknown transform: {m}"),
        }
    }
}

impl std::error::Error for DecoderError {}
