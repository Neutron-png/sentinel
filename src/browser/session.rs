#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::tab::BrowserTab;

#[derive(Debug, Clone)]
pub struct BrowserSession {
    pub id: Uuid,
    pub tabs: Vec<BrowserTab>,
    pub active_tab_index: usize,
    pub user_agent: String,
}

impl BrowserSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            tabs: vec![BrowserTab::new()],
            active_tab_index: 0,
            user_agent: format!("Sentinel/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn active_tab(&self) -> &BrowserTab {
        &self.tabs[self.active_tab_index]
    }
    pub fn active_tab_mut(&mut self) -> &mut BrowserTab {
        &mut self.tabs[self.active_tab_index]
    }

    pub fn create_tab(&mut self) -> Uuid {
        let tab = BrowserTab::new();
        let id = tab.id;
        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
        id
    }

    pub fn close_tab(&mut self, tab_id: Uuid) -> bool {
        if self.tabs.len() <= 1 {
            return false;
        }
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            self.tabs.remove(pos);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn switch_tab(&mut self, tab_id: Uuid) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            self.active_tab_index = pos;
        }
    }

    pub fn duplicate_tab(&mut self, tab_id: Uuid) -> Option<Uuid> {
        let target = self.tabs.iter().find(|t| t.id == tab_id)?;
        let mut dup = target.clone();
        dup.id = Uuid::new_v4();
        let id = dup.id;
        self.tabs.push(dup);
        self.active_tab_index = self.tabs.len() - 1;
        Some(id)
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}
