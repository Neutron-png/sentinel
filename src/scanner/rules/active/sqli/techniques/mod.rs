pub mod boolean_based;
pub mod error_based;
pub mod time_based;
pub mod union_based;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::sdk::context::ScanRuleContext;
use crate::scanner::sdk::result::{RuleConfidence, RuleResult, RuleSeverity};

pub trait SqliTechnique: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        param: &str,
        db: &str,
    ) -> Vec<(String, HttpRequest)>;

    fn analyze_response(
        &self,
        baseline: &HttpResponse,
        test: &HttpResponse,
        payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding>;
}

#[derive(Debug, Clone)]
pub struct SqliFinding {
    pub technique: String,
    pub parameter: String,
    pub payload: String,
    pub confidence: RuleConfidence,
    pub database_hint: Option<String>,
    pub evidence: String,
}
