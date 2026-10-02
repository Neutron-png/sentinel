use crate::network::models::{HttpRequest, HttpResponse};
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
        let diff = baseline_len.abs_diff(test_len);
        // Require a material relative or absolute change, and only for the
        // false-condition payload. A boolean-based conclusion needs the
        // true/false pair compared; a single response is a candidate only.
        let threshold = (baseline_len / 5).max(200);
        if payload == "false" && diff > threshold && test_len < baseline_len {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Low,
                database_hint: None,
                evidence: format!(
                    "The false-condition response differs materially from the baseline.\nRequest: {}\nBaseline length: {}\nTest length: {}\nDifference: {}\nStatus: Potential - confirm by comparing the true- and false-condition responses side by side.",
                    original_request.url, baseline_len, test_len, diff
                ),
            })
        } else { None }
    }
}
