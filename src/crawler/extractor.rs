#![allow(dead_code)]

pub struct UrlExtractor;

impl UrlExtractor {
    pub fn from_html(html: &str, base_url: &str) -> Vec<(String, String)> {
        let mut urls = Vec::new();
        let lower = html.to_lowercase();
        // <a href="...">
        for tag in extract_attr(&lower, "<a ", "href") {
            urls.push((tag, "a".into()));
        }
        // <form action="...">
        for tag in extract_attr(&lower, "<form ", "action") {
            urls.push((tag, "form".into()));
        }
        // <img src="...">
        for tag in extract_attr(&lower, "<img ", "src") {
            urls.push((tag, "img".into()));
        }
        // <script src="...">
        for tag in extract_attr(&lower, "<script ", "src") {
            urls.push((tag, "script".into()));
        }
        // <link href="...">
        for tag in extract_attr(&lower, "<link ", "href") {
            urls.push((tag, "link".into()));
        }
        // <iframe src="...">, <frame src="...">, <source src="...">, <video src="...">, <audio src="...">
        for el in &["iframe", "frame", "source", "video", "audio"] {
            for tag in extract_attr(&lower, &format!("<{} ", el), "src") {
                urls.push((tag, el.to_string()));
            }
        }
        // resolve relative
        urls.into_iter()
            .map(|(u, e)| (resolve_relative(base_url, &u), e))
            .collect()
    }

    pub fn from_json(json: &str, base_url: &str) -> Vec<(String, String)> {
        let mut urls = Vec::new();
        // Simple pattern matching for "url": "...", "href": "...", "link": "..."
        for pattern in &["\"url\"", "\"href\"", "\"link\"", "\"uri\"", "\"location\""] {
            let mut search = json;
            while let Some(pos) = search.find(pattern) {
                let after = &search[pos + pattern.len()..];
                if let Some(start) = after.find('"') {
                    let val = &after[start + 1..];
                    if let Some(end) = val.find('"') {
                        let url_val = &val[..end];
                        if url_val.starts_with("http") || url_val.starts_with('/') {
                            urls.push((resolve_relative(base_url, url_val), "json".into()));
                        }
                        search = &val[end + 1..];
                        continue;
                    }
                }
                search = &after[1..];
            }
        }
        urls
    }

    pub fn from_js(js: &str, base_url: &str) -> Vec<(String, String)> {
        let mut urls = Vec::new();
        for pattern in &[
            "fetch(\"",
            "fetch('",
            "XMLHttpRequest",
            "axios",
            "$.ajax",
            "$.get",
            "$.post",
        ] {
            let remaining = js;
            let mut search = remaining;
            while let Some(pos) = search.find(pattern) {
                let after = &search[pos..];
                urls.extend(extract_js_urls(after, pattern));
                if pos + 1 < after.len() {
                    search = &after[1..];
                } else {
                    break;
                }
            }
        }
        // Extract quoted URLs
        for quote in &['"', '\''] {
            let q = format!("{}https://", quote);
            let mut search = js;
            while let Some(pos) = search.find(&q) {
                let start = pos + 1;
                let end = search[start..]
                    .find(*quote)
                    .map(|e| start + e)
                    .unwrap_or(search.len());
                let url = &search[start..end];
                if url.len() > 10 {
                    urls.push((url.to_string(), "js".into()));
                }
                search = &search[end + 1..];
            }
        }
        urls.into_iter()
            .map(|(u, e)| (resolve_relative(base_url, &u), e))
            .collect()
    }

    pub fn from_xml(xml: &str, base_url: &str) -> Vec<(String, String)> {
        let mut urls = Vec::new();
        for tag in extract_attr(xml, "<loc>", "") {
            urls.push((tag, "sitemap".into()));
        }
        for tag in extract_attr(xml, "<url>", "") {
            urls.push((tag, "xml".into()));
        }
        urls.into_iter()
            .map(|(u, e)| (resolve_relative(base_url, &u), e))
            .collect()
    }
}

fn extract_attr(html: &str, tag_open: &str, attr_name: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut search = html;
    while let Some(pos) = search.find(tag_open) {
        let tag_content = &search[pos..];
        let end = tag_content.find('>').unwrap_or(tag_content.len());
        let tag = &tag_content[..end];

        if attr_name.is_empty() {
            // Extract text content inside tag
            if let Some(inner) = tag_content[end..].trim_start().split('<').next() {
                if !inner.is_empty() && (inner.starts_with("http") || inner.starts_with('/')) {
                    results.push(inner.trim().to_string());
                }
            }
        } else {
            // Extract attribute value
            let patterns = [
                format!("{}=", attr_name),
                format!("{} =", attr_name),
                format!("{}='", attr_name),
                format!("{}=\"", attr_name),
            ];
            for pat in &patterns {
                if let Some(attr_pos) = tag.find(&pat[..]) {
                    let after_attr = &tag[attr_pos + pat.len()..];
                    let quote = after_attr.chars().next().unwrap_or('"');
                    let q = if quote == '\'' || quote == '"' {
                        quote
                    } else {
                        '"'
                    };
                    let val_start = if quote == q { 1 } else { 0 };
                    let val = &after_attr[val_start..];
                    if let Some(val_end) = val.find(q) {
                        let url_val = &val[..val_end].trim();
                        if url_val.starts_with("http")
                            || url_val.starts_with('/')
                            || url_val.starts_with('.')
                        {
                            results.push(url_val.to_string());
                        }
                    }
                    break;
                }
            }
        }
        let next = pos + tag.len().min(tag_content.len()).max(1);
        search = &search[next..];
    }
    results
}

fn resolve_relative(base: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    if url.starts_with("//") {
        let scheme = base.split("://").next().unwrap_or("https");
        return format!("{}:{}", scheme, url);
    }
    if url.starts_with('/') {
        if let Some(domain_end) = base.find("://") {
            let after_scheme = &base[domain_end + 3..];
            let domain = after_scheme.split('/').next().unwrap_or(after_scheme);
            return format!("{}://{}{}", &base[..domain_end], domain, url);
        }
    }
    // relative
    let base_dir = if base.ends_with('/') {
        base.to_string()
    } else {
        let mut parts: Vec<&str> = base.split('/').collect();
        if parts.len() > 3 {
            parts.pop();
            parts.join("/") + "/"
        } else {
            format!("{}/", base)
        }
    };
    #[allow(clippy::manual_strip)]
    if url.starts_with("./") {
        return format!("{}{}", base_dir, &url[2..]);
    }
    #[allow(clippy::manual_strip)]
    if url.starts_with("../") {
        return format!("{}{}", base_dir, &url[3..]);
    }
    format!("{}{}", base_dir, url)
}

fn extract_js_urls(js: &str, _pattern: &str) -> Vec<(String, String)> {
    let mut urls = Vec::new();
    if let Some(open) = js.find('(') {
        let after = &js[open + 1..];
        let mut depth = 1;
        let mut end = 0;
        for (i, c) in after.char_indices() {
            if c == '(' {
                depth += 1;
            }
            if c == ')' {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
        }
        let args = &after[..end];
        // Extract first string argument
        for quote in &['"', '\''] {
            if let Some(qs) = args.find(*quote) {
                if qs + 1 < args.len() {
                    let rest = &args[qs + 1..];
                    if let Some(qe) = rest.find(*quote) {
                        let url_val = &rest[..qe];
                        if url_val.starts_with("http") || url_val.starts_with('/') {
                            urls.push((url_val.to_string(), "js-fetch".into()));
                        }
                    }
                }
            }
        }
    }
    urls
}
