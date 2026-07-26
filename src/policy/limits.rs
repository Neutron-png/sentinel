#![allow(dead_code)]

use crate::policy::models::ScanLimits;

pub struct LimitEnforcer;

impl LimitEnforcer {
    pub fn should_stop(
        limits: &ScanLimits,
        requests_sent: usize,
        depth: usize,
        elapsed_secs: u64,
    ) -> bool {
        requests_sent >= limits.max_requests
            || depth > limits.max_depth
            || elapsed_secs > limits.max_scan_time_secs
    }

    pub fn can_crawl(limits: &ScanLimits, depth: usize, elapsed: u64) -> bool {
        depth <= limits.max_depth && elapsed <= limits.max_crawl_time_secs
    }

    pub fn delay_ms(limits: &ScanLimits) -> u64 {
        limits.request_delay_ms
    }
    pub fn max_concurrent(limits: &ScanLimits) -> usize {
        limits.concurrent_requests
    }
    pub fn retry_count(limits: &ScanLimits) -> u32 {
        limits.retry_count
    }
}
