use sentinel::network::models::{HttpBody, HttpHeader, HttpRequest, HttpResponse, RequestTiming};
use sentinel::scanner::passive::rule::ScanRule;
use sentinel::scanner::rules::passive::security_headers::csp::MissingCsp;
use sentinel::scanner::rules::passive::security_headers::hsts::MissingHsts;
use sentinel::scanner::rules::passive::security_headers::xfo::MissingXfo;

fn html_response(url: &str, status: u16, headers: Vec<(&str, &str)>, body: &str) -> HttpResponse {
    let content_type = headers
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case("content-type"))
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "text/html; charset=utf-8".to_string());
    HttpResponse {
        status_code: status,
        status_text: String::new(),
        headers: headers
            .into_iter()
            .map(|(n, v)| HttpHeader {
                name: n.to_string(),
                value: v.to_string(),
            })
            .collect(),
        cookies: Vec::new(),
        body: HttpBody::Text(body.to_string()),
        protocol: "HTTP/1.1".to_string(),
        content_type: Some(content_type),
        content_length: Some(body.len() as u64),
        timing: RequestTiming::default(),
        url: url.to_string(),
    }
}

fn get(url: &str) -> HttpRequest {
    HttpRequest {
        method: "GET".to_string(),
        url: url.to_string(),
        headers: Vec::new(),
        cookies: Vec::new(),
        body: HttpBody::Empty,
        query_params: Vec::new(),
    }
}

fn findings(rule: &dyn ScanRule, request: &HttpRequest, response: &HttpResponse) -> usize {
    if !rule.applies_to(request, response) {
        return 0;
    }
    rule.analyze(uuid::Uuid::new_v4(), request, response).len()
}

const HARDENED_HEADERS: &[(&str, &str)] = &[
    ("Content-Type", "text/html; charset=utf-8"),
    ("Content-Security-Policy", "default-src 'self'"),
    ("X-Frame-Options", "DENY"),
    ("Strict-Transport-Security", "max-age=63072000; includeSubDomains"),
];

const UNHARDENED_HTML: &[(&str, &str)] = &[("Content-Type", "text/html; charset=utf-8")];

#[test]
fn vulnerable_html_fixture_is_detected() {
    let response = html_response(
        "https://app.example.com/",
        200,
        UNHARDENED_HTML.to_vec(),
        "<html><body>hello</body></html>",
    );
    let request = get("https://app.example.com/");
    assert_eq!(findings(&MissingCsp, &request, &response), 1, "CSP should be reported once");
    assert_eq!(findings(&MissingXfo, &request, &response), 1, "XFO should be reported once");
    assert_eq!(findings(&MissingHsts, &request, &response), 1, "HSTS should be reported over https");
}

#[test]
fn hardened_html_fixture_produces_zero_findings() {
    let response = html_response(
        "https://app.example.com/",
        200,
        HARDENED_HEADERS.to_vec(),
        "<html><body>hello</body></html>",
    );
    let request = get("https://app.example.com/");
    assert_eq!(findings(&MissingCsp, &request, &response), 0);
    assert_eq!(findings(&MissingXfo, &request, &response), 0);
    assert_eq!(findings(&MissingHsts, &request, &response), 0);
}

#[test]
fn json_api_responses_are_not_flagged() {
    let response = html_response(
        "https://api.example.com/v1/users",
        200,
        vec![("Content-Type", "application/json")],
        "{\"users\":[]}",
    );
    let request = get("https://api.example.com/v1/users");
    assert_eq!(findings(&MissingCsp, &request, &response), 0);
    assert_eq!(findings(&MissingXfo, &request, &response), 0);
    assert_eq!(findings(&MissingHsts, &request, &response), 0);
}

#[test]
fn redirects_and_error_pages_are_not_flagged() {
    let request = get("https://app.example.com/");
    for status in [301u16, 302, 400, 401, 403, 404, 500, 503] {
        let response = html_response(
            "https://app.example.com/",
            status,
            UNHARDENED_HTML.to_vec(),
            "<html><body>nope</body></html>",
        );
        assert_eq!(
            findings(&MissingCsp, &request, &response),
            0,
            "status {status} must not produce header findings"
        );
    }
}

#[test]
fn hsts_is_not_reported_over_plain_http() {
    let response = html_response(
        "http://app.example.com/",
        200,
        UNHARDENED_HTML.to_vec(),
        "<html><body>hello</body></html>",
    );
    let request = get("http://app.example.com/");
    assert_eq!(findings(&MissingHsts, &request, &response), 0, "HSTS is meaningless over http");
    assert_eq!(findings(&MissingCsp, &request, &response), 1, "CSP still applies to html over http");
}

#[test]
fn missing_content_type_is_not_flagged() {
    let mut response = html_response(
        "https://app.example.com/",
        200,
        vec![],
        "<html><body>hello</body></html>",
    );
    response.content_type = None;
    let request = get("https://app.example.com/");
    assert_eq!(findings(&MissingCsp, &request, &response), 0);
}
