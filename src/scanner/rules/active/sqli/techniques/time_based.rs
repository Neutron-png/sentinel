use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active::sqli::techniques::{SqliFinding, SqliTechnique};

pub struct TimeBasedTechnique;

impl SqliTechnique for TimeBasedTechnique {
    fn name(&self) -> &'static str { "Time-Based Blind SQLi" }
    fn description(&self) -> &'static str { "Detects SQL injection by measuring response time with sleep/delay payloads" }

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        param: &str,
        db: &str,
    ) -> Vec<(String, HttpRequest)> {
        let payload_str = match db.to_lowercase().as_str() {
            "mysql" => format!("{}' AND SLEEP(5)--", param),
            "postgresql" => format!("{}' OR PG_SLEEP(5)--", param),
            "mssql" => format!("{}'; WAITFOR DELAY '00:00:05'--", param),
            "oracle" => format!("{}' OR DBMS_LOCK.SLEEP(5)--", param),
            "sqlite" => format!("{}' AND LIKE('ABCDEFG',UPPER(HEX(RANDOMBLOB(500000000))))--", param),
            _ => format!("{}' AND SLEEP(5)--", param),
        };
        let point = InsertionPoint::QueryParam(param.to_string());
        vec![("sleep".to_string(), engine.insert_into(request, &point, &payload_str))]
    }

    fn analyze_response(
        &self,
        baseline: &HttpResponse,
        test: &HttpResponse,
        payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding> {
        let base_ms = baseline.timing.total_duration.as_millis() as u64;
        let test_ms = test.timing.total_duration.as_millis() as u64;
        // A time-based signal requires both an absolute delay (the payload
        // asked for ~5s) and a delay clearly beyond the baseline, so ordinary
        // network jitter or a slow page does not qualify.
        let delay_observed = test_ms >= 3000 && test_ms >= base_ms.saturating_add(2000);
        if delay_observed {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::Medium,
                database_hint: None,
                evidence: format!(
                    "The injected response was substantially slower than the baseline.\nRequest: {}\nBaseline time: {}ms\nTest time: {}ms\nPayload: {}\nStatus: Potential - timing is inherently noisy; re-test with multiple samples to confirm.",
                    original_request.url, base_ms, test_ms, payload
                ),
            })
        } else { None }
    }
}
