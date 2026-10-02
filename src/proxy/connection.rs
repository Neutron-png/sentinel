#![allow(dead_code)]

use std::sync::Arc;
use std::time::Instant;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::proxy::events::{EventBus, ProxyEvent};
use crate::proxy::socks::{connect_upstream, Upstream};
use crate::proxy::tls::TlsManager;

const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_CAPTURE_BODY: usize = 64 * 1024;
const CHUNK_TERMINAL: &[u8] = b"0\r\n\r\n";

pub enum ClientStream {
    Plain(TcpStream),
    Tls(tokio_rustls::server::TlsStream<TcpStream>),
}

impl AsyncRead for ClientStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut *self {
            ClientStream::Plain(s) => std::pin::Pin::new(s).poll_read(cx, buf),
            ClientStream::Tls(s) => std::pin::Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for ClientStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match &mut *self {
            ClientStream::Plain(s) => std::pin::Pin::new(s).poll_write(cx, buf),
            ClientStream::Tls(s) => std::pin::Pin::new(s).poll_write(cx, buf),
        }
    }
    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut *self {
            ClientStream::Plain(s) => std::pin::Pin::new(s).poll_flush(cx),
            ClientStream::Tls(s) => std::pin::Pin::new(s).poll_flush(cx),
        }
    }
    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut *self {
            ClientStream::Plain(s) => std::pin::Pin::new(s).poll_shutdown(cx),
            ClientStream::Tls(s) => std::pin::Pin::new(s).poll_shutdown(cx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Capture {
    pub method: String,
    pub url: String,
    pub host: String,
    pub status: u16,
    pub protocol: String,
    pub tls_enabled: bool,
    pub request_size: u64,
    pub response_size: u64,
    pub duration_ms: u64,
    pub request_head: String,
    pub response_head: String,
}

pub async fn handle_connection(
    client: TcpStream,
    client_addr: String,
    event_bus: EventBus,
    tls: Arc<TlsManager>,
    upstream: Arc<Upstream>,
    tls_intercept: bool,
) {
    event_bus.emit(ProxyEvent::ClientConnected {
        client_addr: client_addr.clone(),
    });

    let mut client = client;
    let mut buf = [0u8; 8192];
    let n = match client.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        _ => {
            event_bus.emit(ProxyEvent::ConnectionClosed { reason: "no data".into() });
            event_bus.emit(ProxyEvent::ClientDisconnected { client_addr });
            return;
        }
    };

    let first_line = String::from_utf8_lossy(&buf[..n])
        .lines()
        .next()
        .unwrap_or("")
        .to_string();

    if first_line.starts_with("CONNECT") {
        handle_connect_tunnel(
            client,
            &first_line,
            event_bus.clone(),
            tls,
            upstream,
            tls_intercept,
        )
        .await;
    } else {
        let mut pending: Vec<u8> = buf[..n].to_vec();
        relay_loop(&mut ClientStream::Plain(client), &mut pending, event_bus.clone(), &upstream, "http").await;
    }

    event_bus.emit(ProxyEvent::ClientDisconnected { client_addr });
}

// ────────────────────────── CONNECT tunnel ──────────────────────────

async fn handle_connect_tunnel(
    mut client: TcpStream,
    connect_line: &str,
    event_bus: EventBus,
    tls: Arc<TlsManager>,
    upstream: Arc<Upstream>,
    tls_intercept: bool,
) {
    let parts: Vec<&str> = connect_line.split_whitespace().collect();
    if parts.len() < 2 {
        let _ = client.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
        return;
    }

    let target = parts[1].to_string();
    let (host, port) = split_host_port(&target);

    if !tls_intercept {
        match connect_upstream(&upstream, &host, port).await {
            Ok(up) => {
                let _ = client
                    .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
                    .await;
                let _ = pipe_bidirectional(client, up).await;
            }
            Err(e) => {
                let _ = client.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                event_bus.emit(ProxyEvent::Error { message: e.to_string() });
            }
        }
        return;
    }

    let _ = client
        .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await;

    let tls_stream = match tls.accept_tls(client, &host).await {
        Ok(s) => s,
        Err(e) => {
            event_bus.emit(ProxyEvent::Error {
                message: format!("TLS accept failed: {e}"),
            });
            return;
        }
    };

    event_bus.emit(ProxyEvent::TlsEstablished { host: host.clone() });

    let mut pending: Vec<u8> = Vec::new();
    relay_loop(
        &mut ClientStream::Tls(tls_stream),
        &mut pending,
        event_bus.clone(),
        &upstream,
        "https",
    )
    .await;
}

// ─────────────────── request/response relay loop ───────────────────

async fn relay_loop(
    client: &mut ClientStream,
    pending: &mut Vec<u8>,
    event_bus: EventBus,
    upstream: &Arc<Upstream>,
    scheme: &str,
) {
    loop {
        let head = match read_request_head(client, pending).await {
            Some(h) => h,
            None => return,
        };
        let (meta, _header_len) = match parse_head(&head) {
            Some(x) => x,
            None => return,
        };

        let default_port: u16 = if scheme == "https" { 443 } else { 80 };
        // support both origin-form (/path) and absolute-form (http://host/path)
        let (host, port, path) = if let Ok(u) = url::Url::parse(&meta.path) {
            let h = u.host_str().unwrap_or("").to_string();
            let p = u.port().unwrap_or(default_port);
            let p = if p == 0 { default_port } else { p };
            (h, p, u[url::Position::BeforePath..].to_string())
        } else {
            match &meta.host {
                Some(h) => {
                    let (h, p) = split_host_port(h);
                    let port = if p == 0 { default_port } else { p };
                    (h, port, meta.path.clone())
                }
                None => {
                    let _ = client
                        .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                        .await;
                    return;
                }
            }
        };
        if host.is_empty() {
            return;
        }
        let url = format!("{}://{}{}", scheme, host, path);
        let started = Instant::now();

        let mut up = match connect_upstream(upstream, &host, port).await {
            Ok(s) => s,
            Err(e) => {
                event_bus.emit(ProxyEvent::Error { message: e.to_string() });
                let _ = client
                    .write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                    .await;
                return;
            }
        };

        // forward request head
        if up.write_all(&head).await.is_err() {
            return;
        }

        // forward request body from pending + stream, preserving leftover bytes
        if !forward_request_body(client, &mut up, &meta, pending).await {
            return;
        }

        event_bus.emit(ProxyEvent::RequestReceived {
            method: meta.method.clone(),
            url: url.clone(),
            host: host.clone(),
        });

        let request_size = (head.len() as u64) + meta.content_length.unwrap_or(0);

        let mut resp = Vec::new();
        let mut buf = [0u8; 16384];
        let header_end;
        loop {
            match up.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => {
                    resp.extend_from_slice(&buf[..n]);
                    if let Some(p) = find_header_end(&resp) {
                        header_end = p;
                        break;
                    }
                    if resp.len() > MAX_HEADER_BYTES {
                        return;
                    }
                }
                Err(_) => return,
            }
        }

        let resp_text = String::from_utf8_lossy(&resp);
        let resp_head_raw = resp_text[..header_end].to_string();
        let status_line = resp_text.lines().next().unwrap_or("");
        let status = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(0);

        let mut resp_cl: Option<u64> = None;
        let mut chunked = false;
        for line in resp_text.lines().skip(1) {
            let lower = line.to_ascii_lowercase();
            if let Some(v) = lower.strip_prefix("content-length:") {
                resp_cl = v.trim().parse().ok();
            } else if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
                chunked = true;
            }
        }

        // response head may contain body bytes too; separate them
        let mut rest = resp.split_off(header_end); // body bytes read so far
        if client.write_all(&resp).await.is_err() {
            return;
        }
        let mut response_size = resp.len() as u64;
        let mut body_capture: Vec<u8> = Vec::new();
        if !rest.is_empty() {
            if client.write_all(&rest).await.is_err() {
                return;
            }
            response_size += rest.len() as u64;
            let take = rest.len().min(MAX_CAPTURE_BODY);
            body_capture.extend_from_slice(&rest[..take]);
        }

        if chunked {
            // relay until terminal 0-chunk seen; leftover after terminal is the
            // next pipelined response/keep-alive data → append to pending
            let mut scan_from = 0usize;
            loop {
                if let Some(pos) = find_chunk_terminal(&rest[scan_from.min(rest.len())..]) {
                    let term_end = scan_from + pos + CHUNK_TERMINAL.len();
                    let after = rest.split_off(term_end);
                    pending.extend_from_slice(&after);
                    break;
                }
                scan_from = rest.len().saturating_sub(CHUNK_TERMINAL.len() - 1);
                match up.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if client.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                        response_size += n as u64;
                        if body_capture.len() < MAX_CAPTURE_BODY {
                            let take = n.min(MAX_CAPTURE_BODY - body_capture.len());
                            body_capture.extend_from_slice(&buf[..take]);
                        }
                        rest.extend_from_slice(&buf[..n]);
                    }
                }
            }
        } else if let Some(cl) = resp_cl {
            let mut remaining = cl.saturating_sub(rest.len() as u64);
            while remaining > 0 {
                match up.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let take = n.min(remaining as usize);
                        if client.write_all(&buf[..take]).await.is_err() {
                            return;
                        }
                        response_size += take as u64;
                        if body_capture.len() < MAX_CAPTURE_BODY {
                            let t2 = take.min(MAX_CAPTURE_BODY - body_capture.len());
                            body_capture.extend_from_slice(&buf[..t2]);
                        }
                        remaining -= take as u64;
                    }
                }
            }
        } else {
            // read until upstream closes
            loop {
                match up.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if client.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                        response_size += n as u64;
                        if body_capture.len() < MAX_CAPTURE_BODY {
                            let take = n.min(MAX_CAPTURE_BODY - body_capture.len());
                            body_capture.extend_from_slice(&buf[..take]);
                        }
                    }
                }
            }
            if scheme == "http" {
                // HTTP/1.0-style: connection is done after the response
                return;
            }
        }

        let via = match upstream.as_ref() {
            Upstream::Direct => "direct".to_string(),
            Upstream::Socks5 { addr, .. } => format!("socks5://{addr}"),
        };

        event_bus.emit(ProxyEvent::TransactionCaptured {
            method: meta.method.clone(),
            url,
            host: host.clone(),
            port,
            status,
            protocol: "HTTP/1.1".to_string(),
            tls_enabled: scheme == "https",
            request_size,
            response_size,
            duration_ms: started.elapsed().as_millis() as u64,
            request_body: meta.head_raw.clone(),
            response_body: format!(
                "{}\n{}",
                resp_head_raw,
                String::from_utf8_lossy(&body_capture)
            )
            .chars()
            .take(MAX_CAPTURE_BODY * 2)
            .collect(),
            via_upstream: via,
        });
    }
}

/// Forward the request body (content-length or chunked) reading from the
/// client stream, while preserving any excess bytes (pipelined next request)
/// back into `pending`.
async fn forward_request_body(
    client: &mut ClientStream,
    up: &mut TcpStream,
    meta: &ReqMeta,
    pending: &mut Vec<u8>,
) -> bool {
    let mut buf = [0u8; 16384];

    if meta.is_chunked {
        // stream chunks until terminal sequence, keep the rest in pending
        loop {
            if let Some(pos) = find_chunk_terminal(pending) {
                let after = pending.split_off(pos + CHUNK_TERMINAL.len());
                if up.write_all(pending).await.is_err() {
                    return false;
                }
                *pending = after;
                return true;
            }
            // write everything except the last few bytes: a terminal sequence
            // may span read boundaries, so keep the tail buffered
            let keep_from = pending.len().saturating_sub(CHUNK_TERMINAL.len() - 1);
            let keep: Vec<u8> = pending.split_off(keep_from);
            if up.write_all(pending).await.is_err() {
                return false;
            }
            *pending = keep;
            match client.read(&mut buf).await {
                Ok(0) | Err(_) => return false,
                Ok(n) => pending.extend_from_slice(&buf[..n]),
            }
        }
    }

    if let Some(cl) = meta.content_length {
        if cl == 0 {
            return true;
        }
        let mut remaining = cl;
        // first flush whatever we already buffered
        let take = pending.len().min(remaining as usize);
        if up.write_all(&pending[..take]).await.is_err() {
            return false;
        }
        remaining -= take as u64;
        let leftover: Vec<u8> = pending.split_off(take); // pipelined next request
        *pending = leftover;

        while remaining > 0 {
            let want = remaining.min(buf.len() as u64) as usize;
            match client.read(&mut buf[..want]).await {
                Ok(0) | Err(_) => return false,
                Ok(n) => {
                    if up.write_all(&buf[..n]).await.is_err() {
                        return false;
                    }
                    remaining -= n as u64;
                }
            }
        }
        return true;
    }

    true
}

// ────────────────────────── parsing helpers ──────────────────────────

#[derive(Clone, Debug, Default)]
struct ReqMeta {
    method: String,
    path: String,
    host: Option<String>,
    content_length: Option<u64>,
    head_raw: String,
    is_chunked: bool,
}

fn parse_head(data: &[u8]) -> Option<(ReqMeta, usize)> {
    let text = String::from_utf8_lossy(data);
    let end = text.find("\r\n\r\n")? + 4;
    let head = &text[..end];
    let mut lines = head.lines();
    let start = lines.next()?;

    let mut parts = start.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut meta = ReqMeta {
        method,
        path,
        head_raw: head.to_string(),
        ..Default::default()
    };

    for line in lines {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("host:") {
            meta.host = Some(v.trim().to_string());
        } else if let Some(v) = lower.strip_prefix("content-length:") {
            meta.content_length = v.trim().parse().ok();
        } else if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            meta.is_chunked = true;
        }
    }

    Some((meta, end))
}

async fn read_request_head(client: &mut ClientStream, pending: &mut Vec<u8>) -> Option<Vec<u8>> {
    loop {
        if let Some(pos) = find_header_end(pending) {
            return Some(pending.drain(..pos).collect());
        }
        if pending.len() > MAX_HEADER_BYTES {
            return None;
        }
        let mut buf = [0u8; 8192];
        let n = client.read(&mut buf).await.ok()?;
        if n == 0 {
            return None;
        }
        pending.extend_from_slice(&buf[..n]);
    }
}

fn find_header_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n").map(|p| p + 4)
}

fn find_chunk_terminal(data: &[u8]) -> Option<usize> {
    data.windows(CHUNK_TERMINAL.len())
        .position(|w| w == CHUNK_TERMINAL)
}

fn ends_with_terminal(data: &[u8]) -> bool {
    data.len() >= CHUNK_TERMINAL.len()
        && &data[data.len() - CHUNK_TERMINAL.len()..] == CHUNK_TERMINAL
}

// ────────────────────────── blind pipe ──────────────────────────

async fn pipe_bidirectional(a: TcpStream, b: TcpStream) -> std::io::Result<()> {
    let (a_read, a_write) = a.into_split();
    let (b_read, b_write) = b.into_split();
    let t1 = tokio::spawn(async move {
        let mut r = a_read;
        let mut w = b_write;
        let _ = tokio::io::copy(&mut r, &mut w).await;
        let _ = w.shutdown().await;
    });
    let t2 = tokio::spawn(async move {
        let mut r = b_read;
        let mut w = a_write;
        let _ = tokio::io::copy(&mut r, &mut w).await;
        let _ = w.shutdown().await;
    });
    let _ = t1.await;
    let _ = t2.await;
    Ok(())
}

fn split_host_port(target: &str) -> (String, u16) {
    match target.rsplit_once(':') {
        Some((h, p)) => (h.trim_matches(|c| c == '[' || c == ']').to_string(), p.parse().unwrap_or(0)),
        None => (target.trim_matches(|c| c == '[' || c == ']').to_string(), 0),
    }
}
