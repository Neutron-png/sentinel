#![allow(dead_code)]

pub mod wry;

use uuid::Uuid;

use crate::browser::errors::BrowserError;

pub trait BrowserBackend: Send + Sync {
    fn create(&mut self) -> Result<Uuid, BrowserError>;
    fn destroy(&mut self, id: Uuid) -> Result<(), BrowserError>;
    fn navigate(&self, id: Uuid, tab_id: Uuid, url: &str) -> Result<(), BrowserError>;
    fn reload(&self, id: Uuid, tab_id: Uuid) -> Result<(), BrowserError>;
    fn stop(&self, id: Uuid, tab_id: Uuid) -> Result<(), BrowserError>;
    fn go_back(&self, id: Uuid, tab_id: Uuid) -> Result<(), BrowserError>;
    fn go_forward(&self, id: Uuid, tab_id: Uuid) -> Result<(), BrowserError>;
}
