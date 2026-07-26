#![allow(dead_code)]

use crate::resources::models::CircuitState;

pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    failure_threshold: usize,
    success_threshold: usize,
    cooldown_secs: u64,
    last_failure: Option<std::time::Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, success_threshold: usize, cooldown_secs: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            failure_threshold,
            success_threshold,
            cooldown_secs,
            last_failure: None,
        }
    }

    pub fn state(&self) -> CircuitState {
        if self.state == CircuitState::Open {
            if let Some(t) = self.last_failure {
                if t.elapsed().as_secs() >= self.cooldown_secs {
                    return CircuitState::HalfOpen;
                }
            }
        }
        self.state
    }

    pub fn record_success(&mut self) {
        self.success_count += 1;
        if self.state == CircuitState::HalfOpen && self.success_count >= self.success_threshold {
            self.state = CircuitState::Closed;
            self.failure_count = 0;
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(std::time::Instant::now());
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }

    pub fn is_open(&self) -> bool {
        self.state() == CircuitState::Open
    }
}
