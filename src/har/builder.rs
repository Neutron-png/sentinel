#![allow(dead_code)]

use chrono::Utc;

use crate::har::models::*;
use crate::network::models::{HttpRequest, HttpResponse};

pub struct HarBuilder;

impl HarBuilder {
    pub fn build_entry(request: &HttpRequest, response: &HttpResponse, timing_ms: f64) -> HarEntry {
        let now = Utc::now().to_rfc3339();
        let resp_headers: Vec<HarHeader> = response
            .headers
            .iter()
            .map(|h| HarHeader {
                name: h.name.clone(),
                value: h.value.clone(),
            })
            .collect();
        let req_headers: Vec<HarHeader> = request
            .headers
            .iter()
            .map(|h| HarHeader {
                name: h.name.clone(),
                value: h.value.clone(),
            })
            .collect();
        let req_headers_size = req_headers.len() as i64 * 40;
        let resp_headers_size = resp_headers.len() as i64 * 40;
        let query: Vec<HarQueryParam> = request
            .query_params
            .iter()
            .map(|(k, v)| HarQueryParam {
                name: k.clone(),
                value: v.clone(),
            })
            .collect();
        let body_size = response.body.len() as i64;

        HarEntry {
            started_date_time: now,
            time: timing_ms,
            request: HarRequest {
                method: request.method.clone(),
                url: request.url.clone(),
                http_version: "HTTP/1.1".into(),
                cookies: vec![],
                headers: req_headers,
                query_string: query,
                post_data: None,
                headers_size: req_headers_size,
                body_size: request.body.len() as i64,
            },
            response: HarResponse {
                status: response.status_code,
                status_text: response.status_text.clone(),
                http_version: "HTTP/1.1".into(),
                cookies: vec![],
                headers: resp_headers,
                content: HarContent {
                    size: body_size,
                    compression: None,
                    mime_type: response.content_type.clone().unwrap_or_default(),
                    text: response.body.as_text().map(|s| s.to_string()),
                },
                redirect_url: String::new(),
                headers_size: resp_headers_size,
                body_size,
            },
            timings: HarTimings {
                blocked: 0.0,
                dns: 0.0,
                connect: 0.0,
                send: timing_ms * 0.2,
                wait: timing_ms * 0.6,
                receive: timing_ms * 0.2,
                ssl: 0.0,
            },
            server_ip_address: None,
            connection: None,
            comment: None,
        }
    }

    pub fn build_page(_url: &str, title: &str, load_time_ms: f64) -> HarPage {
        HarPage {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            started_date_time: Utc::now().to_rfc3339(),
            page_timings: HarPageTimings {
                on_content_load: load_time_ms * 0.7,
                on_load: load_time_ms,
            },
        }
    }
}
