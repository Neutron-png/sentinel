use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::rules::active::xss::techniques::{raw_reflection, XssFinding, XssTechnique};

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
        let refl = raw_reflection(analyzer, response, payload)?;
        Some(XssFinding {
            technique: self.name().to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence: RuleConfidence::Low,
            evidence: format!(
                "The injected value was reflected verbatim (unescaped) in the immediate response.\nURL: {}\nPayload: {}\nContext: {}\nStatus: Potential - this single response does not establish persistence. Stored XSS requires a separate request confirming the value is served unescaped from storage; manual verification is required.",
                request.url, payload, refl.context
            ),
        })
    }
}
