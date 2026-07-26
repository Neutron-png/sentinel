#![allow(dead_code)]

use crate::resources::models::RateLimitConfig;

#[derive(Clone)]
pub struct RateLimiter {
    bucket: f64,
    last_refill: std::time::Instant,
    rate: f64,
}

impl RateLimiter {
    pub fn new(rate_per_sec: f64) -> Self {
        Self {
            bucket: rate_per_sec,
            last_refill: std::time::Instant::now(),
            rate: rate_per_sec,
        }
    }

    pub fn try_acquire(&mut self) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.bucket = (self.bucket + elapsed * self.rate).min(self.rate);
        self.last_refill = now;
        if self.bucket >= 1.0 {
            self.bucket -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn check_limits(config: &RateLimitConfig, current_rps: u32, current_rpm: u32) -> bool {
        current_rps < config.requests_per_second && current_rpm < config.requests_per_minute
    }
}
