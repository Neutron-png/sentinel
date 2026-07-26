#![allow(dead_code)]

#[derive(Debug)]
pub enum PayloadError {
    Encode(String),
    Mutate(String),
    Insert(String),
    Generate(String),
}
impl std::fmt::Display for PayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(e) => write!(f, "Encode: {e}"),
            Self::Mutate(e) => write!(f, "Mutate: {e}"),
            Self::Insert(e) => write!(f, "Insert: {e}"),
            Self::Generate(e) => write!(f, "Generate: {e}"),
        }
    }
}
impl std::error::Error for PayloadError {}
