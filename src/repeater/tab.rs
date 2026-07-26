#![allow(dead_code)]

use uuid::Uuid;

use crate::network::models::HttpRequest;
use crate::repeater::errors::RepeaterError;
use crate::repeater::models::RepeaterTab;

pub struct TabManager {
    tabs: Vec<RepeaterTab>,
    max_tabs: usize,
    active_index: usize,
}

impl TabManager {
    pub fn new(max_tabs: usize) -> Self {
        Self {
            tabs: Vec::new(),
            max_tabs,
            active_index: 0,
        }
    }

    pub fn create(
        &mut self,
        name: &str,
        request: HttpRequest,
    ) -> Result<&RepeaterTab, RepeaterError> {
        if self.tabs.len() >= self.max_tabs {
            return Err(RepeaterError::TabLimit("Max tabs reached".into()));
        }
        let tab = RepeaterTab::new(name, request);
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        Ok(self.tabs.last().unwrap())
    }

    pub fn duplicate(&mut self, id: Uuid) -> Result<&RepeaterTab, RepeaterError> {
        let original = self.find(id)?.clone();
        let name = format!("{} (copy)", original.name);
        self.create(&name, original.request)
    }

    pub fn close(&mut self, id: Uuid) -> Result<(), RepeaterError> {
        let pos = self
            .tabs
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| RepeaterError::NotFound(id.to_string()))?;
        self.tabs.remove(pos);
        if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len().saturating_sub(1);
        }
        Ok(())
    }

    pub fn find(&self, id: Uuid) -> Result<&RepeaterTab, RepeaterError> {
        self.tabs
            .iter()
            .find(|t| t.id == id)
            .ok_or_else(|| RepeaterError::NotFound(id.to_string()))
    }

    pub fn find_mut(&mut self, id: Uuid) -> Result<&mut RepeaterTab, RepeaterError> {
        self.tabs
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| RepeaterError::NotFound(id.to_string()))
    }

    pub fn tabs(&self) -> &[RepeaterTab] {
        &self.tabs
    }
    pub fn active_index(&self) -> usize {
        self.active_index
    }
    pub fn set_active(&mut self, id: Uuid) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            self.active_index = pos;
        }
    }
    pub fn active_tab(&self) -> Option<&RepeaterTab> {
        self.tabs.get(self.active_index)
    }
    pub fn active_tab_mut(&mut self) -> Option<&mut RepeaterTab> {
        self.tabs.get_mut(self.active_index)
    }
}
