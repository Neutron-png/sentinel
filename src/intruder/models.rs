#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadSet {
    List(Vec<String>),
    Numeric { start: i64, end: i64, step: i64 },
    Charset { alphabet: String, min_len: usize, max_len: usize },
    File { path: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadEncoding {
    None,
    Url,
    Base64,
    Hex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackMode {
    Sniper,
    Pitchfork,
    ClusterBomb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntruderConfig {
    pub mode: AttackMode,
    pub encoding: PayloadEncoding,
    pub sets: Vec<PayloadSet>,
    pub concurrency: usize,
    pub rate_limit_rps: f64,
    pub max_requests: u64,
    pub request_timeout: Duration,
}

impl Default for IntruderConfig {
    fn default() -> Self {
        Self {
            mode: AttackMode::Sniper,
            encoding: PayloadEncoding::None,
            sets: Vec::new(),
            concurrency: 8,
            rate_limit_rps: 50.0,
            max_requests: 10_000,
            request_timeout: Duration::from_secs(15),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseMetrics {
    pub status_code: u16,
    pub body_length: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuzzResult {
    pub index: u64,
    pub payloads: Vec<String>,
    pub metrics: ResponseMetrics,
    pub anomaly: bool,
}

#[derive(Debug, Clone)]
pub struct AnalysisBaseline {
    pub median_status: u16,
    pub median_length: usize,
    pub median_duration_ms: u64,
}

impl AnalysisBaseline {
    pub fn from_results(results: &[ResponseMetrics]) -> Self {
        if results.is_empty() {
            return Self {
                median_status: 0,
                median_length: 0,
                median_duration_ms: 0,
            };
        }
        let mut statuses: Vec<u16> = results.iter().map(|m| m.status_code).collect();
        let mut lengths: Vec<usize> = results.iter().map(|m| m.body_length).collect();
        let mut durations: Vec<u64> = results.iter().map(|m| m.duration.as_millis() as u64).collect();
        statuses.sort_unstable();
        lengths.sort_unstable();
        durations.sort_unstable();
        Self {
            median_status: median(&statuses),
            median_length: median(&lengths),
            median_duration_ms: median(&durations),
        }
    }

    pub fn is_anomaly(&self, m: &ResponseMetrics) -> bool {
        if self.median_status == 0 {
            return false;
        }
        let status_class_change = m.status_code / 100 != self.median_status / 100;
        let length_tolerance = (self.median_length / 10).max(32);
        let timed_out = self.median_duration_ms > 0
            && m.duration.as_millis() as u64 > self.median_duration_ms.saturating_mul(3);
        status_class_change
            || m.body_length.abs_diff(self.median_length) > length_tolerance
            || timed_out
    }
}

#[inline]
fn median<T: Copy>(v: &[T]) -> T {
    *v.get((v.len().saturating_sub(1)) / 2)
        .unwrap_or_else(|| unreachable!("caller guarantees non-empty"))
}

#[derive(Debug, Clone, Copy)]
pub struct InsertionPoint {
    pub start: usize,
    pub end: usize,
}
