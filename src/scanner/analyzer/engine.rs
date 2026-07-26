#![allow(dead_code)]

use std::time::Duration;

use crate::network::models::HttpResponse;
use crate::scanner::analyzer::compare;
use crate::scanner::analyzer::matcher;
use crate::scanner::analyzer::models::{
    DiffResult, ReflectionResult, SimilarityScore, TimingStats,
};
use crate::scanner::analyzer::reflection;
use crate::scanner::analyzer::similarity;
use crate::scanner::analyzer::timing::TimingAnalyzer;

pub struct ResponseAnalyzer {
    timing: TimingAnalyzer,
}

impl ResponseAnalyzer {
    pub fn new(threshold_ms: u64) -> Self {
        Self {
            timing: TimingAnalyzer::new(threshold_ms),
        }
    }

    pub fn add_timing_sample(&mut self, duration: Duration) {
        self.timing.add_sample(duration);
    }
    pub fn timing_stats(&self) -> TimingStats {
        self.timing.stats()
    }
    pub fn is_anomalous_timing(&self, duration: Duration) -> bool {
        self.timing.is_anomalous(duration)
    }

    pub fn compare(&self, a: &HttpResponse, b: &HttpResponse) -> Vec<DiffResult> {
        compare::compare_all(a, b)
    }
    pub fn compare_status(&self, a: &HttpResponse, b: &HttpResponse) -> DiffResult {
        compare::compare_status(a, b)
    }
    pub fn compare_length(&self, a: &HttpResponse, b: &HttpResponse) -> DiffResult {
        compare::compare_length(a, b)
    }

    pub fn similarity(&self, a: &HttpResponse, b: &HttpResponse) -> SimilarityScore {
        similarity::calculate_similarity(a, b)
    }

    pub fn detect_reflection(&self, response: &HttpResponse, payload: &str) -> ReflectionResult {
        reflection::detect_reflection(response, payload)
    }

    pub fn has_reflection(&self, response: &HttpResponse, payload: &str) -> bool {
        self.detect_reflection(response, payload).found
    }

    pub fn count_keywords(&self, text: &str, keywords: &[&str]) -> Vec<matcher::KeywordMatch> {
        matcher::count_keywords(text, keywords)
    }

    pub fn match_pattern(&self, text: &str, pattern: &str) -> bool {
        matcher::match_regex(text, pattern)
    }
}
