#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub verify_certificate: bool,
    pub accept_invalid_hostnames: bool,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
    pub ca_certificate: Option<String>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_certificate: true,
            accept_invalid_hostnames: false,
            client_certificate: None,
            client_key: None,
            ca_certificate: None,
        }
    }
}
