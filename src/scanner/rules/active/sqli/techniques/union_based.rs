use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active::sqli::techniques::{SqliFinding, SqliTechnique};

pub struct UnionBasedTechnique;

impl SqliTechnique for UnionBasedTechnique {
    fn name(&self) -> &'static str { "UNION-Based SQLi" }
    fn description(&self) -> &'static str { "Detects SQL injection using UNION SELECT to extract data" }

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        param: &str,
        _db: &str,
    ) -> Vec<(String, HttpRequest)> {
        let payloads = vec![
            format!("{}' UNION SELECT NULL--", param),
            format!("{}' UNION SELECT NULL,NULL--", param),
            format!("{}' UNION SELECT NULL,NULL,NULL--", param),
        ];
        let point = InsertionPoint::QueryParam(param.to_string());
        payloads.iter().enumerate().map(|(i, val)| {
            (format!("union_{}", i), engine.insert_into(request, &point, val))
        }).collect()
    }

    fn analyze_response(
        &self,
        baseline: &HttpResponse,
        test: &HttpResponse,
        _payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding> {
        let b_len = baseline.body.as_text().map(|s| s.len()).unwrap_or(0);
        let t_len = test.body.as_text().map(|s| s.len()).unwrap_or(0);
        // UNION can change response structure
        if t_len != b_len && t_len > 0 {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: "UNION SELECT".to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Medium,
                database_hint: None,
                evidence: format!("UNION-based SQLi detected.\nRequest: {}\nBaseline length: {}\nTest length: {}",
                    original_request.url, b_len, t_len),
            })
        } else { None }
    }
}
