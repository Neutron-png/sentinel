#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::backends::BrowserBackend;
use crate::browser::errors::BrowserError;

pub struct WryBackend;

impl WryBackend {
    pub fn new() -> Self {
        Self
    }
}

impl BrowserBackend for WryBackend {
    fn create(&mut self) -> Result<Uuid, BrowserError> {
        Ok(Uuid::new_v4())
    }

    fn destroy(&mut self, _id: Uuid) -> Result<(), BrowserError> {
        Ok(())
    }

    fn navigate(&self, _id: Uuid, _tab_id: Uuid, _url: &str) -> Result<(), BrowserError> {
        Ok(())
    }

    fn reload(&self, _id: Uuid, _tab_id: Uuid) -> Result<(), BrowserError> {
        Ok(())
    }

    fn stop(&self, _id: Uuid, _tab_id: Uuid) -> Result<(), BrowserError> {
        Ok(())
    }

    fn go_back(&self, _id: Uuid, _tab_id: Uuid) -> Result<(), BrowserError> {
        Ok(())
    }

    fn go_forward(&self, _id: Uuid, _tab_id: Uuid) -> Result<(), BrowserError> {
        Ok(())
    }
}
