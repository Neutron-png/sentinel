#![allow(dead_code)]

use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_rustls::rustls::{pki_types::CertificateDer, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::proxy::ca::CertificateAuthority;
use crate::proxy::errors::ProxyError;

pub struct TlsManager {
    ca: Arc<CertificateAuthority>,
}

impl TlsManager {
    pub fn new(ca: Arc<CertificateAuthority>) -> Self {
        Self { ca }
    }

    pub fn ca(&self) -> &CertificateAuthority {
        &self.ca
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

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| ProxyError::Tls(e.to_string()))?;

        let acceptor = TlsAcceptor::from(Arc::new(config));
        acceptor
            .accept(stream)
            .await
            .map_err(|e| ProxyError::Tls(e.to_string()))
    }
}
