#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};

#[derive(Debug, Clone)]
pub struct ResponseEntry {
    pub id: Uuid,
    pub response: HttpResponse,
    pub sent_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub response_size: u64,
}

#[derive(Debug, Clone)]
pub struct RepeaterTab {
    pub id: Uuid,
    pub name: String,
    pub request: HttpRequest,
    pub latest_response: Option<ResponseEntry>,
    pub response_history: Vec<ResponseEntry>,
    pub history_index: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RepeaterTab {
    pub fn new(name: &str, request: HttpRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            request,
            latest_response: None,
            response_history: Vec::new(),
            history_index: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn has_previous(&self) -> bool {
        self.history_index > 0
    }
    pub fn has_next(&self) -> bool {
        self.history_index + 1 < self.response_history.len()
    }

    pub fn current_response(&self) -> Option<&ResponseEntry> {
        self.response_history.get(self.history_index)
    }

    pub fn previous_response(&mut self) -> Option<&ResponseEntry> {
        if self.has_previous() {
            self.history_index -= 1;
        }
        self.current_response()
    }

    pub fn next_response(&mut self) -> Option<&ResponseEntry> {
        if self.has_next() {
            self.history_index += 1;
        }
        self.current_response()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Raw,
    Json,
    Xml,
    FormUrlEncoded,
    MultipartForm,
    Binary,
}

impl BodyType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Raw => "Raw",
            Self::Json => "JSON",
            Self::Xml => "XML",
            Self::FormUrlEncoded => "Form URL Encoded",
            Self::MultipartForm => "Multipart Form",
            Self::Binary => "Binary",
        }
    }
}
