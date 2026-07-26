use uuid::Uuid;

use crate::jwt::detector;
use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::models::{ScanConfidence, ScanResult, ScanSeverity};
use crate::scanner::passive::rule::ScanRule;

pub struct JwtAnalysisRule;

impl ScanRule for JwtAnalysisRule {
    fn id(&self) -> &'static str {
        "jwt-analysis"
    }

    fn name(&self) -> &'static str {
        "JWT Security Analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes JSON Web Tokens (JWTs) found in HTTP traffic for security weaknesses including weak algorithms, missing expiration, sensitive claims, and algorithm confusion vulnerabilities."
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
        "https://datatracker.ietf.org/doc/html/rfc7519\nhttps://owasp.org/www-project-api-security/\nhttps://cwe.mitre.org/data/definitions/345.html"
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

        let tokens = detector::detect_in_request(
            &request.method,
            &request.url,
            &req_headers,
            req_body,
            req_content_type,
        );

        for token in tokens {
            let analysis = crate::jwt::analysis::analyze_token(&token);

            for finding in analysis {
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
                    "jwt-analysis",
                    transaction_id,
                    &finding.title,
                    severity,
                    confidence,
                );

                result.description = format!(
                    "{}\n\nLocation: {}\nAlgorithm: {}\n\nRecommendation: {}",
                    finding.description,
                    token.location.location_type.label(),
                    token.algorithm,
                    finding.recommendation,
                );

                if let Some(cwe) = &finding.cwe {
                    result.references = format!(
                        "{}\n{}\n{}",
                        cwe,
                        finding.owasp_category.as_deref().unwrap_or(""),
                        "https://cwe.mitre.org/data/definitions/345.html"
                    );
                }

                result.evidence = format!(
                    "JWT Header: {}\nJWT Payload (decoded): {}",
                    serde_json::to_string_pretty(&token.header).unwrap_or_default(),
                    serde_json::to_string_pretty(&token.claims).unwrap_or_default(),
                );

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

        let resp_tokens = detector::detect_in_response(
            &response.url,
            &resp_headers,
            resp_body,
            resp_content_type,
        );

        for token in resp_tokens {
            let analysis = crate::jwt::analysis::analyze_token(&token);

            for finding in analysis {
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
                    "jwt-analysis",
                    transaction_id,
                    &finding.title,
                    severity,
                    confidence,
                );

                result.description = format!(
                    "{}\n\nLocation: {}\nAlgorithm: {}\n\nRecommendation: {}",
                    finding.description,
                    token.location.location_type.label(),
                    token.algorithm,
                    finding.recommendation,
                );

                if let Some(cwe) = &finding.cwe {
                    result.references = format!(
                        "{}\n{}",
                        cwe,
                        finding.owasp_category.as_deref().unwrap_or("")
                    );
                }

                result.evidence = format!(
                    "JWT Header: {}\nJWT Payload (decoded): {}",
                    serde_json::to_string_pretty(&token.header).unwrap_or_default(),
                    serde_json::to_string_pretty(&token.claims).unwrap_or_default(),
                );

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

    fn jwt_request() -> HttpRequest {
        let header_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"alg":"none","typ":"JWT"}"#,
        );
        let payload_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"sub":"admin","password":"secret"}"#,
        );
        let token = format!("{}.{}.", header_b64, payload_b64);

        HttpRequest {
            method: "GET".into(),
            url: "https://example.com/api".into(),
            headers: vec![HttpHeader {
                name: "Authorization".into(),
                value: format!("Bearer {}", token),
            }],
            cookies: vec![],
            body: HttpBody::Empty,
            query_params: vec![],
        }
    }

    fn empty_response() -> HttpResponse {
        HttpResponse {
            status_code: 200,
            status_text: "OK".into(),
            headers: vec![],
            cookies: vec![],
            body: HttpBody::Empty,
            protocol: "HTTP/1.1".into(),
            content_type: None,
            content_length: None,
            timing: Default::default(),
            url: "https://example.com/api".into(),
        }
    }

    #[test]
    fn test_jwt_analysis_rule_detects_none_alg() {
        let rule = JwtAnalysisRule;
        let results = rule.analyze(Uuid::new_v4(), &jwt_request(), &empty_response());
        assert!(!results.is_empty());
        let has_none = results.iter().any(|r| r.title.contains("none"));
        let has_password = results.iter().any(|r| r.title.contains("password"));
        assert!(has_none || has_password);
    }

    #[test]
    fn test_jwt_rule_metadata() {
        let rule = JwtAnalysisRule;
        assert_eq!(rule.id(), "jwt-analysis");
        assert_eq!(rule.category(), "Authentication");
    }
}
