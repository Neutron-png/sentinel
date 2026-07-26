#![allow(dead_code)]

#[derive(Debug)]
pub enum BrowserError {
    Create(String),
    Navigation(String),
    Tab(String),
    Backend(String),
}
impl std::fmt::Display for BrowserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create(e) => write!(f, "Create: {e}"),
            Self::Navigation(e) => write!(f, "Navigation: {e}"),
            Self::Tab(e) => write!(f, "Tab: {e}"),
            Self::Backend(e) => write!(f, "Backend: {e}"),
        }
    }
}
impl std::error::Error for BrowserError {}
