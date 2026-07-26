#![allow(dead_code)]

use uuid::Uuid;

use crate::browser::proxy::session::BrowserProxySession;

#[derive(Default)]
pub struct BrowserRoutingRules {
    pub bypass_localhost: bool,
    pub bypass_intranet: bool,
    pub additional_bypass: Vec<String>,
}

impl BrowserRoutingRules {
    pub fn should_proxy(&self, _url: &str) -> bool {
        true
    }
    pub fn route_through_pipeline(
        &self,
        session: &BrowserProxySession,
        browser_id: Uuid,
        tab_id: Uuid,
        url: &str,
        method: &str,
    ) -> RouteResult {
        if !session.attached {
            return RouteResult::Direct;
        }
        RouteResult::ThroughProxy {
            proxy_addr: session.proxy_address(),
            browser_id,
            tab_id,
            url: url.to_string(),
            method: method.to_string(),
            tls_intercept: session.tls_intercept,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RouteResult {
    Direct,
    ThroughProxy {
        proxy_addr: String,
        browser_id: Uuid,
        tab_id: Uuid,
        url: String,
        method: String,
        tls_intercept: bool,
    },
}
