#![allow(dead_code)]

pub struct Normalizer;

impl Normalizer {
    pub fn normalize_url(url: &str) -> String {
        url.to_lowercase()
            .trim_end_matches('/')
            .split('?')
            .next()
            .unwrap_or(url)
            .to_string()
    }

    pub fn normalize_param(_param: &str) -> String {
        String::new()
    }
    pub fn normalize_host(host: &str) -> String {
        host.to_lowercase()
    }
}
