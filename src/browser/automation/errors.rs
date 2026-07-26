#![allow(dead_code)]

#[derive(Debug)]
pub enum AutomationError {
    Action(String),
    SelectorNotFound(String),
    Timeout(String),
    Navigation(String),
}
impl std::fmt::Display for AutomationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(e) => write!(f, "Action: {e}"),
            Self::SelectorNotFound(e) => write!(f, "Selector: {e}"),
            Self::Timeout(e) => write!(f, "Timeout: {e}"),
            Self::Navigation(e) => write!(f, "Navigation: {e}"),
        }
    }
}
impl std::error::Error for AutomationError {}
