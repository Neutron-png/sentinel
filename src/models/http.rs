use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub id: Uuid,
    pub method: String,
    pub url: String,
    pub headers: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl HttpRequest {
    pub fn format(&self) -> String {
        let mut s = format!("{} {} HTTP/1.1\n", self.method, self.url);
        if !self.headers.is_empty() {
            s.push_str(&self.headers);
            if !self.headers.ends_with('\n') {
                s.push('\n');
            }
        }
        s.push('\n');
        s.push_str(&self.body);
        s
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub id: Uuid,
    pub status_code: i32,
    pub headers: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl HttpResponse {
    pub fn format(&self) -> String {
        let status_text = status_text(self.status_code);
        let mut s = format!("HTTP/1.1 {} {}\n", self.status_code, status_text);
        if !self.headers.is_empty() {
            s.push_str(&self.headers);
            if !self.headers.ends_with('\n') {
                s.push('\n');
            }
        }
        s.push('\n');
        s.push_str(&self.body);
        s
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpTransaction {
    pub id: Uuid,
    pub task_id: Uuid,
    pub request_id: Uuid,
    pub response_id: Uuid,
    pub duration_ms: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[allow(dead_code)]
impl HttpMethod {
    pub const ALL: &[HttpMethod] = &[
        Self::Get,
        Self::Post,
        Self::Put,
        Self::Delete,
        Self::Patch,
        Self::Head,
        Self::Options,
    ];
    pub fn label(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }
    pub fn next(self) -> Self {
        let all = Self::ALL;
        let p = all.iter().position(|e| *e == self).unwrap_or(0);
        all[(p + 1) % all.len()]
    }
    pub fn prev(self) -> Self {
        let all = Self::ALL;
        let p = all.iter().position(|e| *e == self).unwrap_or(0);
        all[(p + all.len() - 1) % all.len()]
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

fn status_text(code: i32) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "",
    }
}
