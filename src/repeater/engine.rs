#![allow(dead_code)]

use std::sync::Arc;
use uuid::Uuid;

use crate::network::client::HttpClient;
use crate::network::models::HttpRequest;
use crate::repeater::errors::RepeaterError;
use crate::repeater::executor::RequestExecutor;
use crate::repeater::history;
use crate::repeater::models::{RepeaterTab, ResponseEntry};
use crate::repeater::request_editor::{apply_edits, RequestEdit};
use crate::repeater::tab::TabManager;

pub struct RepeaterEngine {
    tabs: TabManager,
    executor: RequestExecutor,
}

impl RepeaterEngine {
    pub fn new(client: Arc<HttpClient>, max_tabs: usize) -> Self {
        Self {
            tabs: TabManager::new(max_tabs),
            executor: RequestExecutor::new(client),
        }
    }

    pub fn create_tab(
        &mut self,
        name: &str,
        request: HttpRequest,
    ) -> Result<&RepeaterTab, RepeaterError> {
        self.tabs.create(name, request)
    }

    pub fn duplicate_tab(&mut self, id: Uuid) -> Result<&RepeaterTab, RepeaterError> {
        self.tabs.duplicate(id)
    }

    pub fn close_tab(&mut self, id: Uuid) -> Result<(), RepeaterError> {
        self.tabs.close(id)
    }

    pub fn edit_request(&mut self, id: Uuid, edits: &[RequestEdit]) -> Result<(), RepeaterError> {
        let tab = self.tabs.find_mut(id)?;
        apply_edits(&mut tab.request, edits);
        Ok(())
    }

    pub async fn send(&mut self, id: Uuid) -> Result<ResponseEntry, RepeaterError> {
        let tab = self.tabs.find_mut(id)?;
        let entry = self.executor.send_request(tab).await?;
        Ok(entry)
    }

    pub async fn resend(&mut self, id: Uuid) -> Result<ResponseEntry, RepeaterError> {
        self.send(id).await
    }

    pub async fn send_multiple(
        &mut self,
        id: Uuid,
        count: usize,
    ) -> Result<Vec<ResponseEntry>, RepeaterError> {
        let tab = self.tabs.find_mut(id)?;
        self.executor.send_multiple(tab, count).await
    }

    pub fn cancel(&self, id: Uuid) -> Result<(), RepeaterError> {
        let tab = self.tabs.find(id)?;
        // Async cancellation is a no-op for now
        let _ = tab;
        Ok(())
    }

    pub fn previous_response(&mut self, id: Uuid) -> Option<&ResponseEntry> {
        self.tabs.find_mut(id).ok()?.previous_response()
    }

    pub fn next_response(&mut self, id: Uuid) -> Option<&ResponseEntry> {
        self.tabs.find_mut(id).ok()?.next_response()
    }

    pub fn clear_history(&mut self, id: Uuid) -> Result<(), RepeaterError> {
        let tab = self.tabs.find_mut(id)?;
        history::clear_history(tab);
        Ok(())
    }

    pub fn tabs(&self) -> &[RepeaterTab] {
        self.tabs.tabs()
    }
    pub fn active_tab(&self) -> Option<&RepeaterTab> {
        self.tabs.active_tab()
    }
    pub fn set_active_tab(&mut self, id: Uuid) {
        self.tabs.set_active(id);
    }
}
