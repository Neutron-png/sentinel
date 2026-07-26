use crate::network::models::{HttpBody, HttpRequest, HttpResponse};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active::sqli::techniques::{SqliFinding, SqliTechnique};

pub struct BooleanBasedTechnique;

impl SqliTechnique for BooleanBasedTechnique {
    fn name(&self) -> &'static str { "Boolean-Based Blind SQLi" }
    fn description(&self) -> &'static str { "Detects SQL injection by comparing responses to boolean-true and boolean-false conditions" }

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        param: &str,
        _db: &str,
    ) -> Vec<(String, HttpRequest)> {
        let payloads = vec![
            ("true", format!("{}' AND 1=1--", param)),
            ("false", format!("{}' AND 1=2--", param)),
        ];
        let point = InsertionPoint::QueryParam(param.to_string());
        payloads.iter().map(|(label, val)| {
            (label.to_string(), engine.insert_into(request, &point, val))
        }).collect()
    }

    fn analyze_response(
        &self,
        baseline: &HttpResponse,
        test: &HttpResponse,
        payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding> {
        let baseline_len = baseline.body.as_text().map(|s| s.len()).unwrap_or(0);
        let test_len = test.body.as_text().map(|s| s.len()).unwrap_or(0);
        let diff = if baseline_len > test_len { baseline_len - test_len } else { test_len - baseline_len };

        if diff > 100 && payload == "false" {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Medium,
                database_hint: None,
                evidence: format!("Boolean-based SQLi detected.\nRequest: {}\nBaseline length: {}\nTest length: {}\nDifference: {}",
                    original_request.url, baseline_len, test_len, diff),
            })
        } else { None }
    }
}
