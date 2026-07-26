use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

pub struct HtmlContextXss;

impl XssTechnique for HtmlContextXss {
    fn name(&self) -> &'static str { "HTML Context XSS" }
    fn description(&self) -> &'static str { "Detects XSS in raw HTML body context by injecting script tags" }
    fn payload_category(&self) -> &'static str { "XSS" }

    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
        let payloads = engine.generate("XSS");
        let html_payloads: Vec<&str> = payloads.iter().filter_map(|p| {
            if p.value.contains("<script") || p.value.contains("<img") { Some(p.value.as_str()) } else { None }
        }).collect();
        let mut results = Vec::new();
        for p in html_payloads {
            for point in points {
                results.push((p.to_string(), engine.insert_into(request, point, p)));
            }
        }
        if results.is_empty() {
            for point in points {
                results.push(("<script>alert(1)</script>".to_string(), engine.insert_into(request, point, "<script>alert(1)</script>")));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let body = response.body.as_text().unwrap_or("");
        if body.contains("<script>") || body.contains("alert(1)") || analyzer.detect_reflection(response, payload).found {
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::High,
                evidence: format!("HTML context XSS detected.\nURL: {}\nPayload: {}", request.url, payload),
            })
        } else { None }
    }
}
