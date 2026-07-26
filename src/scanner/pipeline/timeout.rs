#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub request_timeout: Duration,
    pub rule_timeout: Duration,
    pub task_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            request_timeout: Duration::from_secs(30),
            rule_timeout: Duration::from_secs(300),
            task_timeout: Duration::from_secs(600),
        }
    }
}
