pub mod attribute;
pub mod dom;
pub mod html;
pub mod javascript;
pub mod reflected;
pub mod stored;
pub mod url;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::sdk::result::RuleConfidence;
use crate::scanner::payload::models::InsertionPoint;

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
