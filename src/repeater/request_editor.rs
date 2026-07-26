#![allow(dead_code)]

use crate::network::models::{HttpBody, HttpCookie, HttpHeader, HttpRequest};

#[derive(Debug, Clone)]
pub enum RequestEdit {
    SetMethod(String),
    SetUrl(String),
    SetHeader(String, String),
    RemoveHeader(String),
    SetCookie(String, String),
    RemoveCookie(String),
    SetBody(HttpBody),
    AppendQueryParam(String, String),
    RemoveQueryParam(String),
}

pub fn apply_edits(request: &mut HttpRequest, edits: &[RequestEdit]) {
    for edit in edits {
        match edit {
            RequestEdit::SetMethod(m) => request.method = m.clone(),
            RequestEdit::SetUrl(u) => request.url = u.clone(),
            RequestEdit::SetHeader(n, v) => {
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
            RequestEdit::RemoveHeader(n) => {
                request.headers.retain(|h| !h.name.eq_ignore_ascii_case(n));
            }
            RequestEdit::SetCookie(n, v) => {
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
            RequestEdit::RemoveCookie(n) => {
                request.cookies.retain(|c| c.name != *n);
            }
            RequestEdit::SetBody(b) => request.body = b.clone(),
            RequestEdit::AppendQueryParam(k, v) => {
                request.query_params.push((k.clone(), v.clone()))
            }
            RequestEdit::RemoveQueryParam(k) => {
                request.query_params.retain(|(key, _)| key != k);
            }
        }
    }
}
