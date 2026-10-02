#![allow(dead_code)]

use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::proxy::errors::ProxyError;

#[derive(Debug, Clone)]
pub enum Upstream {
    Direct,
    Socks5 { addr: String, target_dns: bool },
}

/// Minimal SOCKS5 CONNECT client (no auth, CONNECT only).
/// Compatible with Tor (127.0.0.1:9050) and any SOCKS5 proxy.
pub async fn socks5_connect(
    proxy_addr: &str,
    target_host: &str,
    target_port: u16,
) -> Result<TcpStream, ProxyError> {
    let mut stream = TcpStream::connect(proxy_addr)
        .await
        .map_err(|e| ProxyError::Bind(format!("socks proxy connect failed: {e}")))?;

    stream
        .write_all(&[0x05, 0x01, 0x00])
        .await
        .map_err(socks_io("greeting"))?;

    let mut resp = [0u8; 2];
    stream
        .read_exact(&mut resp)
        .await
        .map_err(socks_io("greeting reply"))?;
    if resp[0] != 0x05 || resp[1] != 0x00 {
        return Err(ProxyError::Bind(format!(
            "socks proxy rejected auth: version={:02x} method={:02x}",
            resp[0], resp[1]
        )));
    }

    let host = target_host.as_bytes();
    let mut req = Vec::with_capacity(7 + host.len());
    req.extend_from_slice(&[0x05, 0x01, 0x00, 0x03]);
    req.push(host.len() as u8);
    req.extend_from_slice(host);
    req.extend_from_slice(&target_port.to_be_bytes());
    stream.write_all(&req).await.map_err(socks_io("connect"))?;

    let mut head = [0u8; 4];
    stream.read_exact(&mut head).await.map_err(socks_io("connect reply"))?;
    if head[1] != 0x00 {
        return Err(ProxyError::Bind(format!(
            "socks connect failed with reply code {:02x}",
            head[1]
        )));
    }

    let rest = match head[3] {
        0x01 => 4 + 2,
        0x03 => 1 + head[4] as usize + 2,
        0x04 => 16 + 2,
        other => return Err(ProxyError::Bind(format!("bad socks addr type {other:02x}"))),
    };
    let mut tail = vec![0u8; rest];
    stream
        .read_exact(&mut tail)
        .await
        .map_err(socks_io("connect reply addr"))?;

    Ok(stream)
}

fn socks_io(
    phase: &'static str,
) -> impl Fn(std::io::Error) -> ProxyError {
    move |e| ProxyError::Bind(format!("socks {phase} io error: {e}"))
}

pub async fn connect_upstream(upstream: &Arc<Upstream>, host: &str, port: u16) -> Result<TcpStream, ProxyError> {
    match upstream.as_ref() {
        Upstream::Direct => TcpStream::connect((host, port))
            .await
            .map_err(|e| ProxyError::Bind(format!("upstream connect failed: {e}"))),
        Upstream::Socks5 { addr, .. } => socks5_connect(addr, host, port).await,
    }
}
