#![allow(dead_code)]

use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub max_depth: usize,
    pub max_pages: usize,
    pub request_delay: Duration,
    pub follow_links: bool,
    pub submit_forms: bool,
    pub discover_apis: bool,
    pub same_origin_only: bool,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            max_pages: 200,
            request_delay: Duration::from_millis(500),
            follow_links: true,
            submit_forms: false,
            discover_apis: true,
            same_origin_only: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrawlState {
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Debug, Clone)]
pub struct CrawlTask {
    pub id: Uuid,
    pub url: String,
    pub depth: usize,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct DiscoveredPage {
    pub url: String,
    pub title: String,
    pub routes: Vec<String>,
    pub forms: usize,
    pub links: usize,
    pub apis: Vec<String>,
}
