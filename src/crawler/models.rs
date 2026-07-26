#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub max_depth: usize,
    pub max_pages: usize,
    pub request_delay: Duration,
    pub max_concurrent: usize,
    pub follow_redirects: bool,
    pub parse_robots: bool,
    pub parse_sitemap: bool,
    pub extract_js_urls: bool,
    pub respect_scope: bool,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            max_pages: 500,
            request_delay: Duration::from_millis(200),
            max_concurrent: 5,
            follow_redirects: true,
            parse_robots: true,
            parse_sitemap: true,
            extract_js_urls: true,
            respect_scope: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrawlStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Error,
}

#[derive(Debug, Clone)]
pub struct SeedUrl {
    pub url: String,
    pub method: String,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct DiscoveredUrl {
    pub url: String,
    pub method: String,
    pub source_url: String,
    pub depth: usize,
    pub element: String,
}
