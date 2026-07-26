use crate::network::models::{HttpRequest, HttpResponse, HttpBody};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::sdk::context::ScanRuleContext;
use crate::scanner::sdk::errors::SdkError;
use crate::scanner::sdk::metadata::RuleMetadata;
use crate::scanner::sdk::result::{RuleConfidence, RuleResult, RuleSeverity, RuleStatus};
use crate::scanner::sdk::rule::ScanRule;
use crate::scanner::rules::active\xss\techniques::{XssTechnique, XssFinding};
use crate::scanner::rules::active\xss\techniques::attribute::AttributeXss;
use crate::scanner::rules::active\xss\techniques::dom::DomXss;
use crate::scanner::rules::active\xss\techniques::html::HtmlContextXss;
use crate::scanner::rules::active\xss\techniques::javascript::JavaScriptXss;
use crate::scanner::rules::active\xss\techniques::reflected::ReflectedXss;
use crate::scanner::rules::active\xss\techniques::stored::StoredXss;
use crate::scanner::rules::active\xss\techniques::url::UrlContextXss;

pub struct XssRule {
    metadata: RuleMetadata,
    techniques: Vec<Box<dyn XssTechnique>>,
    payload_engine: PayloadEngine,
    analyzer: ResponseAnalyzer,
}

impl XssRule {
    pub fn new() -> Self {
        let mut techniques: Vec<Box<dyn XssTechnique>> = Vec::new();
        techniques.push(Box::new(ReflectedXss));
        techniques.push(Box::new(StoredXss));
        techniques.push(Box::new(DomXss));
        techniques.push(Box::new(HtmlContextXss));
        techniques.push(Box::new(AttributeXss));
        techniques.push(Box::new(JavaScriptXss));
        techniques.push(Box::new(UrlContextXss));
        Self {
            metadata: RuleMetadata::new("ACTIVE-XSS-001", "Cross-Site Scripting", "Injection")
                .with_severity(RuleSeverity::High)
                .with_confidence(RuleConfidence::Medium)
                .with_tags(vec!["xss", "injection", "owasp-top-10", "client-side"]),
            techniques,
            payload_engine: PayloadEngine::new(),
            analyzer: ResponseAnalyzer::new(500),
        }
    }
}

impl ScanRule for XssRule {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }

    fn should_run(&self, _context: &ScanRuleContext) -> bool { true }

    fn build_requests(&self, context: &ScanRuleContext) -> Vec<HttpRequest> {
        let base = context.build_get("/");
        let points = vec![
            InsertionPoint::QueryParam("q".into()),
            InsertionPoint::QueryParam("search".into()),
            InsertionPoint::QueryParam("id".into()),
            InsertionPoint::PathParam("id".into()),
            InsertionPoint::Header("Referer".into()),
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
        for technique in &self.techniques {
            let payloads = self.payload_engine.generate(technique.payload_category());
            for payload in &payloads {
                if let Some(finding) = technique.analyze(&self.analyzer, request, response, &payload.value) {
                    let mut result = RuleResult::new(&self.metadata.id, &format!("XSS - {}", finding.technique), RuleSeverity::High);
                    result = result
                        .with_status(RuleStatus::Potential)
                        .with_confidence(finding.confidence)
                        .with_description(&format!("{} in {}", finding.technique, finding.injection_point))
                        .with_evidence(&finding.evidence);
                    results.push(result);
                }
            }
        }
        results
    }
}
