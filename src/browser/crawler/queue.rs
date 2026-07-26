#![allow(dead_code)]

use std::collections::HashSet;
use url::Url;

use crate::browser::crawler::models::CrawlTask;

pub struct CrawlQueue {
    queue: Vec<CrawlTask>,
    visited: HashSet<String>,
    max_size: usize,
}

impl CrawlQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: Vec::new(),
            visited: HashSet::new(),
            max_size,
        }
    }

    pub fn enqueue(&mut self, task: CrawlTask) -> bool {
        let norm = normalize_for_queue(&task.url);
        if self.visited.contains(&norm) || self.queue.len() >= self.max_size {
            return false;
        }
        self.visited.insert(norm);
        self.queue.push(task);
        true
    }

    pub fn dequeue(&mut self) -> Option<CrawlTask> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn visited(&self) -> &HashSet<String> {
        &self.visited
    }
}

fn normalize_for_queue(url: &str) -> String {
    let url = url.split('#').next().unwrap_or(url);
    if let Ok(parsed) = Url::parse(url) {
        let mut s = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or(""));
        if let Some(port) = parsed.port() {
            s.push_str(&format!(":{}", port));
        }
        s.push_str(parsed.path().trim_end_matches('/'));
        s
    } else {
        url.trim_end_matches('/').to_lowercase()
    }
}
