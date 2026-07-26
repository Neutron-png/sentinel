#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::proxy::errors::ProxyIntegrationError;
use crate::browser::proxy::events::{BrowserProxyEvent, BrowserProxyEventBus};
use crate::browser::proxy::routing::{BrowserRoutingRules, RouteResult};
use crate::browser::proxy::session::BrowserProxySession;

pub struct BrowserProxyManager {
    sessions: Vec<BrowserProxySession>,
    routing: BrowserRoutingRules,
    event_bus: BrowserProxyEventBus,
    default_proxy_port: u16,
}

impl BrowserProxyManager {
    pub fn new(proxy_port: u16) -> Self {
        Self {
            sessions: Vec::new(),
            routing: BrowserRoutingRules::default(),
            event_bus: BrowserProxyEventBus::new(256),
            default_proxy_port: proxy_port,
        }
    }

    pub fn event_bus(&self) -> BrowserProxyEventBus {
        self.event_bus.clone()
    }

    pub fn attach_browser(
        &mut self,
        browser_id: Uuid,
    ) -> Result<&BrowserProxySession, ProxyIntegrationError> {
        if self.sessions.iter().any(|s| s.browser_id == browser_id) {
            return Err(ProxyIntegrationError::Attach("Already attached".into()));
        }
        let mut session = BrowserProxySession::new(browser_id, self.default_proxy_port);
        session.attached = true;
        self.sessions.push(session);
        self.event_bus
            .emit(BrowserProxyEvent::BrowserAttached { browser_id });
        self.event_bus.emit(BrowserProxyEvent::ProxyConnected {
            browser_id,
            proxy_port: self.default_proxy_port,
        });
        Ok(self.sessions.last().unwrap())
    }

    pub fn detach_browser(&mut self, browser_id: Uuid) -> Result<(), ProxyIntegrationError> {
        self.sessions.retain(|s| s.browser_id != browser_id);
        self.event_bus
            .emit(BrowserProxyEvent::BrowserDetached { browser_id });
        self.event_bus
            .emit(BrowserProxyEvent::ProxyDisconnected { browser_id });
        Ok(())
    }

    pub fn find_session(&self, browser_id: Uuid) -> Option<&BrowserProxySession> {
        self.sessions.iter().find(|s| s.browser_id == browser_id)
    }

    pub fn find_session_mut(&mut self, browser_id: Uuid) -> Option<&mut BrowserProxySession> {
        self.sessions
            .iter_mut()
            .find(|s| s.browser_id == browser_id)
    }

    pub fn route_request(
        &self,
        browser_id: Uuid,
        tab_id: Uuid,
        url: &str,
        method: &str,
    ) -> RouteResult {
        if let Some(session) = self.find_session(browser_id) {
            self.routing
                .route_through_pipeline(session, browser_id, tab_id, url, method)
        } else {
            RouteResult::Direct
        }
    }

    pub fn enable_scope(
        &mut self,
        browser_id: Uuid,
        enabled: bool,
    ) -> Result<(), ProxyIntegrationError> {
        let s = self
            .find_session_mut(browser_id)
            .ok_or_else(|| ProxyIntegrationError::Session("Not found".into()))?;
        s.scope_enabled = enabled;
        Ok(())
    }

    pub fn enable_history(
        &mut self,
        browser_id: Uuid,
        enabled: bool,
    ) -> Result<(), ProxyIntegrationError> {
        let s = self
            .find_session_mut(browser_id)
            .ok_or_else(|| ProxyIntegrationError::Session("Not found".into()))?;
        s.history_enabled = enabled;
        Ok(())
    }

    pub fn enable_tls_intercept(
        &mut self,
        browser_id: Uuid,
        enabled: bool,
    ) -> Result<(), ProxyIntegrationError> {
        let s = self
            .find_session_mut(browser_id)
            .ok_or_else(|| ProxyIntegrationError::Session("Not found".into()))?;
        s.tls_intercept = enabled;
        Ok(())
    }

    pub fn notify_request_started(&self, browser_id: Uuid, tab_id: Uuid, url: &str, method: &str) {
        self.event_bus
            .emit(BrowserProxyEvent::BrowserRequestStarted {
                browser_id,
                tab_id,
                url: url.to_string(),
                method: method.to_string(),
            });
    }

    pub fn notify_request_finished(&self, browser_id: Uuid, tab_id: Uuid, url: &str, status: u16) {
        self.event_bus
            .emit(BrowserProxyEvent::BrowserRequestFinished {
                browser_id,
                tab_id,
                url: url.to_string(),
                status,
            });
    }
}
