use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::oauth::engine::OAuthEngine;
use crate::scanner::passive::models::{ScanConfidence, ScanResult, ScanSeverity};
use crate::scanner::passive::rule::ScanRule;

pub struct OAuthAnalysisRule;

impl ScanRule for OAuthAnalysisRule {
    fn id(&self) -> &'static str {
        "oauth-analysis"
    }

    fn name(&self) -> &'static str {
        "OAuth 2.0 & OIDC Security Analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes OAuth 2.0 and OpenID Connect traffic for security weaknesses including missing PKCE, insecure redirect URIs, legacy flows, missing state parameters, excessive scopes, and long-lived tokens."
    }

    fn category(&self) -> &'static str {
        "Authentication"
    }

    fn severity(&self) -> ScanSeverity {
        ScanSeverity::High
    }

    fn confidence(&self) -> ScanConfidence {
        ScanConfidence::High
    }

    fn references(&self) -> &'static str {
        "https://datatracker.ietf.org/doc/html/rfc6749\nhttps://datatracker.ietf.org/doc/html/rfc7636\nhttps://openid.net/specs/\nhttps://owasp.org/www-project-api-security/"
    }

    fn applies_to(&self, _request: &HttpRequest, _response: &HttpResponse) -> bool {
        true
    }

    fn analyze(
        &self,
        transaction_id: Uuid,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<ScanResult> {
        let mut results = Vec::new();
        let mut engine = OAuthEngine::new();

        let req_headers: Vec<(String, String)> = request
            .headers
            .iter()
            .map(|h| (h.name.clone(), h.value.clone()))
            .collect();
        let req_body = request.body.as_text();
        let req_content_type = request
            .headers
            .iter()
            .find(|h| h.name.to_lowercase() == "content-type")
            .map(|h| h.value.as_str());

        let req_ids = engine.detect_in_request(
            &request.method,
            &request.url,
            &req_headers,
            req_body,
            req_content_type,
        );

        for id in &req_ids {
            let findings = engine.analyze_session(*id);
            for finding in findings {
                let severity = match finding.severity.as_str() {
                    "Critical" => ScanSeverity::Critical,
                    "High" => ScanSeverity::High,
                    "Medium" => ScanSeverity::Medium,
                    "Low" => ScanSeverity::Low,
                    _ => ScanSeverity::Informational,
                };

                let confidence = match finding.confidence.as_str() {
                    "Confirmed" => ScanConfidence::Confirmed,
                    "High" => ScanConfidence::High,
                    "Medium" => ScanConfidence::Medium,
                    "Low" => ScanConfidence::Low,
                    _ => ScanConfidence::Tentative,
                };

                let mut result = ScanResult::new(
                    "oauth-analysis",
                    transaction_id,
                    &finding.title,
                    severity,
                    confidence,
                );

                result.description = format!(
                    "{}\n\nRecommendation: {}",
                    finding.description, finding.recommendation,
                );

                if let Some(cwe) = &finding.cwe {
                    result.references = format!(
                        "{}\n{}",
                        cwe,
                        finding.owasp_category.as_deref().unwrap_or("")
                    );
                }

                result.evidence = finding.evidence.clone();

                results.push(result);
            }
        }

        let resp_headers: Vec<(String, String)> = response
            .headers
            .iter()
            .map(|h| (h.name.clone(), h.value.clone()))
            .collect();
        let resp_body = response.body.as_text();
        let resp_content_type = response.content_type.as_deref();

        let resp_ids = engine.detect_in_response(
            &response.url,
            response.status_code,
            &resp_headers,
            resp_body,
            resp_content_type,
        );

        for id in &resp_ids {
            let findings = engine.analyze_session(*id);
            for finding in findings {
                let severity = match finding.severity.as_str() {
                    "Critical" => ScanSeverity::Critical,
                    "High" => ScanSeverity::High,
                    "Medium" => ScanSeverity::Medium,
                    "Low" => ScanSeverity::Low,
                    _ => ScanSeverity::Informational,
                };

                let confidence = match finding.confidence.as_str() {
                    "Confirmed" => ScanConfidence::Confirmed,
                    "High" => ScanConfidence::High,
                    "Medium" => ScanConfidence::Medium,
                    "Low" => ScanConfidence::Low,
                    _ => ScanConfidence::Tentative,
                };

                let mut result = ScanResult::new(
                    "oauth-analysis",
                    transaction_id,
                    &finding.title,
                    severity,
                    confidence,
                );

                result.description = format!(
                    "{}\n\nRecommendation: {}",
                    finding.description, finding.recommendation,
                );

                if let Some(cwe) = &finding.cwe {
                    result.references = format!(
                        "{}\n{}",
                        cwe,
                        finding.owasp_category.as_deref().unwrap_or("")
                    );
                }

                result.evidence = finding.evidence.clone();

                results.push(result);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::models::{HttpBody, HttpHeader};

    fn oauth_request() -> HttpRequest {
        HttpRequest {
            method: "GET".into(),
            url: "https://example.com/authorize?client_id=test&redirect_uri=http://localhost/callback&response_type=code&scope=admin".into(),
            headers: vec![],
            cookies: vec![],
            body: HttpBody::Empty,
            query_params: vec![],
        }
    }

    fn oauth_token_response() -> HttpResponse {
        HttpResponse {
            status_code: 200,
            status_text: "OK".into(),
            headers: vec![HttpHeader {
                name: "Content-Type".into(),
                value: "application/json".into(),
            }],
            cookies: vec![],
            body: HttpBody::Text(
                r#"{"access_token":"abc.def.ghi","token_type":"Bearer","expires_in":86400,"scope":"admin"}"#
                    .into(),
            ),
            protocol: "HTTP/1.1".into(),
            content_type: Some("application/json".into()),
            content_length: None,
            timing: Default::default(),
            url: "https://example.com/token".into(),
        }
    }

    #[test]
    fn test_oauth_analysis_detects_insecure_redirect() {
        let rule = OAuthAnalysisRule;
        let request = oauth_request();
        let response = HttpResponse {
            status_code: 200,
            status_text: "OK".into(),
            headers: vec![],
            cookies: vec![],
            body: HttpBody::Empty,
            protocol: "HTTP/1.1".into(),
            content_type: None,
            content_length: None,
            timing: Default::default(),
            url: "https://example.com/".into(),
        };
        let results = rule.analyze(Uuid::new_v4(), &request, &response);
        assert!(!results.is_empty());
        let has_redirect = results.iter().any(|r| r.title.contains("redirect"));
        let has_pkce = results.iter().any(|r| r.title.contains("PKCE"));
        let has_scopes = results.iter().any(|r| r.title.contains("scope"));
        assert!(
            has_redirect || has_pkce || has_scopes,
            "Should detect at least one OAuth issue"
        );
    }

    #[test]
    fn test_oauth_analysis_detects_token_issues() {
        let rule = OAuthAnalysisRule;
        let request = HttpRequest {
            method: "POST".into(),
            url: "https://example.com/token".into(),
            headers: vec![],
            cookies: vec![],
            body: HttpBody::Empty,
            query_params: vec![],
        };
        let response = oauth_token_response();
        let results = rule.analyze(Uuid::new_v4(), &request, &response);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_rule_metadata() {
        let rule = OAuthAnalysisRule;
        assert_eq!(rule.id(), "oauth-analysis");
        assert_eq!(rule.category(), "Authentication");
    }
}
