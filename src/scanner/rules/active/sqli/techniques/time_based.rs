use crate::network::models::{HttpBody, HttpRequest, HttpResponse};
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
        _baseline: &HttpResponse,
        test: &HttpResponse,
        payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding> {
        let duration = test.timing.total_duration;
        if duration.as_millis() > 3000 {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::High,
                database_hint: None,
                evidence: format!("Time-based SQLi detected.\nRequest: {}\nResponse time: {}ms\nPayload: {}",
                    original_request.url, duration.as_millis(), payload),
            })
        } else { None }
    }
}
