use regex::Regex;

use super::models::{JwtLocation, JwtLocationType, JwtToken};
use super::parser;

const JWT_PATTERN: &str = r"[A-Za-z0-9\-_]{20,}\.[A-Za-z0-9\-_]+\.[A-Za-z0-9\-_]*";

pub fn detect_in_string(input: &str, location: JwtLocation) -> Vec<JwtToken> {
    let re = match Regex::new(JWT_PATTERN) {
        Ok(re) => re,
        Err(_) => return Vec::new(),
    };

    re.find_iter(input)
        .filter_map(|m| parser::parse_token(m.as_str(), location.clone()).ok())
        .collect()
}

pub fn detect_in_headers(
    headers: &[(String, String)],
    url: Option<&str>,
    host: Option<&str>,
) -> Vec<JwtToken> {
    let mut tokens = Vec::new();
    let auth_prefixes = ["Bearer ", "JWT ", "Bearer:", "JWT:"];

    for (name, value) in headers {
        let loc = JwtLocation {
            location_type: JwtLocationType::CustomHeader,
            key: Some(name.clone()),
            url: url.map(|s| s.to_string()),
            host: host.map(|s| s.to_string()),
        };

        if name.to_lowercase() == "authorization" {
            let loc_auth = JwtLocation {
                location_type: JwtLocationType::AuthorizationHeader,
                key: Some(name.clone()),
                url: url.map(|s| s.to_string()),
                host: host.map(|s| s.to_string()),
            };
            for prefix in &auth_prefixes {
                if let Some(stripped) = value.strip_prefix(prefix) {
                    let trimmed = stripped.trim();
                    if parser::is_jwt_format(trimmed) {
                        if let Ok(token) = parser::parse_token(trimmed, loc_auth.clone()) {
                            tokens.push(token);
                        }
                    }
                }
            }
            if parser::is_jwt_format(value) {
                if let Ok(token) = parser::parse_token(value, loc_auth.clone()) {
                    tokens.push(token);
                }
            }
        } else if name.to_lowercase().starts_with("x-")
            || name.to_lowercase().contains("token")
            || name.to_lowercase().contains("jwt")
        {
            let found = detect_in_string(value, loc.clone());
            tokens.extend(found);
        } else if parser::is_jwt_format(value) {
            if let Ok(token) = parser::parse_token(value, loc.clone()) {
                tokens.push(token);
            }
        }
    }

    tokens
}

pub fn detect_in_cookies(
    cookies: &[(String, String)],
    url: Option<&str>,
    host: Option<&str>,
) -> Vec<JwtToken> {
    let mut tokens = Vec::new();
    for (name, value) in cookies {
        let loc = JwtLocation {
            location_type: JwtLocationType::Cookie,
            key: Some(name.clone()),
            url: url.map(|s| s.to_string()),
            host: host.map(|s| s.to_string()),
        };
        if parser::is_jwt_format(value) {
            if let Ok(token) = parser::parse_token(value, loc) {
                tokens.push(token);
            }
        }
    }
    tokens
}

pub fn detect_in_body(
    body: &str,
    content_type: Option<&str>,
    url: Option<&str>,
    host: Option<&str>,
) -> Vec<JwtToken> {
    let mut tokens = Vec::new();

    let loc_type = match content_type {
        Some(ct) if ct.contains("application/json") => JwtLocationType::JsonBody,
        Some(ct) if ct.contains("application/x-www-form-urlencoded") => JwtLocationType::FormBody,
        Some(ct) if ct.contains("multipart/form-data") => JwtLocationType::MultipartForm,
        _ => JwtLocationType::PostBody,
    };

    let loc = JwtLocation {
        location_type: loc_type,
        key: None,
        url: url.map(|s| s.to_string()),
        host: host.map(|s| s.to_string()),
    };

    if let Some(ct) = content_type {
        if ct.contains("application/json") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
                tokens.extend(detect_in_json_value(&json, loc.clone()));
            }
        } else if ct.contains("application/x-www-form-urlencoded") {
            for pair in body.split('&') {
                if let Some((_key, value)) = pair.split_once('=') {
                    let decoded = urlencoding_decode(value);
                    tokens.extend(detect_in_string(&decoded, loc.clone()));
                    tokens.extend(detect_in_string(value, loc.clone()));
                }
            }
        } else {
            tokens.extend(detect_in_string(body, loc.clone()));
        }
    } else {
        tokens.extend(detect_in_string(body, loc.clone()));
    }

    tokens
}

fn detect_in_json_value(value: &serde_json::Value, location: JwtLocation) -> Vec<JwtToken> {
    let mut tokens = Vec::new();
    match value {
        serde_json::Value::String(s) => {
            if parser::is_jwt_format(s) {
                if let Ok(token) = parser::parse_token(s, location) {
                    tokens.push(token);
                }
            }
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let mut loc = location.clone();
                loc.key = Some(k.clone());
                tokens.extend(detect_in_json_value(v, loc));
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                tokens.extend(detect_in_json_value(v, location.clone()));
            }
        }
        _ => {}
    }
    tokens
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                result.push(' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    result.push((hi << 4 | lo) as char);
                    i += 3;
                } else {
                    result.push('%');
                    i += 1;
                }
            }
            _ => {
                result.push(bytes[i] as char);
                i += 1;
            }
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

pub fn detect_in_request(
    _method: &str,
    url_str: &str,
    headers: &[(String, String)],
    body: Option<&str>,
    content_type: Option<&str>,
) -> Vec<JwtToken> {
    let parsed = url::Url::parse(url_str);
    let host = parsed
        .as_ref()
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()));
    let url_opt = Some(url_str);

    let mut tokens = detect_in_headers(headers, url_opt, host.as_deref());
    tokens.extend(detect_in_cookies(
        &headers
            .iter()
            .filter(|(k, _)| k.to_lowercase() == "cookie")
            .flat_map(|(_, v)| {
                v.split(';')
                    .filter_map(|c| {
                        let parts: Vec<&str> = c.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        url_opt,
        host.as_deref(),
    ));

    if let Ok(parsed_url) = parsed.as_ref() {
        for (key, values) in parsed_url.query_pairs() {
            for value in values.split(',') {
                let loc = JwtLocation {
                    location_type: JwtLocationType::QueryParameter,
                    key: Some(key.to_string()),
                    url: url_opt.map(|s| s.to_string()),
                    host: host.clone(),
                };
                if parser::is_jwt_format(value) {
                    if let Ok(token) = parser::parse_token(value, loc) {
                        tokens.push(token);
                    }
                }
            }
        }
    }

    if let Some(b) = body {
        if !b.is_empty() {
            tokens.extend(detect_in_body(b, content_type, url_opt, host.as_deref()));
        }
    }

    tokens
}

pub fn detect_in_response(
    url_str: &str,
    headers: &[(String, String)],
    body: Option<&str>,
    content_type: Option<&str>,
) -> Vec<JwtToken> {
    let parsed = url::Url::parse(url_str);
    let host = parsed
        .as_ref()
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()));
    let url_opt = Some(url_str);

    let mut tokens = detect_in_headers(headers, url_opt, host.as_deref());
    tokens.extend(detect_in_cookies(
        &headers
            .iter()
            .filter(|(k, _)| k.to_lowercase() == "set-cookie")
            .filter_map(|(_, v)| {
                let parts: Vec<&str> = v.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let cookie_val = parts[1].split(';').next().unwrap_or("").trim();
                    Some((parts[0].trim().to_string(), cookie_val.to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        url_opt,
        host.as_deref(),
    ));

    if let Some(b) = body {
        if !b.is_empty() {
            tokens.extend(detect_in_body(b, content_type, url_opt, host.as_deref()));
        }
    }

    tokens
}
