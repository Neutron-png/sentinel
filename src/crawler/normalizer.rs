#![allow(dead_code)]

use url::Url;

pub struct UrlNormalizer;

impl UrlNormalizer {
    pub fn normalize(raw: &str, base: &str) -> Option<String> {
        let url = if raw.starts_with("http") {
            Url::parse(raw).ok()?
        } else {
            let base_url = Url::parse(base).ok()?;
            base_url.join(raw).ok()?
        };
        let mut s = url.to_string();
        if let Some(pos) = s.find('#') {
            s.truncate(pos);
        }
        Some(s)
    }

    pub fn same_host(url1: &str, url2: &str) -> bool {
        let u1 = Url::parse(url1);
        let u2 = Url::parse(url2);
        match (u1, u2) {
            (Ok(a), Ok(b)) => a.host_str() == b.host_str() && a.scheme() == b.scheme(),
            _ => false,
        }
    }

    pub fn is_html_content(content_type: &str) -> bool {
        content_type.contains("text/html") || content_type.contains("application/xhtml")
    }

    pub fn is_json_content(content_type: &str) -> bool {
        content_type.contains("application/json") || content_type.contains("+json")
    }

    pub fn is_xml_content(content_type: &str) -> bool {
        content_type.contains("application/xml")
            || content_type.contains("text/xml")
            || content_type.contains("+xml")
    }

    pub fn is_js_content(content_type: &str) -> bool {
        content_type.contains("javascript") || content_type.contains("ecmascript")
    }

    pub fn is_css_content(content_type: &str) -> bool {
        content_type.contains("text/css")
    }
}
