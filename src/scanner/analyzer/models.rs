#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub field: String,
    pub baseline_value: String,
    pub test_value: String,
    pub diff_type: DiffType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffType {
    Same,
    Changed,
    Added,
    Removed,
}

#[derive(Debug, Clone)]
pub struct ReflectionResult {
    pub found: bool,
    pub reflection_type: ReflectionType,
    pub offset: usize,
    pub length: usize,
    pub payload: String,
    pub context: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionType {
    Full,
    Partial,
    Encoded,
    Html,
    Json,
    None,
}

#[derive(Debug, Clone)]
pub struct SimilarityScore {
    pub header_score: f64,
    pub body_score: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone)]
pub struct TimingStats {
    pub baseline: Duration,
    pub average: Duration,
    pub median: Duration,
    pub stddev: f64,
    pub samples: Vec<Duration>,
}

impl TimingStats {
    pub fn from_samples(mut samples: Vec<Duration>) -> Self {
        let baseline = samples.first().copied().unwrap_or_default();
        let total: Duration = samples.iter().sum();
        let avg = if samples.is_empty() {
            Duration::ZERO
        } else {
            total / samples.len() as u32
        };
        samples.sort();
        let median = *samples.get(samples.len() / 2).unwrap_or(&Duration::ZERO);
        let mean_ms = avg.as_secs_f64() * 1000.0;
        let variance: f64 = samples
            .iter()
            .map(|d| {
                let diff = d.as_secs_f64() * 1000.0 - mean_ms;
                diff * diff
            })
            .sum::<f64>()
            / samples.len().max(1) as f64;
        let stddev = variance.sqrt();
        Self {
            baseline,
            average: avg,
            median,
            stddev,
            samples,
        }
    }
}
