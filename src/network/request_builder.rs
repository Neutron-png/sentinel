#![allow(dead_code)]

use crate::network::errors::NetworkError;
use crate::network::models::{HttpBody, HttpCookie, HttpHeader, HttpRequest};
use serde::Serialize;
use std::collections::HashMap;

pub struct RequestBuilder {
    method: String,
    url: String,
    headers: Vec<HttpHeader>,
    cookies: Vec<HttpCookie>,
    query_params: Vec<(String, String)>,
    path_params: HashMap<String, String>,
    body: HttpBody,
}

impl RequestBuilder {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_uppercase(),
            url: url.to_string(),
            headers: vec![],
            cookies: vec![],
            query_params: vec![],
            path_params: HashMap::new(),
            body: HttpBody::Empty,
        }
    }
    pub fn get(url: &str) -> Self {
        Self::new("GET", url)
    }
    pub fn post(url: &str) -> Self {
        Self::new("POST", url)
    }
    pub fn put(url: &str) -> Self {
        Self::new("PUT", url)
    }
    pub fn patch(url: &str) -> Self {
        Self::new("PATCH", url)
    }
    pub fn delete(url: &str) -> Self {
        Self::new("DELETE", url)
    }
    pub fn options(url: &str) -> Self {
        Self::new("OPTIONS", url)
    }
    pub fn head(url: &str) -> Self {
        Self::new("HEAD", url)
    }
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push(HttpHeader {
            name: name.into(),
            value: value.into(),
        });
        self
    }
    pub fn headers(mut self, hdrs: Vec<(&str, &str)>) -> Self {
        for (n, v) in hdrs {
            self.headers.push(HttpHeader {
                name: n.into(),
                value: v.into(),
            });
        }
        self
    }
    pub fn cookie(mut self, name: &str, value: &str) -> Self {
        self.cookies.push(HttpCookie {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            expires: None,
        });
        self
    }
    pub fn query_param(mut self, key: &str, value: &str) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }
    pub fn query_params(mut self, params: &[(&str, &str)]) -> Self {
        for (k, v) in params {
            self.query_params.push((k.to_string(), v.to_string()));
        }
        self
    }
    pub fn path_param(mut self, key: &str, value: &str) -> Self {
        self.path_params.insert(key.into(), value.into());
        self
    }
    pub fn json_body<T: Serialize>(mut self, payload: &T) -> Result<Self, NetworkError> {
        let v = serde_json::to_value(payload).map_err(|e| NetworkError::Build(e.to_string()))?;
        self.body = HttpBody::Json(v);
        if !self
            .headers
            .iter()
            .any(|h| h.name.eq_ignore_ascii_case("content-type"))
        {
            self.headers.push(HttpHeader {
                name: "Content-Type".into(),
                value: "application/json".into(),
            });
        }
        Ok(self)
    }
    pub fn text_body(mut self, text: &str) -> Self {
        self.body = HttpBody::Text(text.into());
        self
    }
    pub fn bytes_body(mut self, bytes: Vec<u8>) -> Self {
        self.body = HttpBody::Bytes(bytes);
        self
    }
    pub fn form_body(mut self, fields: &[(&str, &str)]) -> Self {
        let encoded: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding(k), urlencoding(v)))
            .collect();
        self.body = HttpBody::Text(encoded.join("&"));
        if !self
            .headers
            .iter()
            .any(|h| h.name.eq_ignore_ascii_case("content-type"))
        {
            self.headers.push(HttpHeader {
                name: "Content-Type".into(),
                value: "application/x-www-form-urlencoded".into(),
            });
        }
        self
    }
    pub fn build(self) -> HttpRequest {
        let mut url = self.url;
        for (k, v) in &self.path_params {
            url = url.replace(&format!("{{{}}}", k), v);
        }
        if !self.query_params.is_empty() {
            let qs: Vec<String> = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding(k), urlencoding(v)))
                .collect();
            url = format!("{}?{}", url, qs.join("&"));
        }
        HttpRequest {
            method: self.method,
            url,
            headers: self.headers,
            cookies: self.cookies,
            body: self.body,
            query_params: self.query_params,
        }
    }
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
}
