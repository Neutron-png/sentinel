#![allow(dead_code)]

use crate::browser::crawler::normalizer::normalize_url;
use crate::browser::dom::engine::DomEngine;
use crate::browser::dom::models::LinkInfo;

pub struct PageExtractor;

impl PageExtractor {
    pub fn extract_routes(dom: &DomEngine, base_url: &str) -> Vec<String> {
        let links = dom.extract_links();
        let scripts = dom.extract_scripts();
        let all: Vec<&LinkInfo> = links.iter().chain(scripts.iter()).collect();
        all.iter()
            .filter_map(|l| normalize_url(&l.url, base_url))
            .filter(|u| u.starts_with("http"))
            .collect()
    }

    pub fn extract_apis(dom: &DomEngine) -> Vec<(String, String)> {
        let scripts = dom.extract_scripts();
        let mut apis = Vec::new();
        for s in scripts {
            if s.url.contains("/api/") || s.url.contains("fetch") {
                apis.push((s.url.clone(), "GET".into()));
            }
        }
        apis
    }

    pub fn extract_forms(dom: &DomEngine) -> usize {
        dom.discover_forms().len()
    }
}
