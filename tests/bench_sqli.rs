use std::time::Duration;

use sentinel::network::models::{HttpBody, HttpRequest, HttpResponse, RequestTiming};
use sentinel::scanner::rules::active::sqli::techniques::boolean_based::BooleanBasedTechnique;
use sentinel::scanner::rules::active::sqli::techniques::error_based::ErrorBasedTechnique;
use sentinel::scanner::rules::active::sqli::techniques::time_based::TimeBasedTechnique;
use sentinel::scanner::rules::active::sqli::techniques::union_based::UnionBasedTechnique;
use sentinel::scanner::rules::active::sqli::techniques::SqliTechnique;

fn response(body: &str, status: u16, duration_ms: u64) -> HttpResponse {
    let mut timing = RequestTiming::default();
    timing.total_duration = Duration::from_millis(duration_ms);
    HttpResponse {
        status_code: status,
        status_text: String::new(),
        headers: Vec::new(),
        cookies: Vec::new(),
        body: HttpBody::Text(body.to_string()),
        protocol: "HTTP/1.1".to_string(),
        content_type: Some("text/html".to_string()),
        content_length: Some(body.len() as u64),
        timing,
        url: "http://target.example/item?id=1".to_string(),
    }
}

fn request() -> HttpRequest {
    HttpRequest {
        method: "GET".to_string(),
        url: "http://target.example/item?id=1".to_string(),
        headers: Vec::new(),
        cookies: Vec::new(),
        body: HttpBody::Empty,
        query_params: vec![("id".to_string(), "1".to_string())],
    }
}

fn analyze(
    technique: &dyn SqliTechnique,
    baseline: &HttpResponse,
    test: &HttpResponse,
    payload: &str,
) -> bool {
    technique
        .analyze_response(baseline, test, payload, &request())
        .is_some()
}

#[test]
fn error_based_requires_new_error_signature() {
    let clean = response("<html>ok</html>", 200, 50);
    let errored = response("<html>You have an error in your SQL syntax near '1''</html>", 500, 50);
    assert!(
        analyze(&ErrorBasedTechnique, &clean, &errored, "'"),
        "a new database error after injection should be detected"
    );

    let page_already_has_it = response("<html>SQL syntax error handler</html>", 200, 50);
    let same_error = response("<html>SQL syntax error handler</html>", 200, 50);
    assert!(
        !analyze(&ErrorBasedTechnique, &page_already_has_it, &same_error, "'"),
        "an error string already present in the baseline is page content, not injection evidence"
    );
}

#[test]
fn time_based_requires_absolute_and_relative_delay() {
    let fast_baseline = response("ok", 200, 100);
    let slow_test = response("ok", 200, 5000);
    assert!(
        analyze(&TimeBasedTechnique, &fast_baseline, &slow_test, "sleep"),
        "a ~5s delay well beyond baseline should be detected"
    );

    let slightly_slow = response("ok", 200, 700);
    assert!(
        !analyze(&TimeBasedTechnique, &fast_baseline, &slightly_slow, "sleep"),
        "ordinary jitter must not be reported"
    );

    let slow_baseline = response("ok", 200, 4000);
    let slightly_slower = response("ok", 200, 4500);
    assert!(
        !analyze(&TimeBasedTechnique, &slow_baseline, &slightly_slower, "sleep"),
        "a slow page that is not materially slower must not be reported"
    );
}

#[test]
fn boolean_based_requires_material_change_on_false_condition() {
    let baseline = response(&"a".repeat(1000), 200, 50);
    let shrunk = response(&"a".repeat(100), 200, 50);
    assert!(
        analyze(&BooleanBasedTechnique, &baseline, &shrunk, "false"),
        "a material shrink on the false condition is a candidate"
    );

    let nearly_same = response(&"a".repeat(950), 200, 50);
    assert!(
        !analyze(&BooleanBasedTechnique, &baseline, &nearly_same, "false"),
        "a trivial length change is not evidence"
    );

    let grown = response(&"a".repeat(2000), 200, 50);
    assert!(
        !analyze(&BooleanBasedTechnique, &baseline, &grown, "false"),
        "growth on the false condition is not the expected boolean signal"
    );
}

#[test]
fn union_based_requires_material_growth() {
    let baseline = response(&"a".repeat(1000), 200, 50);
    let grown = response(&"a".repeat(2000), 200, 50);
    assert!(
        analyze(&UnionBasedTechnique, &baseline, &grown, "union_0"),
        "material growth on a success response is a candidate"
    );

    let slightly_grown = response(&"a".repeat(1100), 200, 50);
    assert!(
        !analyze(&UnionBasedTechnique, &baseline, &slightly_grown, "union_0"),
        "a trivial length change is not evidence"
    );

    let error_response = response(&"a".repeat(5000), 500, 50);
    assert!(
        !analyze(&UnionBasedTechnique, &baseline, &error_response, "union_0"),
        "an error response is not a UNION success signal"
    );
}
