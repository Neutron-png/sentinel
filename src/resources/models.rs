#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub requests_per_minute: u32,
    pub max_concurrent_requests: usize,
    pub max_concurrent_hosts: usize,
    pub max_concurrent_scanners: usize,
    pub max_browser_tabs: usize,
    pub max_browser_sessions: usize,
    pub max_workflow_executions: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            requests_per_minute: 300,
            max_concurrent_requests: 20,
            max_concurrent_hosts: 10,
            max_concurrent_scanners: 3,
            max_browser_tabs: 10,
            max_browser_sessions: 2,
            max_workflow_executions: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub strategy: RetryStrategy,
    pub base_delay_ms: u64,
    pub jitter_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            strategy: RetryStrategy::Exponential,
            base_delay_ms: 500,
            jitter_ms: 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryStrategy {
    Fixed,
    Linear,
    Exponential,
}

#[derive(Debug, Clone)]
pub struct HealthMetrics {
    pub active_workers: usize,
    pub queue_length: usize,
    pub avg_response_ms: f64,
    pub error_rate: f64,
    pub retry_count: usize,
    pub success_rate: f64,
    pub circuit_state: CircuitState,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            active_workers: 0,
            queue_length: 0,
            avg_response_ms: 0.0,
            error_rate: 0.0,
            retry_count: 0,
            success_rate: 1.0,
            circuit_state: CircuitState::Closed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connect_secs: u64,
    pub read_secs: u64,
    pub write_secs: u64,
    pub idle_secs: u64,
    pub scan_secs: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect_secs: 10,
            read_secs: 30,
            write_secs: 30,
            idle_secs: 60,
            scan_secs: 1800,
        }
    }
}
