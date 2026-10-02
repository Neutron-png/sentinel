use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::rules::active::xss::techniques::{raw_reflection, XssFinding, XssTechnique};

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
        let refl = raw_reflection(analyzer, response, payload)?;
        let body = response.body.as_text().unwrap_or("").to_lowercase();
        let sinks = ["innerhtml", "document.write", "eval(", "settimeout", "setinterval"];
        let found: Vec<&str> = sinks.iter().filter(|s| body.contains(**s)).copied().collect();
        if found.is_empty() {
            return None;
        }
        Some(XssFinding {
            technique: self.name().to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence: RuleConfidence::Low,
            evidence: format!(
                "An unescaped reflection was observed and the page contains JavaScript sink(s): {}.\nURL: {}\nPayload: {}\nContext: {}\nStatus: Potential - DOM XSS requires client-side execution and the data-flow into the sink was not verified; this is a candidate only.",
                found.join(", "), request.url, payload, refl.context
            ),
        })
    }
}
