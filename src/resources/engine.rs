#![allow(dead_code)]

use crate::resources::circuit_breaker::CircuitBreaker;
use crate::resources::concurrency::ConcurrencyController;
use crate::resources::errors::ResourceError;
use crate::resources::events::{ResourceEvent, ResourceEventBus};
use crate::resources::metrics::MetricsCollector;
use crate::resources::models::{HealthMetrics, RateLimitConfig, RetryPolicy, TimeoutConfig};
use crate::resources::rate_limit::RateLimiter;
use crate::resources::retry;

pub struct ResourceEngine {
    config: RateLimitConfig,
    rate_limiter: RateLimiter,
    concurrency: ConcurrencyController,
    circuit_breaker: CircuitBreaker,
    metrics: MetricsCollector,
    event_bus: ResourceEventBus,
    timeout: TimeoutConfig,
    retry_policy: RetryPolicy,
}

impl ResourceEngine {
    pub fn new() -> Self {
        Self {
            config: RateLimitConfig::default(),
            rate_limiter: RateLimiter::new(10.0),
            concurrency: ConcurrencyController::new(),
            circuit_breaker: CircuitBreaker::new(5, 3, 30),
            metrics: MetricsCollector::new(),
            event_bus: ResourceEventBus::new(256),
            timeout: TimeoutConfig::default(),
            retry_policy: RetryPolicy::default(),
        }
    }

    pub fn event_bus(&self) -> ResourceEventBus {
        self.event_bus.clone()
    }
    pub fn set_config(&mut self, config: RateLimitConfig) {
        let rps = config.requests_per_second;
        self.config = config;
        self.rate_limiter = RateLimiter::new(rps as f64);
    }

    // ── Rate limiting ──
    pub fn try_request(&mut self) -> bool {
        if !self.rate_limiter.try_acquire() {
            self.event_bus.emit(ResourceEvent::RateLimited {
                reason: "Rate limit reached".into(),
            });
            return false;
        }
        true
    }

    // ── Concurrency ──
    pub fn can_issue_request(&self, host: Option<&str>) -> bool {
        let ok = self.concurrency.can_request(&self.config, host);
        if !ok {
            self.event_bus.emit(ResourceEvent::ConcurrencyThrottled {
                current: self.concurrency.active_requests(),
                max: self.config.max_concurrent_requests,
            });
        }
        ok
    }

    pub fn acquire_request(&mut self, host: Option<&str>) {
        self.concurrency.acquire_request(host);
    }
    pub fn release_request(&mut self, host: Option<&str>) {
        self.concurrency.release_request(host);
    }

    // ── Circuit breaker ──
    pub fn check_circuit(&self) -> Result<(), ResourceError> {
        if self.circuit_breaker.is_open() {
            return Err(ResourceError::CircuitOpen("Circuit is open".into()));
        }
        Ok(())
    }
    pub fn record_success(&mut self) {
        self.circuit_breaker.record_success();
    }
    pub fn record_failure(&mut self) {
        self.circuit_breaker.record_failure();
        self.event_bus.emit(ResourceEvent::CircuitOpened {
            reason: "Failure threshold reached".into(),
        });
    }

    // ── Retry ──
    pub fn should_retry(&self, attempt: u32) -> bool {
        retry::should_retry(&self.retry_policy, attempt)
    }
    pub fn retry_delay(&self, attempt: u32) -> u64 {
        retry::compute_delay(&self.retry_policy, attempt)
    }

    // ── Metrics ──
    pub fn record_metric(&mut self, response_ms: u64, success: bool) {
        self.metrics.record_request(response_ms, success);
    }
    pub fn health(&self) -> HealthMetrics {
        self.metrics.snapshot(
            self.concurrency.active_requests(),
            0,
            self.circuit_breaker.state(),
        )
    }

    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
    pub fn timeout(&self) -> &TimeoutConfig {
        &self.timeout
    }
}
