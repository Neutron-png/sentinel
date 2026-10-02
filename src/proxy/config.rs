#![allow(dead_code)]

use std::net::SocketAddr;

use crate::proxy::socks::Upstream;

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub listen_addr: SocketAddr,
    pub ca_cert_path: Option<String>,
    pub ca_key_path: Option<String>,
    pub tls_intercept: bool,
    pub verify_upstream_certs: bool,
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub upstream: Upstream,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
            ca_cert_path: None,
            ca_key_path: None,
            tls_intercept: true,
            verify_upstream_certs: false,
            max_connections: 256,
            connection_timeout_secs: 30,
            upstream: Upstream::Direct,
        }
    }
}
