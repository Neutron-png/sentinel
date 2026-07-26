use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

pub struct DomXss;

impl XssTechnique for DomXss {
    fn name(&self) -> &'static str { "DOM XSS Candidate" }
    fn description(&self) -> &'static str { "Detects potential DOM XSS by identifying JavaScript sinks in response body" }
    fn payload_category(&self) -> &'static str { "XSS" }

    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
        let payloads = engine.generate("XSS");
        let mut results = Vec::new();
        for p in &payloads {
            for point in points {
                results.push((p.value.clone(), engine.insert_into(request, point, &p.value)));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let body = response.body.as_text().unwrap_or("");
        let sinks = ["innerHTML", "document.write", "eval(", "setTimeout", "setInterval"];
        let found_sinks: Vec<&str> = sinks.iter().filter(|s| body.to_lowercase().contains(&s.to_lowercase())).copied().collect();
        if !found_sinks.is_empty() {
            let refl = analyzer.detect_reflection(response, payload);
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: if refl.found { crate::scanner::sdk::result::RuleConfidence::Medium } else { crate::scanner::sdk::result::RuleConfidence::Low },
                evidence: format!("DOM XSS sinks found: {}\nURL: {}\nPayload: {}", found_sinks.join(", "), request.url, payload),
            })
        } else { None }
    }
}
