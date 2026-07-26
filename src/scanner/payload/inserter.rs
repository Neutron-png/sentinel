#![allow(dead_code)]

use crate::network::models::{HttpBody, HttpHeader, HttpRequest};
use crate::scanner::payload::models::InsertionPoint;

pub fn insert(request: &mut HttpRequest, point: &InsertionPoint, value: &str) {
    match point {
        InsertionPoint::QueryParam(key) => {
            request.query_params.push((key.clone(), value.to_string()));
        }
        InsertionPoint::PathParam(key) => {
            request.url = request.url.replace(&format!("{{{}}}", key), value);
        }
        InsertionPoint::Header(name) => {
            if let Some(h) = request
                .headers
                .iter_mut()
                .find(|h| h.name.eq_ignore_ascii_case(name))
            {
                h.value = value.to_string();
            } else {
                request.headers.push(HttpHeader {
                    name: name.clone(),
                    value: value.to_string(),
                });
            }
        }
        InsertionPoint::Cookie(name) => {
            request.cookies.push(crate::network::models::HttpCookie {
                name: name.clone(),
                value: value.to_string(),
                domain: None,
                path: None,
                secure: false,
                http_only: false,
                expires: None,
            });
        }
        InsertionPoint::JsonValue(key) => {
            request.body = HttpBody::Text(format!("{{\"{}\":\"{}\"}}", key, value));
        }
        InsertionPoint::XmlValue(key) => {
            request.body = HttpBody::Text(format!("<{}>{}</{}>", key, value, key));
        }
        InsertionPoint::FormField(key) => {
            if let HttpBody::Text(ref body) = request.body {
                request.body = HttpBody::Text(format!("{}&{}={}", body, key, value));
            } else {
                request.body = HttpBody::Text(format!("{}={}", key, value));
            }
        }
        InsertionPoint::MultipartField(key) => {
            request.body = HttpBody::Text(format!("--boundary\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n--boundary--", key, value));
        }
        InsertionPoint::RawBody => {
            request.body = HttpBody::Text(value.to_string());
        }
    }
}

pub fn insert_all(
    request: &HttpRequest,
    points: &[InsertionPoint],
    value: &str,
) -> Vec<HttpRequest> {
    points
        .iter()
        .map(|point| {
            let mut req = request.clone();
            insert(&mut req, point, value);
            req
        })
        .collect()
}
