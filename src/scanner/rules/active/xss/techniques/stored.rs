use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

pub struct StoredXss;

impl XssTechnique for StoredXss {
    fn name(&self) -> &'static str { "Stored XSS (Framework)" }
    fn description(&self) -> &'static str { "Framework for detecting stored XSS - requires manual verification of persistence" }
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
        let refl = analyzer.detect_reflection(response, payload);
        if refl.found {
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Low,
                evidence: format!("Potential stored XSS payload submitted.\nURL: {}\nPayload: {}\nResponse reflection: {:?}",
                    request.url, payload, refl.reflection_type),
            })
        } else { None }
    }
}
