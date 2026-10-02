use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::rules::active::xss::techniques::{raw_reflection, XssFinding, XssTechnique};

pub struct ReflectedXss;

impl XssTechnique for ReflectedXss {
    fn name(&self) -> &'static str { "Reflected XSS" }
    fn description(&self) -> &'static str { "Detects reflected cross-site scripting by injecting payloads and checking for reflection" }
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
        let tag_injection = payload.contains('<') || payload.contains('>');
        let confidence = if tag_injection { RuleConfidence::High } else { RuleConfidence::Medium };
        Some(XssFinding {
            technique: self.name().to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence,
            evidence: format!(
                "The injected value was reflected verbatim (unescaped) in the response body.\nURL: {}\nPayload: {}\nContext: {}\nStatus: Potential - an unescaped reflection is a strong XSS candidate, but script execution was not independently verified. Manual confirmation is required before treating this as confirmed.",
                request.url, payload, refl.context
            ),
        })
    }
}
