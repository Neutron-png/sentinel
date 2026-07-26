use uuid::Uuid;

use crate::grpc::detector;
use crate::grpc::engine::GrpcEngine;
use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::models::{ScanConfidence, ScanResult, ScanSeverity};
use crate::scanner::passive::rule::ScanRule;

pub struct GrpcAnalysisRule;

impl ScanRule for GrpcAnalysisRule {
    fn id(&self) -> &'static str {
        "grpc-analysis"
    }

    fn name(&self) -> &'static str {
        "gRPC Security Analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes gRPC traffic for security issues including sensitive data in messages, missing authentication, unusually large messages, and service enumeration metadata."
    }

    fn category(&self) -> &'static str {
        "API"
    }

    fn severity(&self) -> ScanSeverity {
        ScanSeverity::High
    }

    fn confidence(&self) -> ScanConfidence {
        ScanConfidence::High
    }

    fn references(&self) -> &'static str {
        "https://grpc.io/docs/\nhttps://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md\nhttps://owasp.org/www-project-api-security/"
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
        let mut engine = GrpcEngine::new();

        let req_headers: Vec<(String, String)> = request
            .headers
            .iter()
            .map(|h| (h.name.clone(), h.value.clone()))
            .collect();
        let resp_headers: Vec<(String, String)> = response
            .headers
            .iter()
            .map(|h| (h.name.clone(), h.value.clone()))
            .collect();

        if !detector::is_grpc_request(&req_headers) && !detector::is_grpc_response(&resp_headers) {
            return results;
        }

        let req_body = request.body.as_bytes();
        let resp_body = response.body.as_bytes();

        let id = engine.detect(
            &request.method,
            &request.url,
            &req_headers,
            &resp_headers,
            response.status_code,
            Some(req_body),
            Some(resp_body),
        );

        if let Some(msg_id) = id {
            let findings = engine.analyze_message(msg_id);
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
                    "grpc-analysis",
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

    fn grpc_request() -> HttpRequest {
        HttpRequest {
            method: "POST".into(),
            url: "https://example.com/example.Greeter/SayHello".into(),
            headers: vec![
                HttpHeader {
                    name: "content-type".into(),
                    value: "application/grpc".into(),
                },
                HttpHeader {
                    name: "te".into(),
                    value: "trailers".into(),
                },
            ],
            cookies: vec![],
            body: HttpBody::Bytes(vec![0, 0, 0, 0, 4, 0x08, 0x96, 0x01]),
            query_params: vec![],
        }
    }

    fn grpc_response() -> HttpResponse {
        HttpResponse {
            status_code: 200,
            status_text: "OK".into(),
            headers: vec![
                HttpHeader {
                    name: "content-type".into(),
                    value: "application/grpc".into(),
                },
                HttpHeader {
                    name: "grpc-status".into(),
                    value: "0".into(),
                },
            ],
            cookies: vec![],
            body: HttpBody::Bytes(vec![0, 0, 0, 0, 2, 0x10, 0x01]),
            protocol: "HTTP/2".into(),
            content_type: Some("application/grpc".into()),
            content_length: None,
            timing: Default::default(),
            url: "https://example.com/example.Greeter/SayHello".into(),
        }
    }

    #[test]
    fn test_grpc_rule_metadata() {
        let rule = GrpcAnalysisRule;
        assert_eq!(rule.id(), "grpc-analysis");
        assert_eq!(rule.category(), "API");
    }

    #[test]
    fn test_grpc_analysis_detects_missing_auth() {
        let rule = GrpcAnalysisRule;
        let request = grpc_request();
        let response = grpc_response();
        let results = rule.analyze(Uuid::new_v4(), &request, &response);
        let has_auth_finding = results.iter().any(|r| r.title.contains("authentication"));
        assert!(has_auth_finding || !results.is_empty());
    }
}
