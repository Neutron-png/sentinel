#![allow(dead_code)]

use crate::resources::models::HealthMetrics;

#[derive(Default)]
pub struct MetricsCollector {
    total_requests: usize,
    total_errors: usize,
    total_retries: usize,
    response_times_ms: Vec<u64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_request(&mut self, response_ms: u64, success: bool) {
        self.total_requests += 1;
        self.response_times_ms.push(response_ms);
        if !success {
            self.total_errors += 1;
        }
    }

    pub fn record_retry(&mut self) {
        self.total_retries += 1;
    }

    pub fn snapshot(
        &self,
        active_workers: usize,
        queue_length: usize,
        circuit_state: super::models::CircuitState,
    ) -> HealthMetrics {
        let total = self.total_requests.max(1) as f64;
        HealthMetrics {
            active_workers,
            queue_length,
            avg_response_ms: if self.response_times_ms.is_empty() {
                0.0
            } else {
                self.response_times_ms.iter().sum::<u64>() as f64
                    / self.response_times_ms.len() as f64
            },
            error_rate: self.total_errors as f64 / total,
            retry_count: self.total_retries,
            success_rate: (total - self.total_errors as f64) / total,
            circuit_state,
        }
    }
}
