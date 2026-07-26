#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub rules: RuleControl,
    pub limits: ScanLimits,
    pub payloads: PayloadConfig,
    pub technology_filters: Vec<String>,
}

impl ScanPolicy {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            version: "1.0".into(),
            author: String::new(),
            rules: RuleControl::default(),
            limits: ScanLimits::default(),
            payloads: PayloadConfig::default(),
            technology_filters: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleControl {
    pub enabled_rules: Vec<String>,
    pub disabled_rules: Vec<String>,
    pub rule_groups: Vec<RuleGroup>,
    pub rule_overrides: Vec<RuleOverride>,
    pub default_priority: u8,
}

impl Default for RuleControl {
    fn default() -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            rule_groups: vec![],
            rule_overrides: vec![],
            default_priority: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGroup {
    pub name: String,
    pub rule_ids: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOverride {
    pub rule_id: String,
    pub priority: Option<u8>,
    pub max_requests: Option<usize>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanLimits {
    pub max_requests: usize,
    pub max_depth: usize,
    pub max_crawl_time_secs: u64,
    pub max_scan_time_secs: u64,
    pub request_delay_ms: u64,
    pub concurrent_requests: usize,
    pub retry_count: u32,
    pub timeout_secs: u64,
}

impl Default for ScanLimits {
    fn default() -> Self {
        Self {
            max_requests: 1000,
            max_depth: 5,
            max_crawl_time_secs: 600,
            max_scan_time_secs: 1800,
            request_delay_ms: 100,
            concurrent_requests: 4,
            retry_count: 2,
            timeout_secs: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadConfig {
    pub categories: Vec<String>,
    pub max_payload_size: usize,
    pub enabled_mutations: Vec<String>,
    pub encoding_strategies: Vec<String>,
}

impl Default for PayloadConfig {
    fn default() -> Self {
        Self {
            categories: vec![
                "SQL Injection".into(),
                "XSS".into(),
                "Path Traversal".into(),
            ],
            max_payload_size: 4096,
            enabled_mutations: vec!["prefix".into(), "suffix".into()],
            encoding_strategies: vec!["url".into(), "base64".into()],
        }
    }
}
