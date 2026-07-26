#![allow(dead_code)]

use std::collections::HashSet;

use crate::network::models::HttpResponse;
use crate::scanner::analyzer::models::SimilarityScore;

pub fn calculate_similarity(a: &HttpResponse, b: &HttpResponse) -> SimilarityScore {
    let header_score = header_similarity(a, b);
    let body_score = body_similarity(a, b);
    let overall = header_score * 0.3 + body_score * 0.7;
    SimilarityScore {
        header_score,
        body_score,
        overall_score: overall,
    }
}

fn header_similarity(a: &HttpResponse, b: &HttpResponse) -> f64 {
    let a_set: HashSet<&str> = a.headers.iter().map(|h| h.name.as_str()).collect();
    let b_set: HashSet<&str> = b.headers.iter().map(|h| h.name.as_str()).collect();
    let intersect = a_set.intersection(&b_set).count();
    let union = a_set.union(&b_set).count();
    if union == 0 {
        return 1.0;
    }
    // For matching headers, compare values
    let mut val_match = 0;
    for key in a_set.intersection(&b_set) {
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
        if av == bv {
            val_match += 1;
        }
    }
    let struct_score = intersect as f64 / union as f64;
    let val_score = if intersect > 0 {
        val_match as f64 / intersect as f64
    } else {
        0.0
    };
    (struct_score + val_score) / 2.0
}

fn body_similarity(a: &HttpResponse, b: &HttpResponse) -> f64 {
    let a_body = a.body.as_text().unwrap_or("");
    let b_body = b.body.as_text().unwrap_or("");
    if a_body.is_empty() && b_body.is_empty() {
        return 1.0;
    }
    if a_body.is_empty() || b_body.is_empty() {
        return 0.0;
    }
    let a_words: HashSet<&str> = a_body.split_whitespace().collect();
    let b_words: HashSet<&str> = b_body.split_whitespace().collect();
    let intersect = a_words.intersection(&b_words).count();
    let union = a_words.union(&b_words).count();
    if union == 0 {
        return 1.0;
    }
    intersect as f64 / union as f64
}
