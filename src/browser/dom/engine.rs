#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::dom::document::DocumentApi;
use crate::browser::dom::element::ElementApi;
use crate::browser::dom::events::{DomEvent, DomEventBus};
use crate::browser::dom::forms::FormApi;
use crate::browser::dom::javascript::JavaScriptApi;
use crate::browser::dom::links::LinkApi;
use crate::browser::dom::models;
use crate::browser::dom::resources::ResourceApi;
use crate::browser::dom::storage::StorageApi;

pub struct DomEngine {
    event_bus: DomEventBus,
}

impl DomEngine {
    pub fn new() -> Self {
        Self {
            event_bus: DomEventBus::new(256),
        }
    }
    pub fn event_bus(&self) -> DomEventBus {
        self.event_bus.clone()
    }

    // Document
    pub fn get_title(&self) -> String {
        DocumentApi::get_title()
    }
    pub fn get_url(&self) -> String {
        DocumentApi::get_url()
    }
    pub fn get_html(&self) -> String {
        DocumentApi::get_html()
    }

    // Element
    pub fn query_selector(&self, s: &str) -> Option<models::DomElement> {
        ElementApi::query_selector(s)
    }
    pub fn query_selector_all(&self, s: &str) -> Vec<models::DomElement> {
        ElementApi::query_selector_all(s)
    }
    pub fn get_element_by_id(&self, id: &str) -> Option<models::DomElement> {
        ElementApi::get_element_by_id(id)
    }

    // Forms
    pub fn discover_forms(&self) -> Vec<models::FormInfo> {
        FormApi::discover_forms()
    }
    pub fn get_inputs(&self) -> Vec<models::FormField> {
        FormApi::get_inputs()
    }
    pub fn set_input_value(&self, sel: &str, val: &str) {
        FormApi::set_input_value(sel, val)
    }

    // Links
    pub fn extract_links(&self) -> Vec<models::LinkInfo> {
        LinkApi::extract_links()
    }
    pub fn extract_scripts(&self) -> Vec<models::LinkInfo> {
        LinkApi::extract_scripts()
    }

    // Resources
    pub fn get_all_resources(&self) -> Vec<models::ResourceInfo> {
        ResourceApi::get_all_resources()
    }

    // Storage
    pub fn get_cookies(&self) -> Vec<models::CookieInfo> {
        StorageApi::get_cookies()
    }
    pub fn get_local_storage(&self) -> Vec<models::StorageEntry> {
        StorageApi::get_local_storage()
    }

    // JavaScript
    pub fn evaluate_js(&self, script: &str) -> Result<serde_json::Value, super::errors::DomError> {
        JavaScriptApi::evaluate(script)
    }

    // Events
    pub fn emit_dom_ready(&self, browser_id: Uuid, tab_id: Uuid) {
        self.event_bus
            .emit(DomEvent::DomReady { browser_id, tab_id });
    }
}
