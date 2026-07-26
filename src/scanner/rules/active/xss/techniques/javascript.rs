use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

pub struct JavaScriptXss;

impl XssTechnique for JavaScriptXss {
    fn name(&self) -> &'static str { "JavaScript Context XSS" }
    fn description(&self) -> &'static str { "Detects XSS in JavaScript context by breaking out of string literals" }
    fn payload_category(&self) -> &'static str { "XSS" }

    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
        let js_payloads = vec!["';alert(1)//", "\";alert(1)//", "</script><script>alert(1)</script>", "\\'-alert(1)//"];
        let mut results = Vec::new();
        for p in &js_payloads {
            for point in points {
                results.push((p.to_string(), engine.insert_into(request, point, p)));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let body = response.body.as_text().unwrap_or("");
        let has_js_escape = body.contains("alert(1)") || body.contains("alert(1)") || analyzer.detect_reflection(response, payload).found;
        if has_js_escape {
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::High,
                evidence: format!("JavaScript context XSS detected.\nURL: {}\nPayload: {}", request.url, payload),
            })
        } else { None }
    }
}
