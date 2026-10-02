#![allow(dead_code)]

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::watch;

use crate::network::protocol::HttpVersion;
use crate::proxy::ca::CertificateAuthority;
use crate::proxy::config::ProxyConfig;
use crate::proxy::errors::ProxyError;
use crate::proxy::events::{EventBus, ProxyEvent};
use crate::proxy::tls::TlsManager;

use crate::proxy::connection;

pub struct ProxyServer {
    config: ProxyConfig,
    event_bus: EventBus,
    ca: Arc<CertificateAuthority>,
    shutdown_tx: Option<watch::Sender<bool>>,
    protocol_stats: ProtocolStats,
}

#[derive(Debug, Clone, Default)]
pub struct ProtocolStats {
    pub http11_count: u64,
    pub http2_count: u64,
    pub http3_count: u64,
    pub alpn_negotiations: Vec<String>,
}

impl ProtocolStats {
    pub fn record(&mut self, version: HttpVersion) {
        match version {
            HttpVersion::Http11 => self.http11_count += 1,
            HttpVersion::Http2 => self.http2_count += 1,
            HttpVersion::Http3 => self.http3_count += 1,
            _ => {}
        }
    }

    pub fn record_alpn(&mut self, alpn: &str) {
        self.alpn_negotiations.push(alpn.to_string());
    }
}

impl ProxyServer {
    pub fn new(config: ProxyConfig) -> Result<Self, ProxyError> {
        let ca = Arc::new(CertificateAuthority::generate()?);
        let event_bus = EventBus::new(1024);
        Ok(Self {
            config,
            event_bus,
            ca,
            shutdown_tx: None,
            protocol_stats: ProtocolStats::default(),
        })
    }

    pub fn event_bus(&self) -> EventBus {
        self.event_bus.clone()
    }
    pub fn ca(&self) -> Arc<CertificateAuthority> {
        self.ca.clone()
    }
    pub fn protocol_stats(&self) -> &ProtocolStats {
        &self.protocol_stats
    }

    pub async fn start(&mut self) -> Result<(), ProxyError> {
        let listener = TcpListener::bind(self.config.listen_addr)
            .await
            .map_err(|e| ProxyError::Bind(e.to_string()))?;

        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        self.shutdown_tx = Some(shutdown_tx);

        let event_bus = self.event_bus.clone();
        let tls = Arc::new(TlsManager::new(self.ca.clone()));
        let addr = self.config.listen_addr;
        let upstream = Arc::new(self.config.upstream.clone());
        let tls_intercept = self.config.tls_intercept;

        event_bus.emit(ProxyEvent::TlsEstablished {
            host: format!("Proxy listening on {addr}"),
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((stream, client_addr)) => {
                                let eb = event_bus.clone();
                                let t = tls.clone();
                                let up = upstream.clone();
                                let ti = tls_intercept;
                                let ca_str = client_addr.to_string();
                                tokio::spawn(async move {
                                    connection::handle_connection(stream, ca_str, eb, t, up, ti).await;
                                });
                            }
                            Err(e) => {
                                event_bus.emit(ProxyEvent::Error { message: e.to_string() });
                            }
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn shutdown(&self) -> Result<(), ProxyError> {
        if let Some(tx) = &self.shutdown_tx {
            let _ = tx.send(true);
        }
        Ok(())
    }

    pub fn shutdown_tx_clone(&self) -> Option<watch::Sender<bool>> {
        self.shutdown_tx.clone()
    }
}
