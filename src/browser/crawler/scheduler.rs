#![allow(dead_code)]

use tokio::time::sleep;

use crate::browser::crawler::models::{CrawlConfig, CrawlState, CrawlTask};

pub struct CrawlScheduler {
    config: CrawlConfig,
    state: CrawlState,
    pages_crawled: usize,
}

impl CrawlScheduler {
    pub fn new(config: CrawlConfig) -> Self {
        Self {
            config,
            state: CrawlState::Idle,
            pages_crawled: 0,
        }
    }

    pub fn start(&mut self) {
        self.state = CrawlState::Running;
    }
    pub fn stop(&mut self) {
        self.state = CrawlState::Completed;
    }
    pub fn state(&self) -> CrawlState {
        self.state
    }
    pub fn pages_crawled(&self) -> usize {
        self.pages_crawled
    }

    pub fn should_crawl(&self, task: &CrawlTask) -> bool {
        self.state == CrawlState::Running
            && self.pages_crawled < self.config.max_pages
            && task.depth <= self.config.max_depth
    }

    pub fn page_crawled(&mut self) {
        self.pages_crawled += 1;
    }

    pub async fn delay(&self) {
        sleep(self.config.request_delay).await;
    }
}
