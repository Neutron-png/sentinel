#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Payload {
    pub value: String,
    pub original: String,
    pub encoding: Option<String>,
    pub mutation: Option<String>,
}

impl Payload {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            original: value.to_string(),
            encoding: None,
            mutation: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PayloadSet {
    pub name: String,
    pub description: String,
    pub payloads: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncoderType {
    Url,
    DoubleUrl,
    Html,
    Base64,
    Hex,
    Unicode,
    JsonEscape,
    XmlEscape,
    None,
}

impl EncoderType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Url => "URL",
            Self::DoubleUrl => "Double URL",
            Self::Html => "HTML",
            Self::Base64 => "Base64",
            Self::Hex => "Hex",
            Self::Unicode => "Unicode",
            Self::JsonEscape => "JSON",
            Self::XmlEscape => "XML",
            Self::None => "None",
        }
    }
    pub fn all() -> Vec<EncoderType> {
        vec![
            Self::Url,
            Self::DoubleUrl,
            Self::Html,
            Self::Base64,
            Self::Hex,
            Self::Unicode,
            Self::JsonEscape,
            Self::XmlEscape,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationType {
    Prefix(String),
    Suffix(String),
    Replace(String, String),
    Wrap(String, String),
    Duplicate,
    RandomCase,
    RandomPadding(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertionPoint {
    QueryParam(String),
    PathParam(String),
    Header(String),
    Cookie(String),
    JsonValue(String),
    XmlValue(String),
    FormField(String),
    MultipartField(String),
    RawBody,
}

impl InsertionPoint {
    pub fn label(&self) -> &'static str {
        match self {
            Self::QueryParam(_) => "Query",
            Self::PathParam(_) => "Path",
            Self::Header(_) => "Header",
            Self::Cookie(_) => "Cookie",
            Self::JsonValue(_) => "JSON",
            Self::XmlValue(_) => "XML",
            Self::FormField(_) => "Form",
            Self::MultipartField(_) => "Multipart",
            Self::RawBody => "Raw Body",
        }
    }
}
