#[allow(dead_code)]
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub user_agent: String,
    pub request_timeout: Duration,
    pub connection_timeout: Duration,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub tls_verify: bool,
    pub proxy_url: Option<String>,
    pub accept_invalid_certs: bool,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            user_agent: format!("Sentinel/{}", env!("CARGO_PKG_VERSION")),
            request_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            follow_redirects: true,
            max_redirects: 10,
            tls_verify: true,
            proxy_url: None,
            accept_invalid_certs: false,
        }
    }
}
