#![allow(dead_code)]

use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::proxy::events::{EventBus, ProxyEvent};
use crate::proxy::tls::TlsManager;

pub async fn handle_connection(
    mut client: TcpStream,
    client_addr: String,
    event_bus: EventBus,
    tls: Arc<TlsManager>,
) {
    event_bus.emit(ProxyEvent::ClientConnected {
        client_addr: client_addr.clone(),
    });

    let mut buf = [0u8; 8192];
    let n = match client.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        _ => {
            event_bus.emit(ProxyEvent::ConnectionClosed {
                reason: "read failed".into(),
            });
            return;
        }
    };

    let request_str = String::from_utf8_lossy(&buf[..n]);
    let first_line = request_str.lines().next().unwrap_or("");

    if first_line.starts_with("CONNECT") {
        handle_connect_tunnel(client, &client_addr, first_line, event_bus.clone(), tls).await;
    } else {
        handle_http_forward(client, &buf[..n], event_bus.clone()).await;
    }

    event_bus.emit(ProxyEvent::ClientDisconnected { client_addr });
}

async fn handle_http_forward(mut client: TcpStream, initial_data: &[u8], event_bus: EventBus) {
    let request_str = String::from_utf8_lossy(initial_data);
    if let Some(line) = request_str.lines().next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            event_bus.emit(ProxyEvent::RequestReceived {
                method: parts[0].to_string(),
                url: parts[1].to_string(),
                host: String::new(),
            });
        }
    }

    let response =
        b"HTTP/1.1 200 OK\r\nContent-Length: 38\r\n\r\nHTTP Forward Proxy: request received\n";
    let _ = client.write_all(response).await;
    let _ = client.shutdown().await;
}

async fn handle_connect_tunnel(
    mut client: TcpStream,
    _client_addr: &str,
    connect_line: &str,
    event_bus: EventBus,
    tls: Arc<TlsManager>,
) {
    let parts: Vec<&str> = connect_line.split_whitespace().collect();
    if parts.len() < 2 {
        let _ = client.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
        return;
    }

    let target = parts[1];
    let hostname = target.split(':').next().unwrap_or(target);

    event_bus.emit(ProxyEvent::RequestReceived {
        method: "CONNECT".into(),
        url: target.to_string(),
        host: hostname.to_string(),
    });

    // Respond 200 to establish CONNECT tunnel
    let _ = client
        .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await;

    // TLS intercept
    match tls.accept_tls(client, hostname).await {
        Ok(mut tls_stream) => {
            event_bus.emit(ProxyEvent::TlsEstablished {
                host: hostname.to_string(),
            });

            let mut buf = [0u8; 4096];
            match tls_stream.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    let req = String::from_utf8_lossy(&buf[..n]);
                    if let Some(line) = req.lines().next() {
                        let p: Vec<&str> = line.split_whitespace().collect();
                        if p.len() >= 2 {
                            event_bus.emit(ProxyEvent::RequestReceived {
                                method: p[0].to_string(),
                                url: p[1].to_string(),
                                host: hostname.to_string(),
                            });
                        }
                    }

                    // Forward to upstream
                    match TcpStream::connect(target).await {
                        Ok(mut upstream) => {
                            let _ = upstream.write_all(&buf[..n]).await;
                            let mut resp_buf = [0u8; 16384];
                            match upstream.read(&mut resp_buf).await {
                                Ok(n2) if n2 > 0 => {
                                    let _ = tls_stream.write_all(&resp_buf[..n2]).await;
                                    event_bus.emit(ProxyEvent::ResponseReceived {
                                        status: 200,
                                        url: hostname.to_string(),
                                    });
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            event_bus.emit(ProxyEvent::Error {
                                message: format!("Upstream connect failed: {e}"),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        Err(e) => {
            event_bus.emit(ProxyEvent::Error {
                message: format!("TLS accept failed: {e}"),
            });
        }
    }
}
