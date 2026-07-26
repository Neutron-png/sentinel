#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

use crate::crawler::models::DiscoveredUrl;

pub struct CrawlQueue {
    queue: VecDeque<DiscoveredUrl>,
    visited: HashSet<String>,
    max_size: usize,
}

impl CrawlQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            max_size,
        }
    }

    pub fn push(&mut self, url: DiscoveredUrl) -> bool {
        let normalized = normalize_key(&url.url);
        if self.visited.contains(&normalized) || self.queue.len() >= self.max_size {
            return false;
        }
        self.visited.insert(normalized);
        self.queue.push_back(url);
        true
    }

    pub fn pop(&mut self) -> Option<DiscoveredUrl> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn visited_count(&self) -> usize {
        self.visited.len()
    }
    pub fn visited_urls(&self) -> &HashSet<String> {
        &self.visited
    }
}

fn normalize_key(url: &str) -> String {
    let s = url.split('#').next().unwrap_or(url);
    s.trim_end_matches('/').to_lowercase()
}
