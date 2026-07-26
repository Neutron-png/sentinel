use crate::scanner::sdk::metadata::RuleMetadata;
use crate::scanner::sdk::result::{RuleResult, RuleSeverity, RuleConfidence, RuleStatus};
use crate::scanner::sdk::rule::ScanRule;
use crate::scanner::sdk::context::ScanRuleContext;
use crate::scanner::sdk::errors::SdkError;
use crate::scanner::payload::engine::PayloadEngine;
use crate::network::models::{HttpRequest, HttpResponse, HttpBody};
use crate::scanner::rules::active::sqli::techniques::{SqliTechnique, SqliFinding};
use crate::scanner::rules::active::sqli::techniques::error_based::ErrorBasedTechnique;
use crate::scanner::rules::active::sqli::techniques::boolean_based::BooleanBasedTechnique;
use crate::scanner::rules::active::sqli::techniques::time_based::TimeBasedTechnique;
use crate::scanner::rules::active::sqli::techniques::union_based::UnionBasedTechnique;

pub struct SqliRule {
    metadata: RuleMetadata,
    techniques: Vec<Box<dyn SqliTechnique>>,
    payload_engine: PayloadEngine,
    db_provider: String,
}

impl SqliRule {
    pub fn new() -> Self {
        let mut techniques: Vec<Box<dyn SqliTechnique>> = Vec::new();
        techniques.push(Box::new(ErrorBasedTechnique));
        techniques.push(Box::new(BooleanBasedTechnique));
        techniques.push(Box::new(TimeBasedTechnique));
        techniques.push(Box::new(UnionBasedTechnique));

        Self {
            metadata: RuleMetadata::new("ACTIVE-SQLI-001", "SQL Injection", "Injection")
                .with_severity(RuleSeverity::High)
                .with_confidence(RuleConfidence::Medium)
                .with_tags(vec!["sqli", "injection", "owasp-top-10"]),
            techniques,
            payload_engine: PayloadEngine::new(),
            db_provider: "mysql".to_string(),
        }
    }

    pub fn with_db(mut self, db: &str) -> Self { self.db_provider = db.to_string(); self }
}

impl ScanRule for SqliRule {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }

    fn should_run(&self, _context: &ScanRuleContext) -> bool { true }

    fn build_requests(&self, context: &ScanRuleContext) -> Vec<HttpRequest> {
        let mut requests = Vec::new();
        let base = context.build_get("/");
        for technique in &self.techniques {
            let params = vec!["id", "q", "search", "query", "page", "user"];
            for param in &params {
                let gen = technique.generate_requests(&self.payload_engine, &base, param, &self.db_provider);
                for (_, req) in gen { requests.push(req); }
            }
        }
        requests
    }

    fn execute(
        &self,
        context: &ScanRuleContext,
        _request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<RuleResult> {
        let mut results = Vec::new();
        // Baseline response for comparison
        let baseline = response;

        for technique in &self.techniques {
            let params = vec!["id", "q", "search"];
            for param in &params {
                let base_req = context.build_get(&format!("/?{}={}", param, "test"));
                let gen = technique.generate_requests(&self.payload_engine, &base_req, param, &self.db_provider);
                for (payload, _) in gen {
                    if let Some(finding) = technique.analyze_response(baseline, response, &payload, &base_req) {
                        let mut result = RuleResult::new(&self.metadata.id, &format!("SQL Injection - {}", finding.technique), RuleSeverity::High);
                        result = result
                            .with_status(RuleStatus::Potential)
                            .with_confidence(finding.confidence)
                            .with_description(&format!("{} in parameter '{}'", finding.technique, finding.parameter))
                            .with_evidence(&finding.evidence);
                        results.push(result);
                    }
                }
            }
        }
        results
    }
}
