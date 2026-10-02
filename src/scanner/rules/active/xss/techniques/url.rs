use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::rules::active::xss::techniques::{raw_reflection, XssFinding, XssTechnique};

pub struct UrlContextXss;

impl XssTechnique for UrlContextXss {
    fn name(&self) -> &'static str { "URL Context XSS" }
    fn description(&self) -> &'static str { "Detects XSS in URL context where payloads appear in href/src attributes" }
    fn payload_category(&self) -> &'static str { "XSS" }

    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
        let url_payloads = vec!["javascript:alert(1)", "data:text/html,<script>alert(1)</script>"];
        let mut results = Vec::new();
        for p in &url_payloads {
            for point in points {
                results.push((p.to_string(), engine.insert_into(request, point, p)));
            }
        }
        results
    }

    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<XssFinding> {
        let refl = raw_reflection(analyzer, response, payload)?;
        let scheme_injection = payload.contains("javascript:") || payload.contains("data:text/html");
        if !scheme_injection {
            return None;
        }
        Some(XssFinding {
            technique: self.name().to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence: RuleConfidence::Medium,
            evidence: format!(
                "A URL-scheme injection payload was reflected verbatim (unescaped).\nURL: {}\nPayload: {}\nContext: {}\nStatus: Potential - manual confirmation that the value lands in a navigable URL context is required.",
                request.url, payload, refl.context
            ),
        })
    }
}
