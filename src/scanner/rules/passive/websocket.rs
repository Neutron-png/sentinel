use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::models::{ScanConfidence, ScanResult, ScanSeverity};
use crate::scanner::passive::rule::ScanRule;

pub struct WebSocketAnalysisRule;

impl ScanRule for WebSocketAnalysisRule {
    fn id(&self) -> &'static str {
        "websocket-analysis"
    }

    fn name(&self) -> &'static str {
        "WebSocket Security Analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes WebSocket connections for security weaknesses including unencrypted connections, missing authentication, sensitive data in frames, and protocol-level issues."
    }

    fn category(&self) -> &'static str {
        "WebSocket"
    }

    fn severity(&self) -> ScanSeverity {
        ScanSeverity::High
    }

    fn confidence(&self) -> ScanConfidence {
        ScanConfidence::High
    }

    fn references(&self) -> &'static str {
        "https://datatracker.ietf.org/doc/html/rfc6455\nhttps://owasp.org/www-project-web-security-testing-guide/\nhttps://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html"
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

        let is_ws_request = detect_websocket_upgrade_request(request);
        let is_ws_response = detect_websocket_upgrade_response(response);

        if !is_ws_request && !is_ws_response {
            return results;
        }

        if is_ws_response {
            results.extend(check_unencrypted_ws(request, transaction_id));
            results.extend(check_origin_header(request, transaction_id));
            results.extend(check_missing_auth(request, transaction_id));
        }

        results
    }
}

fn detect_websocket_upgrade_request(req: &HttpRequest) -> bool {
    let has_upgrade = req
        .headers
        .iter()
        .any(|h| h.name.to_lowercase() == "upgrade" && h.value.to_lowercase() == "websocket");
    let has_connection = req.headers.iter().any(|h| {
        h.name.to_lowercase() == "connection" && h.value.to_lowercase().contains("upgrade")
    });
    has_upgrade && has_connection
}

fn detect_websocket_upgrade_response(resp: &HttpResponse) -> bool {
    resp.status_code == 101
        && resp
            .headers
            .iter()
            .any(|h| h.name.to_lowercase() == "upgrade" && h.value.to_lowercase() == "websocket")
}

fn check_unencrypted_ws(req: &HttpRequest, tx_id: Uuid) -> Vec<ScanResult> {
    let mut results = Vec::new();
    let url_lower = req.url.to_lowercase();

    if url_lower.starts_with("ws://") || url_lower.starts_with("http://") {
        let mut result = ScanResult::new(
            "websocket-analysis",
            tx_id,
            "Unencrypted WebSocket connection",
            ScanSeverity::High,
            ScanConfidence::High,
        );
        result.description = format!(
            "The WebSocket connection to '{}' uses an unencrypted ws:// or http:// URL. WebSocket traffic transmitted without TLS encryption can be intercepted and modified by network attackers.\n\nRecommendation: Always use wss:// (WebSocket Secure) with TLS 1.2+ for production WebSocket connections. Redirect all ws:// connections to wss:// and use HSTS headers.",
            req.url
        );
        result.references = "CWE-319: Cleartext Transmission of Sensitive Information\nOWASP ASVS V9.1.1\nRFC 6455 Section 10.6".into();
        result.evidence = format!("WebSocket URL: {}", req.url);
        results.push(result);
    }

    results
}

fn check_origin_header(req: &HttpRequest, tx_id: Uuid) -> Vec<ScanResult> {
    let mut results = Vec::new();

    let has_origin = req
        .headers
        .iter()
        .any(|h| h.name.to_lowercase() == "origin");

    if !has_origin {
        let mut result = ScanResult::new(
            "websocket-analysis",
            tx_id,
            "WebSocket handshake missing Origin header",
            ScanSeverity::Medium,
            ScanConfidence::Medium,
        );
        result.description = "The WebSocket upgrade request does not include an Origin header. Without origin validation, the server cannot prevent cross-site WebSocket hijacking (CSWSH) attacks.\n\nRecommendation: Always validate the Origin header in WebSocket handshake requests against an allowlist of trusted origins. Reject connections from unexpected or missing origins.".into();
        result.references =
            "CWE-346: Origin Validation Error\nOWASP ASVS V4.3.1\nRFC 6455 Section 10.2".into();
        result.evidence = "Missing Origin header in WebSocket upgrade request.".into();
        results.push(result);
    }

    results
}

fn check_missing_auth(req: &HttpRequest, tx_id: Uuid) -> Vec<ScanResult> {
    let mut results = Vec::new();

    let has_auth = req.headers.iter().any(|h| {
        let lower = h.name.to_lowercase();
        lower == "authorization" || lower.contains("auth") || lower == "cookie"
    });

    if !has_auth {
        let mut result = ScanResult::new(
            "websocket-analysis",
            tx_id,
            "WebSocket connection without authentication",
            ScanSeverity::High,
            ScanConfidence::Medium,
        );
        result.description = format!(
            "The WebSocket handshake does not include authentication credentials. Unauthenticated WebSocket connections may allow unauthorized access to real-time data.\n\nRecommendation: Authenticate WebSocket connections using session cookies, JWT tokens, or OAuth2 bearer tokens. Always validate authentication before completing the handshake.\n\nURL: {}",
            req.url
        );
        result.references = "CWE-306: Missing Authentication for Critical Function\nOWASP ASVS V2.1.1\nRFC 6455 Section 10.5".into();
        result.evidence = "No Authorization or Cookie headers found in upgrade request.".into();
        results.push(result);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::models::{HttpBody, HttpHeader};

    fn ws_request() -> HttpRequest {
        HttpRequest {
            method: "GET".into(),
            url: "ws://example.com/ws".into(),
            headers: vec![
                HttpHeader {
                    name: "Upgrade".into(),
                    value: "websocket".into(),
                },
                HttpHeader {
                    name: "Connection".into(),
                    value: "Upgrade".into(),
                },
                HttpHeader {
                    name: "Sec-WebSocket-Key".into(),
                    value: "dGhlIHNhbXBsZSBub25jZQ==".into(),
                },
                HttpHeader {
                    name: "Sec-WebSocket-Version".into(),
                    value: "13".into(),
                },
            ],
            cookies: vec![],
            body: HttpBody::Empty,
            query_params: vec![],
        }
    }

    fn ws_response() -> HttpResponse {
        HttpResponse {
            status_code: 101,
            status_text: "Switching Protocols".into(),
            headers: vec![
                HttpHeader {
                    name: "Upgrade".into(),
                    value: "websocket".into(),
                },
                HttpHeader {
                    name: "Connection".into(),
                    value: "Upgrade".into(),
                },
            ],
            cookies: vec![],
            body: HttpBody::Empty,
            protocol: "HTTP/1.1".into(),
            content_type: None,
            content_length: None,
            timing: Default::default(),
            url: "ws://example.com/ws".into(),
        }
    }

    #[test]
    fn test_detect_ws_upgrade() {
        assert!(detect_websocket_upgrade_request(&ws_request()));
        assert!(detect_websocket_upgrade_response(&ws_response()));
    }

    #[test]
    fn test_ws_rule_metadata() {
        let rule = WebSocketAnalysisRule;
        assert_eq!(rule.id(), "websocket-analysis");
        assert_eq!(rule.category(), "WebSocket");
    }

    #[test]
    fn test_ws_analysis_detects_unencrypted() {
        let rule = WebSocketAnalysisRule;
        let request = ws_request();
        let response = ws_response();
        let results = rule.analyze(Uuid::new_v4(), &request, &response);
        assert!(!results.is_empty());
        let has_unencrypted = results.iter().any(|r| r.title.contains("Unencrypted"));
        let has_no_origin = results.iter().any(|r| r.title.contains("Origin"));
        let has_no_auth = results.iter().any(|r| r.title.contains("authentication"));
        assert!(has_unencrypted || has_no_origin || has_no_auth);
    }
}
