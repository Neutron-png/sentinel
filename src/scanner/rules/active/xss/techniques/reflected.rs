use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

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
        let refl = analyzer.detect_reflection(response, payload);
        if refl.found {
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::High,
                evidence: format!("Reflected XSS detected.\nURL: {}\nPayload: {}\nReflection: {:?}\nContext: {}",
                    request.url, payload, refl.reflection_type, refl.context),
            })
        } else { None }
    }
}
