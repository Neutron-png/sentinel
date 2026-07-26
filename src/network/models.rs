#![allow(dead_code)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct HttpCookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub expires: Option<String>,
}

#[derive(Debug, Clone)]
pub enum HttpBody {
    Empty,
    Text(String),
    Bytes(Vec<u8>),
    Json(serde_json::Value),
}

impl HttpBody {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Bytes(b) => b,
            _ => &[],
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Text(s) => s.len(),
            Self::Bytes(b) => b.len(),
            Self::Json(v) => v.to_string().len(),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestTiming {
    pub started_at: Instant,
    pub dns_duration: Duration,
    pub connect_duration: Duration,
    pub tls_duration: Duration,
    pub send_duration: Duration,
    pub wait_duration: Duration,
    pub total_duration: Duration,
}
impl Default for RequestTiming {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            dns_duration: Duration::ZERO,
            connect_duration: Duration::ZERO,
            tls_duration: Duration::ZERO,
            send_duration: Duration::ZERO,
            wait_duration: Duration::ZERO,
            total_duration: Duration::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<HttpHeader>,
    pub cookies: Vec<HttpCookie>,
    pub body: HttpBody,
    pub protocol: String,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub timing: RequestTiming,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub cookies: Vec<HttpCookie>,
    pub body: HttpBody,
    pub query_params: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct HttpTransaction {
    pub request: HttpRequest,
    pub response: HttpResponse,
}
impl HttpTransaction {
    pub fn status_code(&self) -> u16 {
        self.response.status_code
    }
    pub fn duration(&self) -> Duration {
        self.response.timing.total_duration
    }
    pub fn response_body(&self) -> &HttpBody {
        &self.response.body
    }
}

pub fn parse_cookies_from_header(header_value: &str) -> Vec<HttpCookie> {
    header_value
        .split(';')
        .filter_map(|part| {
            let part = part.trim();
            let mut kv = part.splitn(2, '=');
            let name = kv.next()?.trim().to_string();
            let value = kv.next().unwrap_or("").trim().to_string();
            Some(HttpCookie {
                name,
                value,
                domain: None,
                path: None,
                secure: false,
                http_only: false,
                expires: None,
            })
        })
        .collect()
}

pub fn headers_to_map(headers: &[HttpHeader]) -> HashMap<String, String> {
    headers
        .iter()
        .map(|h| (h.name.clone(), h.value.clone()))
        .collect()
}
