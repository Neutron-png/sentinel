#![allow(dead_code)]

use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::sleep;

use crate::crawler::models::{CrawlConfig, CrawlStatus, DiscoveredUrl};
use crate::crawler::queue::CrawlQueue;

pub struct CrawlScheduler {
    config: CrawlConfig,
    status: CrawlStatus,
    pages_crawled: usize,
    semaphore: Arc<Semaphore>,
}

impl CrawlScheduler {
    pub fn new(config: CrawlConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        Self {
            config,
            status: CrawlStatus::Idle,
            pages_crawled: 0,
            semaphore,
        }
    }

    pub fn start(&mut self) {
        self.status = CrawlStatus::Running;
    }
    pub fn pause(&mut self) {
        self.status = CrawlStatus::Paused;
    }
    pub fn stop(&mut self) {
        self.status = CrawlStatus::Completed;
    }

    pub fn status(&self) -> CrawlStatus {
        self.status
    }
    pub fn is_running(&self) -> bool {
        self.status == CrawlStatus::Running
    }
    pub fn pages_crawled(&self) -> usize {
        self.pages_crawled
    }

    pub fn should_crawl(&self, url: &DiscoveredUrl) -> bool {
        self.is_running()
            && self.pages_crawled < self.config.max_pages
            && url.depth <= self.config.max_depth
    }

    pub fn page_crawled(&mut self) {
        self.pages_crawled += 1;
    }
    pub fn semaphore(&self) -> Arc<Semaphore> {
        self.semaphore.clone()
    }

    pub async fn delay(&self) {
        sleep(self.config.request_delay).await;
    }

    pub fn queue_full(&self, queue: &CrawlQueue) -> bool {
        self.pages_crawled + queue.len() >= self.config.max_pages
    }
}
