#![allow(dead_code)]

use crate::resources::models::{RetryPolicy, RetryStrategy};

pub fn compute_delay(policy: &RetryPolicy, attempt: u32) -> u64 {
    let base = match policy.strategy {
        RetryStrategy::Fixed => policy.base_delay_ms,
        RetryStrategy::Linear => policy.base_delay_ms * (attempt + 1) as u64,
        RetryStrategy::Exponential => policy.base_delay_ms * 2u64.pow(attempt),
    };
    if policy.jitter_ms > 0 {
        base + (rand::random::<u64>() % policy.jitter_ms)
    } else {
        base
    }
}

pub fn should_retry(policy: &RetryPolicy, attempt: u32) -> bool {
    attempt < policy.max_retries
}
