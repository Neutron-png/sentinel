#![allow(dead_code)]

use crate::network::models::HttpResponse;
use crate::scanner::analyzer::models::{ReflectionResult, ReflectionType};

pub fn detect_reflection(response: &HttpResponse, payload: &str) -> ReflectionResult {
    let body = response.body.as_text().unwrap_or("");
    if body.is_empty() || payload.is_empty() {
        return ReflectionResult {
            found: false,
            reflection_type: ReflectionType::None,
            offset: 0,
            length: 0,
            payload: payload.to_string(),
            context: String::new(),
        };
    }

    // Full reflection
    if let Some(offset) = body.find(payload) {
        let start = offset.saturating_sub(20);
        let end = (offset + payload.len() + 20).min(body.len());
        return ReflectionResult {
            found: true,
            reflection_type: ReflectionType::Full,
            offset,
            length: payload.len(),
            payload: payload.to_string(),
            context: body[start..end].to_string(),
        };
    }

    // Partial (first 3+ chars)
    if payload.len() >= 4 {
        let prefix = &payload[..payload.len().min(4)];
        if let Some(offset) = body.find(prefix) {
            let start = offset.saturating_sub(20);
            let end = (offset + prefix.len() + 20).min(body.len());
            return ReflectionResult {
                found: true,
                reflection_type: ReflectionType::Partial,
                offset,
                length: prefix.len(),
                payload: payload.to_string(),
                context: body[start..end].to_string(),
            };
        }
    }

    // HTML-encoded
    let html_encoded = payload
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    if html_encoded != payload && body.contains(&html_encoded) {
        return ReflectionResult {
            found: true,
            reflection_type: ReflectionType::Html,
            offset: 0,
            length: html_encoded.len(),
            payload: payload.to_string(),
            context: html_encoded,
        };
    }

    ReflectionResult {
        found: false,
        reflection_type: ReflectionType::None,
        offset: 0,
        length: 0,
        payload: payload.to_string(),
        context: String::new(),
    }
}
