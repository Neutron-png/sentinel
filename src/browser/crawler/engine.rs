#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::crawler::events::{CrawlEvent, CrawlEventBus};
use crate::browser::crawler::models::{CrawlConfig, CrawlState, CrawlTask, DiscoveredPage};
use crate::browser::crawler::navigator;
use crate::browser::crawler::queue::CrawlQueue;
use crate::browser::crawler::scheduler::CrawlScheduler;
use crate::browser::dom::engine::DomEngine;

pub struct BrowserCrawlerEngine {
    config: CrawlConfig,
    queue: CrawlQueue,
    scheduler: CrawlScheduler,
    event_bus: CrawlEventBus,
    discovered_pages: Vec<DiscoveredPage>,
    dom: DomEngine,
}

impl BrowserCrawlerEngine {
    pub fn new(config: CrawlConfig) -> Self {
        Self {
            config: config.clone(),
            queue: CrawlQueue::new(10000),
            scheduler: CrawlScheduler::new(config),
            event_bus: CrawlEventBus::new(512),
            discovered_pages: Vec::new(),
            dom: DomEngine::new(),
        }
    }

    pub fn event_bus(&self) -> CrawlEventBus {
        self.event_bus.clone()
    }
    pub fn pages(&self) -> &[DiscoveredPage] {
        &self.discovered_pages
    }

    pub fn add_seed(&mut self, url: &str) {
        self.queue.enqueue(CrawlTask {
            id: Uuid::new_v4(),
            url: url.to_string(),
            depth: 0,
            source: "seed".into(),
        });
    }

    pub fn start(&mut self, browser_id: Uuid) {
        self.scheduler.start();
        self.event_bus.emit(CrawlEvent::CrawlStarted { browser_id });
    }

    pub fn stop(&mut self, browser_id: Uuid) {
        self.scheduler.stop();
        self.event_bus.emit(CrawlEvent::CrawlFinished {
            browser_id,
            pages: self.discovered_pages.len(),
        });
    }

    pub async fn crawl_next(&mut self) -> Option<DiscoveredPage> {
        let task = self.queue.dequeue()?;
        if !self.scheduler.should_crawl(&task) {
            return None;
        }

        self.scheduler.delay().await;
        let page = navigator::navigate_and_extract(&self.dom, &task);
        self.scheduler.page_crawled();

        // Emit discovery events
        for route in &page.routes {
            self.event_bus
                .emit(CrawlEvent::RouteDiscovered { url: route.clone() });
        }
        for api in &page.apis {
            self.event_bus.emit(CrawlEvent::EndpointDiscovered {
                url: api.clone(),
                method: "GET".into(),
            });
        }
        if page.forms > 0 {
            self.event_bus.emit(CrawlEvent::FormDiscovered {
                url: page.url.clone(),
                action: String::new(),
                fields: page.forms,
            });
        }

        // Enqueue newly discovered routes
        navigator::enqueue_discovered(&mut self.queue, &page, task.depth);
        self.discovered_pages.push(page.clone());
        Some(page)
    }

    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }
    pub fn visited_count(&self) -> usize {
        self.queue.visited().len()
    }
    pub fn state(&self) -> CrawlState {
        self.scheduler.state()
    }
}
