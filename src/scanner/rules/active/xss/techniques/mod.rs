pub mod attribute;
pub mod dom;
pub mod html;
pub mod javascript;
pub mod reflected;
pub mod stored;
pub mod url;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::analyzer::models::{ReflectionResult, ReflectionType};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::payload::models::InsertionPoint;

/// Returns the reflection only when the injected payload appears verbatim
/// (raw, unescaped) in the response body. HTML-encoded, partial and other
/// non-verbatim reflections are rejected: an application correctly escaping a
/// value is not evidence of client-side injection.
pub(crate) fn raw_reflection(
    analyzer: &ResponseAnalyzer,
    response: &HttpResponse,
    payload: &str,
) -> Option<ReflectionResult> {
    let refl = analyzer.detect_reflection(response, payload);
    if refl.found && refl.reflection_type == ReflectionType::Full {
        Some(refl)
    } else {
        None
    }
}

pub trait XssTechnique: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn payload_category(&self) -> &'static str;

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        points: &[InsertionPoint],
    ) -> Vec<(String, HttpRequest)>;

    fn analyze(
        &self,
        analyzer: &ResponseAnalyzer,
        _request: &HttpRequest,
        response: &HttpResponse,
        payload: &str,
    ) -> Option<XssFinding>;
}

#[derive(Debug, Clone)]
pub struct XssFinding {
    pub technique: String,
    pub injection_point: String,
    pub payload: String,
    pub confidence: RuleConfidence,
    pub evidence: String,
}
