#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::backends::wry::WryBackend;
use crate::browser::backends::BrowserBackend;
use crate::browser::errors::BrowserError;
use crate::browser::events::{BrowserEvent, BrowserEventBus};
use crate::browser::session::BrowserSession;

pub struct BrowserManager {
    sessions: Vec<BrowserSession>,
    event_bus: BrowserEventBus,
    backend: WryBackend,
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            event_bus: BrowserEventBus::new(256),
            backend: WryBackend::new(),
        }
    }

    pub fn event_bus(&self) -> BrowserEventBus {
        self.event_bus.clone()
    }

    pub fn create_session(&mut self) -> Result<&BrowserSession, BrowserError> {
        let id = self.backend.create()?;
        let session = BrowserSession::new();
        self.sessions.push(session);
        self.event_bus.emit(BrowserEvent::BrowserCreated { id });
        Ok(self.sessions.last().unwrap())
    }

    pub fn close_session(&mut self, id: Uuid) -> Result<(), BrowserError> {
        self.backend.destroy(id)?;
        self.sessions.retain(|s| s.id != id);
        self.event_bus.emit(BrowserEvent::BrowserClosed { id });
        Ok(())
    }

    pub fn find_session(&self, id: Uuid) -> Option<&BrowserSession> {
        self.sessions.iter().find(|s| s.id == id)
    }
    pub fn find_session_mut(&mut self, id: Uuid) -> Option<&mut BrowserSession> {
        self.sessions.iter_mut().find(|s| s.id == id)
    }

    pub fn navigate(
        &mut self,
        session_id: Uuid,
        tab_id: Uuid,
        url: &str,
    ) -> Result<(), BrowserError> {
        self.backend.navigate(session_id, tab_id, url)?;
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == session_id) {
            if let Some(pos) = s.tabs.iter().position(|t| t.id == tab_id) {
                s.active_tab_index = pos;
                s.tabs[pos].url = url.to_string();
                s.tabs[pos].loading_state = super::tab::LoadingState::Loading;
            }
        }
        self.event_bus.emit(BrowserEvent::NavigationStarted {
            browser_id: session_id,
            tab_id,
            url: url.to_string(),
        });
        Ok(())
    }

    pub fn reload(&mut self, session_id: Uuid, tab_id: Uuid) -> Result<(), BrowserError> {
        self.backend.reload(session_id, tab_id)?;
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == session_id) {
            if let Some(tab) = s.tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.loading_state = super::tab::LoadingState::Loading;
            }
        }
        Ok(())
    }

    pub fn stop(&mut self, session_id: Uuid, tab_id: Uuid) -> Result<(), BrowserError> {
        self.backend.stop(session_id, tab_id)?;
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == session_id) {
            if let Some(tab) = s.tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.loading_state = super::tab::LoadingState::Idle;
            }
        }
        Ok(())
    }

    pub fn go_back(&mut self, session_id: Uuid, tab_id: Uuid) -> Result<(), BrowserError> {
        self.backend.go_back(session_id, tab_id)?;
        Ok(())
    }

    pub fn go_forward(&mut self, session_id: Uuid, tab_id: Uuid) -> Result<(), BrowserError> {
        self.backend.go_forward(session_id, tab_id)?;
        Ok(())
    }
}
