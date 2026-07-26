#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingState {
    Idle,
    Loading,
    Loaded,
    Failed,
}

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub loading_state: LoadingState,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

impl BrowserTab {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            url: "about:blank".into(),
            loading_state: LoadingState::Idle,
            can_go_back: false,
            can_go_forward: false,
        }
    }
}
