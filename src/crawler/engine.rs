#![allow(dead_code)]

use std::collections::HashSet;
use std::sync::Arc;

use crate::crawler::models::{CrawlConfig, CrawlStatus, DiscoveredUrl, SeedUrl};
use crate::crawler::parser::ContentParser;
use crate::crawler::queue::CrawlQueue;
use crate::crawler::scheduler::CrawlScheduler;
use crate::network::client::HttpClient;
use crate::network::models::HttpBody;

pub struct CrawlerEngine {
    client: Arc<HttpClient>,
    config: CrawlConfig,
    queue: CrawlQueue,
    scheduler: CrawlScheduler,
    seeds: Vec<SeedUrl>,
}

impl CrawlerEngine {
    pub fn new(client: Arc<HttpClient>, config: CrawlConfig) -> Self {
        Self {
            client,
            scheduler: CrawlScheduler::new(config.clone()),
            config,
            queue: CrawlQueue::new(10000),
            seeds: Vec::new(),
        }
    }

    pub fn add_seed(&mut self, url: &str) {
        self.seeds.push(SeedUrl {
            url: url.to_string(),
            method: "GET".into(),
            depth: 0,
        });
        self.queue.push(DiscoveredUrl {
            url: url.to_string(),
            method: "GET".into(),
            source_url: url.to_string(),
            depth: 0,
            element: "seed".into(),
        });
    }

    pub fn status(&self) -> CrawlStatus {
        self.scheduler.status()
    }
    pub fn pages_crawled(&self) -> usize {
        self.scheduler.pages_crawled()
    }
    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }
    pub fn visited_count(&self) -> usize {
        self.queue.visited_count()
    }

    pub fn start(&mut self) {
        self.scheduler.start();
    }
    pub fn pause(&mut self) {
        self.scheduler.pause();
    }
    pub fn stop(&mut self) {
        self.scheduler.stop();
    }

    pub async fn run(&mut self) -> Vec<(DiscoveredUrl, Option<String>)> {
        self.scheduler.start();
        let mut results = Vec::new();
        let semaphore = self.scheduler.semaphore();

        while let Some(url_entry) = self.queue.pop() {
            if !self.scheduler.should_crawl(&url_entry) {
                break;
            }
            self.scheduler.delay().await;

            let _permit = semaphore.acquire().await.unwrap();
            let client = self.client.clone();

            let entry = url_entry.clone();
            let result = tokio::spawn(async move {
                let resp = client
                    .execute(crate::network::models::HttpRequest {
                        method: entry.method.clone(),
                        url: entry.url.clone(),
                        headers: vec![],
                        cookies: vec![],
                        body: HttpBody::Empty,
                        query_params: vec![],
                    })
                    .await;
                (
                    entry,
                    resp.ok()
                        .map(|r| r.body.as_text().unwrap_or("").to_string()),
                )
            })
            .await
            .unwrap_or((url_entry, None));

            self.scheduler.page_crawled();
            results.push(result);
        }

        self.scheduler.stop();
        results
    }

    pub fn discover_links(
        &mut self,
        body: &str,
        content_type: &str,
        base_url: &str,
        depth: usize,
    ) -> Vec<DiscoveredUrl> {
        if depth >= self.config.max_depth || self.scheduler.queue_full(&self.queue) {
            return vec![];
        }
        let urls = ContentParser::parse(body, content_type, base_url, depth);
        let mut added = Vec::new();
        for url in urls {
            if self.queue.push(url.clone()) {
                added.push(url);
            }
        }
        added
    }

    pub fn visited(&self) -> &HashSet<String> {
        self.queue.visited_urls()
    }
}
