#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BrowserProxySession {
    pub browser_id: Uuid,
    pub proxy_port: u16,
    pub proxy_host: String,
    pub tls_intercept: bool,
    pub verify_certs: bool,
    pub scope_enabled: bool,
    pub history_enabled: bool,
    pub attached: bool,
    pub cookie_jar_id: String,
}

impl BrowserProxySession {
    pub fn new(browser_id: Uuid, proxy_port: u16) -> Self {
        Self {
            browser_id,
            proxy_port,
            proxy_host: "127.0.0.1".into(),
            tls_intercept: true,
            verify_certs: false,
            scope_enabled: true,
            history_enabled: true,
            attached: false,
            cookie_jar_id: format!("browser-cookies-{browser_id}"),
        }
    }

    pub fn proxy_address(&self) -> String {
        format!("{}:{}", self.proxy_host, self.proxy_port)
    }
}
