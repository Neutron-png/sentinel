#![allow(dead_code)]

use crate::browser::crawler::models::{CrawlTask, DiscoveredPage};
use crate::browser::crawler::queue::CrawlQueue;
use crate::browser::dom::engine::DomEngine;

pub fn navigate_and_extract(dom: &DomEngine, task: &CrawlTask) -> DiscoveredPage {
    let title = dom.get_title();
    let routes = crate::browser::crawler::extractor::PageExtractor::extract_routes(dom, &task.url);
    let apis: Vec<String> = crate::browser::crawler::extractor::PageExtractor::extract_apis(dom)
        .into_iter()
        .map(|(u, _)| u)
        .collect();
    let forms = crate::browser::crawler::extractor::PageExtractor::extract_forms(dom);

    DiscoveredPage {
        url: task.url.clone(),
        title,
        routes,
        forms,
        links: 0,
        apis,
    }
}

pub fn enqueue_discovered(queue: &mut CrawlQueue, page: &DiscoveredPage, depth: usize) -> usize {
    let mut count = 0;
    for route in &page.routes {
        if queue.enqueue(CrawlTask {
            id: uuid::Uuid::new_v4(),
            url: route.clone(),
            depth: depth + 1,
            source: page.url.clone(),
        }) {
            count += 1;
        }
    }
    count
}
