#![allow(dead_code)]

use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_rustls::rustls::{pki_types::CertificateDer, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::network::protocol::AlpnNegotiation;
use crate::proxy::ca::CertificateAuthority;
use crate::proxy::errors::ProxyError;

pub struct TlsManager {
    ca: Arc<CertificateAuthority>,
    alpn_protocols: Vec<String>,
}

impl TlsManager {
    pub fn new(ca: Arc<CertificateAuthority>) -> Self {
        Self {
            ca,
            alpn_protocols: vec!["h2".into(), "http/1.1".into()],
        }
    }

    pub fn with_alpn(mut self, protocols: Vec<String>) -> Self {
        self.alpn_protocols = protocols;
        self
    }

    pub fn ca(&self) -> &CertificateAuthority {
        &self.ca
    }

    pub fn alpn_protocols(&self) -> &[String] {
        &self.alpn_protocols
    }

    pub fn negotiate_alpn(&self, client_alpn: &[u8]) -> AlpnNegotiation {
        let offered = if client_alpn.is_empty() {
            Vec::new()
        } else {
            client_alpn
                .split(|b| *b == 0x2c || *b == 0x20)
                .filter(|s| !s.is_empty())
                .map(|s| String::from_utf8_lossy(s).trim().to_string())
                .collect()
        };

        let selected = offered
            .iter()
            .find(|o| {
                self.alpn_protocols
                    .iter()
                    .any(|p| p.eq_ignore_ascii_case(o))
            })
            .cloned();

        AlpnNegotiation {
            offered,
            selected,
            supported: self.alpn_protocols.clone(),
        }
    }

    pub async fn accept_tls(
        &self,
        stream: TcpStream,
        hostname: &str,
    ) -> Result<tokio_rustls::server::TlsStream<TcpStream>, ProxyError> {
        let (cert_pem, key_pem) = self.ca.generate_leaf(hostname)?;

        let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_pem.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ProxyError::Tls(e.to_string()))?;

        let key = rustls_pemfile::private_key(&mut key_pem.as_bytes())
            .map_err(|e| ProxyError::Tls(e.to_string()))?
            .ok_or_else(|| ProxyError::Tls("No private key found".into()))?;

        let _alpn: Vec<Vec<u8>> = self
            .alpn_protocols
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| ProxyError::Tls(format!("Config: {e}")))?;

        let acceptor = TlsAcceptor::from(Arc::new(config));
        acceptor
            .accept(stream)
            .await
            .map_err(|e| ProxyError::Tls(e.to_string()))
    }
}
