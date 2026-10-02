#![allow(dead_code)]

use crate::network::models::{HttpBody, HttpCookie, HttpHeader, HttpRequest};

/// Parse a raw HTTP request (as typed in Repeater) into an HttpRequest.
/// Supports origin-form (`GET /path HTTP/1.1` + Host header) and
/// absolute-form (`GET http://host/path HTTP/1.1`, proxy style).
pub fn parse_raw_request(raw: &str) -> Result<HttpRequest, String> {
    let (head, body) = match raw.find("\r\n\r\n") {
        Some(p) => (&raw[..p + 4], raw[p + 4..].to_string()),
        None => match raw.find("\n\n") {
            Some(p) => (&raw[..p + 2], raw[p + 2..].to_string()),
            None => (raw, String::new()),
        },
    };

    let mut lines = head.lines();
    let start = lines.next().ok_or("empty request")?;
    let mut parts = start.split_whitespace();
    let method = parts.next().ok_or("missing method")?.to_uppercase();
    let target = parts.next().ok_or("missing url")?.to_string();

    let mut headers: Vec<HttpHeader> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| format!("bad header line: {line}"))?;
        headers.push(HttpHeader {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        });
    }

    let host_header = headers
        .iter()
        .find(|h| h.name.eq_ignore_ascii_case("host"))
        .map(|h| h.value.clone());

    // resolve the final URL
    let (scheme, authority, path) = if target.starts_with("http://") || target.starts_with("https://")
    {
        let u = url::Url::parse(&target).map_err(|e| format!("bad url: {e}"))?;
        let scheme = u.scheme().to_string();
        let auth = match (u.host_str(), u.port()) {
            (Some(h), Some(p)) => format!("{h}:{p}"),
            (Some(h), None) => h.to_string(),
            _ => return Err("bad url authority".into()),
        };
        let path = u[url::Position::BeforePath..].to_string();
        (scheme, auth, path)
    } else {
        let auth = host_header.clone().ok_or("missing Host header")?;
        let scheme = if authority_has_port(&auth, "443") { "https" } else { "http" };
        (scheme.to_string(), auth, target)
    };

    let url = if path.starts_with('/') || path.is_empty() {
        format!("{}://{}{}", scheme, authority, path)
    } else {
        format!("{}://{}{}", scheme, authority, path)
    };

    // strip Host from headers? keep it; reqwest overrides safely. Actually
    // reqwest will use it as-is — keep user control (needed for host-header tests).

    let cookies: Vec<HttpCookie> = headers
        .iter()
        .filter(|h| h.name.eq_ignore_ascii_case("cookie"))
        .flat_map(|h| crate::network::models::parse_cookies_from_header(&h.value))
        .collect();

    let body_obj = if body.is_empty() {
        HttpBody::Empty
    } else {
        HttpBody::Text(body)
    };

    Ok(HttpRequest {
        method,
        url,
        headers,
        cookies,
        body: body_obj,
        query_params: Vec::new(),
    })
}

fn authority_has_port(auth: &str, port: &str) -> bool {
    auth.rsplit_once(':')
        .map(|(_, p)| p == port)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_form() {
        let req = parse_raw_request("GET /a/b?x=1 HTTP/1.1\r\nHost: example.com\r\nAccept: */*\r\n\r\n").unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.url, "http://example.com/a/b?x=1");
        assert_eq!(req.headers.len(), 2);
    }

    #[test]
    fn absolute_form() {
        let req = parse_raw_request("GET http://api.acme.com:8443/v1 HTTP/1.1\r\nHost: api.acme.com\r\n\r\n").unwrap();
        assert_eq!(req.url, "http://api.acme.com:8443/v1");
    }

    #[test]
    fn https_and_body() {
        let req = parse_raw_request("POST /login HTTP/1.1\r\nHost: acme.com\r\nContent-Length: 9\r\n\r\nuser=admin").unwrap();
        assert_eq!(req.url, "http://acme.com/login");
        assert!(matches!(req.body, HttpBody::Text(ref b) if b == "user=admin"));
    }

    #[test]
    fn missing_host_fails() {
        assert!(parse_raw_request("GET /x HTTP/1.1\r\n\r\n").is_err());
    }
}
