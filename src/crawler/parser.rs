#![allow(dead_code)]

use crate::crawler::extractor::UrlExtractor;
use crate::crawler::models::DiscoveredUrl;
use crate::crawler::normalizer::UrlNormalizer;

pub struct ContentParser;

impl ContentParser {
    pub fn parse(
        body: &str,
        content_type: &str,
        base_url: &str,
        depth: usize,
    ) -> Vec<DiscoveredUrl> {
        if UrlNormalizer::is_html_content(content_type) {
            Self::parse_html(body, base_url, depth)
        } else if UrlNormalizer::is_json_content(content_type) {
            Self::parse_json(body, base_url, depth)
        } else if UrlNormalizer::is_xml_content(content_type) {
            Self::parse_xml(body, base_url, depth)
        } else if UrlNormalizer::is_js_content(content_type) {
            Self::parse_js(body, base_url, depth)
        } else if UrlNormalizer::is_css_content(content_type) {
            Self::parse_css(body, base_url, depth)
        } else {
            vec![]
        }
    }

    fn parse_html(body: &str, base: &str, depth: usize) -> Vec<DiscoveredUrl> {
        UrlExtractor::from_html(body, base)
            .into_iter()
            .map(|(url, element)| DiscoveredUrl {
                url,
                method: "GET".into(),
                source_url: base.into(),
                depth: depth + 1,
                element,
            })
            .collect()
    }

    fn parse_json(body: &str, base: &str, depth: usize) -> Vec<DiscoveredUrl> {
        UrlExtractor::from_json(body, base)
            .into_iter()
            .map(|(url, element)| DiscoveredUrl {
                url,
                method: "GET".into(),
                source_url: base.into(),
                depth: depth + 1,
                element,
            })
            .collect()
    }

    fn parse_xml(body: &str, base: &str, depth: usize) -> Vec<DiscoveredUrl> {
        UrlExtractor::from_xml(body, base)
            .into_iter()
            .map(|(url, element)| DiscoveredUrl {
                url,
                method: "GET".into(),
                source_url: base.into(),
                depth: depth + 1,
                element,
            })
            .collect()
    }

    fn parse_js(body: &str, base: &str, depth: usize) -> Vec<DiscoveredUrl> {
        UrlExtractor::from_js(body, base)
            .into_iter()
            .map(|(url, element)| DiscoveredUrl {
                url,
                method: "GET".into(),
                source_url: base.into(),
                depth: depth + 1,
                element,
            })
            .collect()
    }

    fn parse_css(body: &str, base: &str, depth: usize) -> Vec<DiscoveredUrl> {
        body.lines()
            .filter(|l| l.contains("url("))
            .filter_map(|l| {
                let start = l.find("url(")? + 4;
                let end = l[start..].find(')')?;
                let u = l[start..start + end]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                if u.starts_with("data:") {
                    None
                } else {
                    Some(DiscoveredUrl {
                        url: u.to_string(),
                        method: "GET".into(),
                        source_url: base.into(),
                        depth: depth + 1,
                        element: "css".into(),
                    })
                }
            })
            .collect()
    }
}
