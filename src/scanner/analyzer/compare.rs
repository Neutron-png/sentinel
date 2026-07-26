#![allow(dead_code)]

use std::collections::HashSet;

use crate::network::models::HttpResponse;
use crate::scanner::analyzer::models::{DiffResult, DiffType};

pub fn compare_status(a: &HttpResponse, b: &HttpResponse) -> DiffResult {
    DiffResult {
        field: "status_code".into(),
        baseline_value: a.status_code.to_string(),
        test_value: b.status_code.to_string(),
        diff_type: if a.status_code == b.status_code {
            DiffType::Same
        } else {
            DiffType::Changed
        },
    }
}

pub fn compare_headers(a: &HttpResponse, b: &HttpResponse) -> Vec<DiffResult> {
    let a_keys: HashSet<&str> = a.headers.iter().map(|h| h.name.as_str()).collect();
    let b_keys: HashSet<&str> = b.headers.iter().map(|h| h.name.as_str()).collect();
    let mut diffs = Vec::new();
    for key in a_keys.union(&b_keys) {
        let av = a
            .headers
            .iter()
            .find(|h| h.name == *key)
            .map(|h| h.value.as_str())
            .unwrap_or("");
        let bv = b
            .headers
            .iter()
            .find(|h| h.name == *key)
            .map(|h| h.value.as_str())
            .unwrap_or("");
        diffs.push(DiffResult {
            field: format!("header:{}", key),
            baseline_value: av.to_string(),
            test_value: bv.to_string(),
            diff_type: if av.is_empty() {
                DiffType::Removed
            } else if bv.is_empty() {
                DiffType::Added
            } else if av == bv {
                DiffType::Same
            } else {
                DiffType::Changed
            },
        });
    }
    diffs
}

pub fn compare_body(a: &HttpResponse, b: &HttpResponse) -> DiffResult {
    let a_body = a.body.as_text().unwrap_or("");
    let b_body = b.body.as_text().unwrap_or("");
    DiffResult {
        field: "body".into(),
        baseline_value: format!("{} bytes", a_body.len()),
        test_value: format!("{} bytes", b_body.len()),
        diff_type: if a_body == b_body {
            DiffType::Same
        } else {
            DiffType::Changed
        },
    }
}

pub fn compare_length(a: &HttpResponse, b: &HttpResponse) -> DiffResult {
    let al = a.body.as_text().map(|s| s.len()).unwrap_or(0);
    let bl = b.body.as_text().map(|s| s.len()).unwrap_or(0);
    DiffResult {
        field: "content_length".into(),
        baseline_value: al.to_string(),
        test_value: bl.to_string(),
        diff_type: if al == bl {
            DiffType::Same
        } else {
            DiffType::Changed
        },
    }
}

pub fn compare_timing(a: &HttpResponse, b: &HttpResponse) -> DiffResult {
    DiffResult {
        field: "response_time".into(),
        baseline_value: format!("{:?}", a.timing.total_duration),
        test_value: format!("{:?}", b.timing.total_duration),
        diff_type: DiffType::Changed,
    }
}

pub fn compare_all(a: &HttpResponse, b: &HttpResponse) -> Vec<DiffResult> {
    let mut diffs = vec![compare_status(a, b)];
    diffs.extend(compare_headers(a, b));
    diffs.push(compare_body(a, b));
    diffs.push(compare_length(a, b));
    diffs.push(compare_timing(a, b));
    diffs
}
