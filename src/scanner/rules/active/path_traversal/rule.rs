use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::context::ScanRuleContext;
use crate::scanner::sdk::metadata::RuleMetadata;
use crate::scanner::sdk::result::{RuleConfidence, RuleResult, RuleSeverity, RuleStatus};
use crate::scanner::sdk::rule::ScanRule;
use crate::scanner::rules::active::path_traversal::techniques::{PathTraversalTechnique, PathTraversalFinding};
use crate::scanner::rules::active::path_traversal::techniques::relative::Relative;
use crate::scanner::rules::active::path_traversal::techniques::absolute::Absolute;
use crate::scanner::rules::active::path_traversal::techniques::encoded::Encoded;
use crate::scanner::rules::active::path_traversal::techniques::double_encoded::DoubleEncoded;
use crate::scanner::rules::active::path_traversal::techniques::mixed_encoded::MixedEncoded;

pub struct PathTraversalRule {
    metadata: RuleMetadata,
    techniques: Vec<Box<dyn PathTraversalTechnique>>,
    payload_engine: PayloadEngine,
    analyzer: ResponseAnalyzer,
}

impl PathTraversalRule {
    pub fn new() -> Self {
        let mut techniques: Vec<Box<dyn PathTraversalTechnique>> = Vec::new();
        techniques.push(Box::new(Relative));
        techniques.push(Box::new(Absolute));
        techniques.push(Box::new(Encoded));
        techniques.push(Box::new(DoubleEncoded));
        techniques.push(Box::new(MixedEncoded));
        Self {
            metadata: RuleMetadata::new("ACTIVE-PT-001", "Path Traversal", "Injection")
                .with_severity(RuleSeverity::High)
                .with_confidence(RuleConfidence::Medium)
                .with_tags(vec!["path-traversal", "directory-traversal", "dot-dot-slash"]),
            techniques,
            payload_engine: PayloadEngine::new(),
            analyzer: ResponseAnalyzer::new(500),
        }
    }
}

impl ScanRule for PathTraversalRule {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }
    fn should_run(&self, _context: &ScanRuleContext) -> bool { true }

    fn build_requests(&self, context: &ScanRuleContext) -> Vec<HttpRequest> {
        let base = context.build_get("/");
        let points = vec![
            InsertionPoint::PathParam("file".into()),
            InsertionPoint::QueryParam("file".into()),
            InsertionPoint::QueryParam("path".into()),
            InsertionPoint::QueryParam("page".into()),
            InsertionPoint::QueryParam("document".into()),
        ];
        let mut requests = Vec::new();
        for technique in &self.techniques {
            let gen = technique.generate_requests(&self.payload_engine, &base, &points);
            for (_, req) in gen { requests.push(req); }
        }
        requests
    }

    fn execute(&self, _context: &ScanRuleContext, request: &HttpRequest, response: &HttpResponse) -> Vec<RuleResult> {
        let mut results = Vec::new();
        let payloads = self.payload_engine.generate("Path Traversal");
        for technique in &self.techniques {
            for payload in &payloads {
                if let Some(finding) = technique.analyze(&self.analyzer, request, response, &payload.value) {
                    let mut result = RuleResult::new(&self.metadata.id, &format!("Path Traversal - {}", finding.technique), RuleSeverity::High);
                    result = result.with_status(RuleStatus::Potential).with_confidence(finding.confidence).with_description(&format!("{} in {}", finding.technique, finding.injection_point)).with_evidence(&finding.evidence);
                    results.push(result);
                }
            }
        }
        results
    }
}
