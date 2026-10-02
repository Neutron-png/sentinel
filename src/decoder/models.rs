#![allow(dead_code)]

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    UrlEncode,
    UrlDecode,
    UrlDecodeAll,
    Base64Encode,
    Base64UrlEncode,
    Base64Decode,
    Base64UrlDecode,
    HexEncode,
    HexDecode,
    HtmlEncode,
    HtmlDecode,
    UnicodeEncode,
    UnicodeDecode,
    JsonFormat,
    JsonMinify,
    JwtDecode,
    ToUpper,
    ToLower,
    Reverse,
}

impl Transform {
    pub fn label(&self) -> &'static str {
        match self {
            Self::UrlEncode => "URL encode",
            Self::UrlDecode => "URL decode",
            Self::UrlDecodeAll => "URL decode (recursive)",
            Self::Base64Encode => "Base64 encode",
            Self::Base64UrlEncode => "Base64url encode",
            Self::Base64Decode => "Base64 decode",
            Self::Base64UrlDecode => "Base64url decode",
            Self::HexEncode => "Hex encode",
            Self::HexDecode => "Hex decode",
            Self::HtmlEncode => "HTML entity encode",
            Self::HtmlDecode => "HTML entity decode",
            Self::UnicodeEncode => "Unicode escape encode",
            Self::UnicodeDecode => "Unicode escape decode",
            Self::JsonFormat => "JSON format",
            Self::JsonMinify => "JSON minify",
            Self::JwtDecode => "JWT decode",
            Self::ToUpper => "Uppercase",
            Self::ToLower => "Lowercase",
            Self::Reverse => "Reverse",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HashCandidate {
    pub algorithm: String,
    pub confidence: u8,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JwtParts {
    pub header: Value,
    pub payload: Value,
    pub signature_present: bool,
}
