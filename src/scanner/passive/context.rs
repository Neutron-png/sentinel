#![allow(dead_code)]

use crate::network::models::{HttpRequest, HttpResponse};

/// Returns true only when a response represents a served HTML document with a
/// success status. Security-header rules must not report on API/JSON payloads,
/// redirects, empty bodies, error pages, or responses whose content type
/// cannot be established, because a missing header in those contexts is not
/// evidence of a security weakness.
pub fn is_document_success(response: &HttpResponse) -> bool {
    if !(200..300).contains(&response.status_code) {
        return false;
    }
    let content_type = response
        .content_type
        .clone()
        .or_else(|| {
            response
                .headers
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case("content-type"))
                .map(|h| h.value.clone())
        })
        .unwrap_or_default()
        .to_ascii_lowercase();
    let is_html = content_type.contains("text/html")
        || content_type.contains("application/xhtml+xml");
    if !is_html {
        return false;
    }
    let body_len = response.body.len();
    body_len > 0
}

/// HSTS is only meaningful for responses served over HTTPS.
pub fn is_https(request: &HttpRequest) -> bool {
    request.url.trim_start().to_ascii_lowercase().starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::models::{HttpBody, HttpHeader, HttpResponse, RequestTiming};

    fn response(status: u16, content_type: Option<&str>, body: &str) -> HttpResponse {
        HttpResponse {
            status_code: status,
            status_text: String::new(),
            headers: content_type
                .map(|ct| {
                    vec![HttpHeader {
                        name: "Content-Type".into(),
                        value: ct.into(),
                    }]
                })
                .unwrap_or_default(),
            cookies: Vec::new(),
            body: HttpBody::Text(body.to_string()),
            protocol: "HTTP/1.1".into(),
            content_type: content_type.map(|s| s.to_string()),
            content_length: Some(body.len() as u64),
            timing: RequestTiming::default(),
            url: String::new(),
        }
    }

    fn request(url: &str) -> HttpRequest {
        HttpRequest {
            method: "GET".into(),
            url: url.into(),
            headers: Vec::new(),
            cookies: Vec::new(),
            body: HttpBody::Empty,
            query_params: Vec::new(),
        }
    }

    #[test]
    fn html_success_is_reportable() {
        assert!(is_document_success(&response(200, Some("text/html; charset=utf-8"), "<html>")));
    }

    #[test]
    fn json_api_response_is_not_reportable() {
        assert!(!is_document_success(&response(200, Some("application/json"), "{}")));
    }

    #[test]
    fn redirects_and_errors_are_not_reportable() {
        assert!(!is_document_success(&response(302, Some("text/html"), "<html>")));
        assert!(!is_document_success(&response(404, Some("text/html"), "<html>")));
        assert!(!is_document_success(&response(500, Some("text/html"), "<html>")));
    }

    #[test]
    fn missing_content_type_is_not_reportable() {
        assert!(!is_document_success(&response(200, None, "<html>")));
    }

    #[test]
    fn empty_body_is_not_reportable() {
        assert!(!is_document_success(&response(200, Some("text/html"), "")));
    }

    #[test]
    fn https_detection() {
        assert!(is_https(&request("https://example.com/")));
        assert!(!is_https(&request("http://example.com/")));
    }
}
