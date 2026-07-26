#![allow(dead_code)]

use crate::resources::models::RateLimitConfig;

pub struct ConcurrencyController {
    active_requests: usize,
    active_hosts: std::collections::HashMap<String, usize>,
    active_scanners: usize,
}

impl ConcurrencyController {
    pub fn new() -> Self {
        Self {
            active_requests: 0,
            active_hosts: std::collections::HashMap::new(),
            active_scanners: 0,
        }
    }

    pub fn can_request(&self, config: &RateLimitConfig, host: Option<&str>) -> bool {
        if self.active_requests >= config.max_concurrent_requests {
            return false;
        }
        if let Some(h) = host {
            let count = self.active_hosts.get(h).copied().unwrap_or(0);
            if count >= config.max_concurrent_hosts {
                return false;
            }
        }
        true
    }

    pub fn acquire_request(&mut self, host: Option<&str>) {
        self.active_requests += 1;
        if let Some(h) = host {
            *self.active_hosts.entry(h.to_string()).or_default() += 1;
        }
    }

    pub fn release_request(&mut self, host: Option<&str>) {
        self.active_requests = self.active_requests.saturating_sub(1);
        if let Some(h) = host {
            if let Some(count) = self.active_hosts.get_mut(h) {
                *count = count.saturating_sub(1);
            }
        }
    }

    pub fn can_start_scanner(&self, config: &RateLimitConfig) -> bool {
        self.active_scanners < config.max_concurrent_scanners
    }
    pub fn acquire_scanner(&mut self) {
        self.active_scanners += 1;
    }
    pub fn release_scanner(&mut self) {
        self.active_scanners = self.active_scanners.saturating_sub(1);
    }
    pub fn active_requests(&self) -> usize {
        self.active_requests
    }
}
