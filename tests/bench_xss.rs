use sentinel::network::models::{HttpBody, HttpRequest, HttpResponse, RequestTiming};
use sentinel::scanner::analyzer::engine::ResponseAnalyzer;
use sentinel::scanner::rules::active::xss::techniques::attribute::AttributeXss;
use sentinel::scanner::rules::active::xss::techniques::dom::DomXss;
use sentinel::scanner::rules::active::xss::techniques::html::HtmlContextXss;
use sentinel::scanner::rules::active::xss::techniques::javascript::JavaScriptXss;
use sentinel::scanner::rules::active::xss::techniques::reflected::ReflectedXss;
use sentinel::scanner::rules::active::xss::techniques::stored::StoredXss;
use sentinel::scanner::rules::active::xss::techniques::url::UrlContextXss;
use sentinel::scanner::rules::active::xss::techniques::XssTechnique;

fn response_with_body(body: &str) -> HttpResponse {
    HttpResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        headers: Vec::new(),
        cookies: Vec::new(),
        body: HttpBody::Text(body.to_string()),
        protocol: "HTTP/1.1".to_string(),
        content_type: Some("text/html; charset=utf-8".to_string()),
        content_length: Some(body.len() as u64),
        timing: RequestTiming::default(),
        url: "http://target.example/search?q=x".to_string(),
    }
}

fn request() -> HttpRequest {
    HttpRequest {
        method: "GET".to_string(),
        url: "http://target.example/search?q=x".to_string(),
        headers: Vec::new(),
        cookies: Vec::new(),
        body: HttpBody::Empty,
        query_params: vec![("q".to_string(), "x".to_string())],
    }
}

fn detects(technique: &dyn XssTechnique, body: &str, payload: &str) -> bool {
    let analyzer = ResponseAnalyzer::new(500);
    technique
        .analyze(&analyzer, &request(), &response_with_body(body), payload)
        .is_some()
}

fn techniques() -> Vec<Box<dyn XssTechnique>> {
    vec![
        Box::new(ReflectedXss),
        Box::new(HtmlContextXss),
        Box::new(JavaScriptXss),
        Box::new(AttributeXss),
        Box::new(UrlContextXss),
        Box::new(StoredXss),
        Box::new(DomXss),
    ]
}

#[test]
fn unescaped_reflection_is_detected_by_all_techniques() {
    let cases: Vec<(&str, &str)> = vec![
        ("<script>alert(1)</script>", "<html><body><script>alert(1)</script></body></html>"),
        ("<img src=x onerror=alert(1)>", "<html><body><img src=x onerror=alert(1)></body></html>"),
        ("';alert(1)//", "<html><body>var x = '';alert(1)//';</body></html>"),
        ("\" onmouseover=alert(1)", "<html><body><a title=\" onmouseover=alert(1)\">x</a></body></html>"),
        ("javascript:alert(1)", "<html><body><a href=\"javascript:alert(1)\">x</a></body></html>"),
        ("x</p><script>alert(1)</script>", "<html><body><div>innerHTML x</p><script>alert(1)</script></div></body></html>"),
    ];
    for (payload, body) in cases {
        let any = techniques().iter().any(|t| detects(t.as_ref(), body, payload));
        assert!(any, "payload {payload:?} reflected unescaped should be detected by at least one technique");
    }
}

#[test]
fn html_escaped_reflection_is_never_a_finding() {
    let payload = "<script>alert(1)</script>";
    let escaped = "<html><body>&lt;script&gt;alert(1)&lt;/script&gt;</body></html>";
    for technique in techniques() {
        assert!(
            !detects(technique.as_ref(), escaped, payload),
            "{} must not flag an HTML-escaped (safe) reflection",
            technique.name()
        );
    }
}

#[test]
fn partial_reflection_is_not_evidence() {
    let payload = "<script>alert(1)</script>";
    let body = "<html><body><scr</body></html>";
    for technique in techniques() {
        assert!(
            !detects(technique.as_ref(), body, payload),
            "{} must not flag a partial reflection",
            technique.name()
        );
    }
}

#[test]
fn keyword_only_pages_are_not_findings() {
    let body = "<html><body>This page mentions alert(1) and <script> and innerHTML but reflects nothing.</body></html>";
    let unrelated_payload = "<img src=x onerror=alert(1)>";
    for technique in techniques() {
        assert!(
            !detects(technique.as_ref(), body, unrelated_payload),
            "{} must not flag keywords without a reflected payload",
            technique.name()
        );
    }
}

#[test]
fn dom_technique_requires_sink_adjacent_reflection() {
    let payload = "<img src=x onerror=alert(1)>";
    let reflected_without_sink = "<html><body><img src=x onerror=alert(1)></body></html>";
    assert!(
        !detects(&DomXss, reflected_without_sink, payload),
        "DOM technique must not fire without a nearby sink"
    );
    let reflected_with_sink =
        "<html><body><div id=x><img src=x onerror=alert(1)></div><script>el.innerHTML = x;</script></body></html>";
    assert!(
        detects(&DomXss, reflected_with_sink, payload),
        "DOM technique should fire on an unescaped reflection near a sink"
    );
}
