#![allow(dead_code)]

use crate::policy::models::PayloadConfig;

pub struct PayloadPolicy;

impl PayloadPolicy {
    pub fn allowed_category(config: &PayloadConfig, category: &str) -> bool {
        config.categories.is_empty() || config.categories.contains(&category.to_string())
    }

    pub fn size_ok(config: &PayloadConfig, size: usize) -> bool {
        size <= config.max_payload_size
    }

    pub fn encoding_allowed(config: &PayloadConfig, strategy: &str) -> bool {
        config.encoding_strategies.is_empty()
            || config.encoding_strategies.contains(&strategy.to_string())
    }
}
