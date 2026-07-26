#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub delay: Duration,
    pub backoff_multiplier: f64,
    pub retry_on_timeout: bool,
    pub retry_on_network_error: bool,
    pub retry_on_http_5xx: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            delay: Duration::from_millis(500),
            backoff_multiplier: 2.0,
            retry_on_timeout: true,
            retry_on_network_error: true,
            retry_on_http_5xx: false,
        }
    }
}

impl RetryPolicy {
    pub fn should_retry(&self, attempt: u32, error: &super::errors::PipelineError) -> bool {
        if attempt >= self.max_retries {
            return false;
        }
        match error {
            super::errors::PipelineError::Timeout(_) => self.retry_on_timeout,
            super::errors::PipelineError::Network(_) => self.retry_on_network_error,
            super::errors::PipelineError::Request(e) if e.contains("5") => self.retry_on_http_5xx,
            _ => false,
        }
    }

    pub fn delay_for(&self, attempt: u32) -> Duration {
        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        Duration::from_secs_f64(self.delay.as_secs_f64() * multiplier)
    }
}
