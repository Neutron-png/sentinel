use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;
use crate::scanner::rules::active::sqli::techniques::{SqliFinding, SqliTechnique};

pub struct ErrorBasedTechnique;

impl SqliTechnique for ErrorBasedTechnique {
    fn name(&self) -> &'static str { "Error-Based SQLi" }
    fn description(&self) -> &'static str { "Detects SQL injection by triggering database error messages in HTTP responses" }

    fn generate_requests(
        &self,
        engine: &PayloadEngine,
        request: &HttpRequest,
        param: &str,
        _db: &str,
    ) -> Vec<(String, HttpRequest)> {
        let payloads = engine.generate("SQL Injection");
        let point = InsertionPoint::QueryParam(param.to_string());
        payloads.iter().map(|p| {
            (p.value.clone(), engine.insert_into(request, &point, &p.value))
        }).collect()
    }

    fn analyze_response(
        &self,
        _baseline: &HttpResponse,
        test: &HttpResponse,
        payload: &str,
        original_request: &HttpRequest,
    ) -> Option<SqliFinding> {
        let body = test.body.as_text().unwrap_or("");
        let error_patterns = [
            "SQL syntax", "mysql_fetch", "ORA-", "PostgreSQL", "SQLite",
            "unclosed quotation mark", "Microsoft OLE DB", "SQLServer",
            "syntax error", "unknown column", "unterminated string",
        ];
        if error_patterns.iter().any(|p| body.to_lowercase().contains(&p.to_lowercase())) {
            Some(SqliFinding {
                technique: self.name().to_string(),
                parameter: "query".to_string(),
                payload: payload.to_string(),
                confidence: crate::scanner::sdk::result::RuleConfidence::High,
                database_hint: guess_db(body),
                evidence: format!("Error-based SQLi detected.\nRequest: {}\nPayload: {}\nResponse body (excerpt): {}",
                    original_request.url, payload, &body[..body.len().min(500)]),
            })
        } else { None }
    }
}

fn guess_db(body: &str) -> Option<String> {
    if body.contains("mysql") { Some("MySQL".into()) }
    else if body.contains("ORA-") { Some("Oracle".into()) }
    else if body.contains("PostgreSQL") { Some("PostgreSQL".into()) }
    else if body.contains("SQLite") { Some("SQLite".into()) }
    else if body.contains("SQLServer") || body.contains("Microsoft") { Some("MSSQL".into()) }
    else { None }
}
