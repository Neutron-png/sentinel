#![allow(dead_code)]

use crate::har::builder::HarBuilder;
use crate::har::errors::HarError;
use crate::har::events::{HarEvent, HarEventBus};
use crate::har::models::HarLog;
use crate::network::models::{HttpRequest, HttpResponse};

pub struct HarRecorder {
    log: HarLog,
    event_bus: HarEventBus,
    recording: bool,
}

impl HarRecorder {
    pub fn new() -> Self {
        Self {
            log: HarLog::new(),
            event_bus: HarEventBus::new(512),
            recording: false,
        }
    }

    pub fn event_bus(&self) -> HarEventBus {
        self.event_bus.clone()
    }

    pub fn start(&mut self) {
        self.recording = true;
        self.event_bus.emit(HarEvent::RecordingStarted);
    }

    pub fn stop(&mut self) {
        self.recording = false;
        self.event_bus.emit(HarEvent::RecordingStopped {
            entries: self.log.entries.len(),
        });
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn record(&mut self, request: &HttpRequest, response: &HttpResponse, timing_ms: f64) {
        if !self.recording {
            return;
        }
        let entry = HarBuilder::build_entry(request, response, timing_ms);
        self.event_bus.emit(HarEvent::EntryRecorded {
            url: request.url.clone(),
            method: request.method.clone(),
        });
        self.log.entries.push(entry);
    }

    pub fn add_page(&mut self, url: &str, title: &str, load_time_ms: f64) {
        let page = HarBuilder::build_page(url, title, load_time_ms);
        self.log.pages.push(page);
    }

    pub fn export(&self) -> Result<String, HarError> {
        serde_json::to_string_pretty(&self.log).map_err(|e| HarError::Export(e.to_string()))
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), HarError> {
        self.event_bus.emit(HarEvent::ExportStarted);
        let json = self.export()?;
        std::fs::write(path, json).map_err(|e| HarError::Export(e.to_string()))?;
        self.event_bus.emit(HarEvent::ExportFinished {
            path: path.to_string(),
        });
        Ok(())
    }

    pub fn clear(&mut self) {
        self.log = HarLog::new();
    }
    pub fn entry_count(&self) -> usize {
        self.log.entries.len()
    }
}
