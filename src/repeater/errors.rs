#![allow(dead_code)]

#[derive(Debug)]
pub enum RepeaterError {
    NotFound(String),
    TabLimit(String),
    Send(String),
    Edit(String),
}

impl std::fmt::Display for RepeaterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "Not found: {e}"),
            Self::TabLimit(e) => write!(f, "Tab limit: {e}"),
            Self::Send(e) => write!(f, "Send error: {e}"),
            Self::Edit(e) => write!(f, "Edit error: {e}"),
        }
    }
}
impl std::error::Error for RepeaterError {}
