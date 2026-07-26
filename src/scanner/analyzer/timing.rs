#![allow(dead_code)]

use std::time::Duration;

use crate::scanner::analyzer::models::TimingStats;

#[derive(Debug, Clone)]
pub struct TimingAnalyzer {
    pub threshold_ms: u64,
    pub samples: Vec<Duration>,
}

impl TimingAnalyzer {
    pub fn new(threshold_ms: u64) -> Self {
        Self {
            threshold_ms,
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, duration: Duration) {
        self.samples.push(duration);
    }

    pub fn stats(&self) -> TimingStats {
        TimingStats::from_samples(self.samples.clone())
    }

    pub fn is_anomalous(&self, duration: Duration) -> bool {
        if self.samples.len() < 3 {
            return false;
        }
        let stats = self.stats();
        let test_ms = duration.as_millis() as f64;
        let mean_ms = stats.average.as_millis() as f64;
        if stats.stddev < 1.0 {
            return test_ms > mean_ms + self.threshold_ms as f64;
        }
        let z_score = (test_ms - mean_ms) / stats.stddev;
        z_score > 3.0
    }

    pub fn difference_from_baseline(&self, duration: Duration) -> Duration {
        let baseline = self.samples.first().copied().unwrap_or_default();
        duration.abs_diff(baseline)
    }
}
