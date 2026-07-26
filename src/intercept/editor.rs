#![allow(dead_code)]

use crate::network::models::{HttpBody, HttpCookie, HttpHeader, HttpRequest, HttpResponse};

#[derive(Debug, Clone)]
pub enum EditOperation {
    SetMethod(String),
    SetUrl(String),
    SetHeader(String, String),
    RemoveHeader(String),
    SetCookie(String, String),
    RemoveCookie(String),
    SetBody(HttpBody),
    SetStatusCode(u16),
    AppendQueryParam(String, String),
    RemoveQueryParam(String),
}

pub fn edit_request(request: &mut HttpRequest, operations: &[EditOperation]) {
    for op in operations {
        match op {
            EditOperation::SetMethod(m) => request.method = m.clone(),
            EditOperation::SetUrl(u) => request.url = u.clone(),
            EditOperation::SetHeader(n, v) => {
                if let Some(h) = request
                    .headers
                    .iter_mut()
                    .find(|h| h.name.eq_ignore_ascii_case(n))
                {
                    h.value = v.clone();
                } else {
                    request.headers.push(HttpHeader {
                        name: n.clone(),
                        value: v.clone(),
                    });
                }
            }
            EditOperation::RemoveHeader(n) => {
                request.headers.retain(|h| !h.name.eq_ignore_ascii_case(n));
            }
            EditOperation::SetCookie(n, v) => {
                if let Some(c) = request.cookies.iter_mut().find(|c| c.name == *n) {
                    c.value = v.clone();
                } else {
                    request.cookies.push(HttpCookie {
                        name: n.clone(),
                        value: v.clone(),
                        domain: None,
                        path: None,
                        secure: false,
                        http_only: false,
                        expires: None,
                    });
                }
            }
            EditOperation::RemoveCookie(n) => {
                request.cookies.retain(|c| c.name != *n);
            }
            EditOperation::SetBody(b) => request.body = b.clone(),
            EditOperation::AppendQueryParam(k, v) => {
                request.query_params.push((k.clone(), v.clone()))
            }
            EditOperation::RemoveQueryParam(k) => {
                request.query_params.retain(|(key, _)| key != k);
            }
            EditOperation::SetStatusCode(_) => {}
        }
    }
}

pub fn edit_response(response: &mut HttpResponse, operations: &[EditOperation]) {
    for op in operations {
        match op {
            EditOperation::SetStatusCode(s) => {
                response.status_code = *s;
                response.status_text = String::new();
            }
            EditOperation::SetHeader(n, v) => {
                if let Some(h) = response
                    .headers
                    .iter_mut()
                    .find(|h| h.name.eq_ignore_ascii_case(n))
                {
                    h.value = v.clone();
                } else {
                    response.headers.push(HttpHeader {
                        name: n.clone(),
                        value: v.clone(),
                    });
                }
            }
            EditOperation::RemoveHeader(n) => {
                response.headers.retain(|h| !h.name.eq_ignore_ascii_case(n));
            }
            EditOperation::SetCookie(n, v) => {
                if let Some(c) = response.cookies.iter_mut().find(|c| c.name == *n) {
                    c.value = v.clone();
                } else {
                    response.cookies.push(HttpCookie {
                        name: n.clone(),
                        value: v.clone(),
                        domain: None,
                        path: None,
                        secure: false,
                        http_only: false,
                        expires: None,
                    });
                }
            }
            EditOperation::RemoveCookie(n) => {
                response.cookies.retain(|c| c.name != *n);
            }
            EditOperation::SetBody(b) => response.body = b.clone(),
            _ => {}
        }
    }
}
