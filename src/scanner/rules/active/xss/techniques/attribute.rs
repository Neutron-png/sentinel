use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::rules::active::xss::techniques::{raw_reflection, XssFinding, XssTechnique};

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
                let req = engine.insert_into(request, point, p);
                results.push((p.to_string(), req));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let refl = raw_reflection(analyzer, response, payload)?;
        let attr_break = payload.contains('"') || payload.contains('\'') || payload.contains('>');
        if !attr_break {
            return None;
        }
        Some(XssFinding {
            technique: self.name().to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence: RuleConfidence::Medium,
            evidence: format!(
                "An attribute-context breakout payload was reflected verbatim (unescaped).\nURL: {}\nPayload: {}\nContext: {}\nStatus: Potential - manual confirmation of attribute breakout is required.",
                request.url, payload, refl.context
            ),
        })
    }
}
