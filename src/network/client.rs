#![allow(dead_code)]

use crate::network::config::HttpClientConfig;
use crate::network::cookie_jar::CookieJar;
use crate::network::errors::NetworkError;
use crate::network::models::{
    parse_cookies_from_header, HttpBody, HttpCookie, HttpHeader, HttpRequest, HttpResponse,
    RequestTiming,
};
use crate::network::protocol::HttpVersion;
use reqwest::redirect::Policy;
use std::time::Instant;

pub struct HttpClient {
    inner: reqwest::Client,
    config: HttpClientConfig,
    cookie_jar: CookieJar,
    negotiated_version: Option<HttpVersion>,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, NetworkError> {
        let jar = CookieJar::new();
        let mut builder = reqwest::Client::builder()
            .user_agent(&config.user_agent)
            .timeout(config.request_timeout)
            .connect_timeout(config.connection_timeout)
            .cookie_provider(jar.inner());
        if config.http2_prior_knowledge {
            builder = builder.http2_prior_knowledge();
        }
        if config.follow_redirects {
            builder = builder.redirect(Policy::limited(config.max_redirects));
        } else {
            builder = builder.redirect(Policy::none());
        }
        if let Some(ref proxy) = config.proxy_url {
            let p = reqwest::Proxy::all(proxy).map_err(|e| NetworkError::Build(e.to_string()))?;
            builder = builder.proxy(p);
        }
        if !config.tls_verify {
            builder = builder.danger_accept_invalid_certs(true);
        }
        let inner = builder.build().map_err(NetworkError::from)?;
        Ok(Self {
            inner,
            config,
            cookie_jar: jar,
            negotiated_version: None,
        })
    }
    pub fn config(&self) -> &HttpClientConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut HttpClientConfig {
        &mut self.config
    }

    pub fn negotiated_version(&self) -> Option<HttpVersion> {
        self.negotiated_version
    }

    pub async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError> {
        let started = Instant::now();
        let method = reqwest::Method::from_bytes(request.method.as_bytes())
            .map_err(|_| NetworkError::Build(format!("invalid method: {}", request.method)))?;
        let url = apply_query_params(&request.url, &request.query_params);
        let mut rb = self.inner.request(method, &url);
        for hdr in &request.headers {
            rb = rb.header(&hdr.name, &hdr.value);
        }
        match &request.body {
            HttpBody::Empty => {}
            HttpBody::Text(s) => {
                rb = rb.body(s.clone());
            }
            HttpBody::Bytes(b) => {
                rb = rb.body(b.clone());
            }
            HttpBody::Json(v) => {
                rb = rb.json(v);
            }
        }
        let resp = rb.send().await.map_err(NetworkError::from)?;
        let url = resp.url().to_string();
        let status = resp.status().as_u16();
        let status_text = resp.status().canonical_reason().unwrap_or("").to_string();
        let raw_version = format!("{:?}", resp.version());
        let protocol = HttpVersion::from_reqwest_version(&raw_version)
            .label()
            .to_string();
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let content_length = resp.content_length();
        let raw_headers: Vec<HttpHeader> = resp
            .headers()
            .iter()
            .map(|(n, v)| HttpHeader {
                name: n.as_str().into(),
                value: v.to_str().unwrap_or("").into(),
            })
            .collect();
        let cookies: Vec<HttpCookie> = resp
            .headers()
            .get_all("set-cookie")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .flat_map(parse_cookies_from_header)
            .collect();
        let body_bytes = resp
            .bytes()
            .await
            .map_err(|e| NetworkError::BodyRead(e.to_string()))?;
        let body = String::from_utf8(body_bytes.to_vec())
            .map(HttpBody::Text)
            .unwrap_or_else(|_| HttpBody::Bytes(body_bytes.to_vec()));
        let timing = RequestTiming {
            started_at: started,
            total_duration: started.elapsed(),
            ..Default::default()
        };
        Ok(HttpResponse {
            status_code: status,
            status_text,
            headers: raw_headers,
            cookies,
            body,
            protocol,
            content_type,
            content_length,
            timing,
            url,
        })
    }
    pub async fn get(&self, url: &str) -> Result<HttpResponse, NetworkError> {
        let req = crate::network::request_builder::RequestBuilder::get(url).build();
        self.execute(req).await
    }
    pub async fn post(&self, url: &str, body: &str) -> Result<HttpResponse, NetworkError> {
        let req = crate::network::request_builder::RequestBuilder::post(url)
            .text_body(body)
            .build();
        self.execute(req).await
    }
    pub fn cookie_jar(&self) -> &CookieJar {
        &self.cookie_jar
    }
}

/// Appends structured query parameters to the request URL using proper
/// percent-encoding. Without this, payloads inserted into query parameters by
/// the scanner/intruder would never reach the target.
fn apply_query_params(url: &str, params: &[(String, String)]) -> String {
    if params.is_empty() {
        return url.to_string();
    }
    match url::Url::parse(url) {
        Ok(mut parsed) => {
            {
                let mut pairs = parsed.query_pairs_mut();
                for (key, value) in params {
                    pairs.append_pair(key, value);
                }
            }
            parsed.to_string()
        }
        Err(_) => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::apply_query_params;

    #[test]
    fn appends_and_encodes_query_params() {
        let out = apply_query_params(
            "http://example.com/search",
            &[("q".to_string(), "' OR 1=1--".to_string())],
        );
        assert!(out.starts_with("http://example.com/search?q="));
        assert!(out.contains("%27"), "single quote must be percent-encoded: {out}");
        assert!(!out.contains("' OR 1=1--"), "raw payload must not appear unencoded");
    }

    #[test]
    fn preserves_existing_query_and_noop_when_empty() {
        assert_eq!(apply_query_params("http://example.com/", &[]), "http://example.com/");
        let out = apply_query_params(
            "http://example.com/a?x=1",
            &[("y".to_string(), "2".to_string())],
        );
        assert!(out.contains("x=1") && out.contains("y=2"), "both params expected: {out}");
    }
}
