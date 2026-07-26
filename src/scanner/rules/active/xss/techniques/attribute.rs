use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active\xss\techniques::{XssFinding, XssTechnique};

pub struct AttributeXss;

impl XssTechnique for AttributeXss {
    fn name(&self) -> &'static str { "Attribute Context XSS" }
    fn description(&self) -> &'static str { "Detects XSS in HTML attribute context by testing attribute-breaking payloads" }
    fn payload_category(&self) -> &'static str { "XSS" }

    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
        let attr_payloads = vec!["\" onmouseover=alert(1)", "' onfocus=alert(1) autofocus", "\"><script>alert(1)</script>"];
        let mut results = Vec::new();
        for p in &attr_payloads {
            for point in points {
                let mut req = engine.insert_into(request, point, p);
                results.push((p.to_string(), req));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let body = response.body.as_text().unwrap_or("");
        let indicators = ["onmouseover", "onfocus", "autofocus"];
        let found = indicators.iter().any(|i| body.contains(i)) || analyzer.detect_reflection(response, payload).found;
        if found {
            Some(XssFinding {
                technique: self.name().to_string(),
                injection_point: request.url.clone(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Medium,
                evidence: format!("Attribute context XSS detected.\nURL: {}\nPayload: {}", request.url, payload),
            })
        } else { None }
    }
}
