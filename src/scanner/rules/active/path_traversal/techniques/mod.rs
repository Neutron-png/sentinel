pub mod absolute;
pub mod double_encoded;
pub mod encoded;
pub mod mixed_encoded;
pub mod relative;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::result::RuleConfidence;

pub trait PathTraversalTechnique: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)>;
    fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<PathTraversalFinding>;
}

#[derive(Debug, Clone)]
pub struct PathTraversalFinding {
    pub technique: String,
    pub injection_point: String,
    pub payload: String,
    pub confidence: RuleConfidence,
    pub evidence: String,
}
