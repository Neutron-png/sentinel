#![allow(dead_code)]

use url::Url;

pub fn normalize_url(raw: &str, base: &str) -> Option<String> {
    if raw.starts_with("javascript:") || raw.starts_with("mailto:") || raw.starts_with("tel:") {
        return None;
    }
    let base_url = Url::parse(base).ok()?;
    let resolved = base_url.join(raw).ok()?;
    let mut s = resolved.to_string();
    if let Some(pos) = s.find('#') {
        s.truncate(pos);
    }
    Some(s.trim_end_matches('/').to_string())
}

pub fn same_origin(url1: &str, url2: &str) -> bool {
    let u1 = Url::parse(url1);
    let u2 = Url::parse(url2);
    match (u1, u2) {
        (Ok(a), Ok(b)) => {
            a.scheme() == b.scheme() && a.host_str() == b.host_str() && a.port() == b.port()
        }
        _ => false,
    }
}

pub fn is_same_site(url1: &str, url2: &str) -> bool {
    let u1 = Url::parse(url1);
    let u2 = Url::parse(url2);
    match (u1, u2) {
        (Ok(a), Ok(b)) => a.host_str() == b.host_str(),
        _ => false,
    }
}
