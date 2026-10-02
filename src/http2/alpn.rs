#![allow(dead_code)]

use crate::network::protocol::{AlpnNegotiation, HttpVersion};

#[derive(Debug, Clone)]
pub struct AlpnHandler {
    supported: Vec<String>,
    last_negotiation: Option<AlpnNegotiation>,
}

impl AlpnHandler {
    pub fn new() -> Self {
        Self {
            supported: vec!["h2".into(), "http/1.1".into()],
            last_negotiation: None,
        }
    }

    pub fn with_h3_support(mut self) -> Self {
        self.supported.insert(0, "h3".into());
        self
    }

    pub fn supported_protocols(&self) -> &[String] {
        &self.supported
    }

    pub fn negotiate(&mut self, server_alpn: &str) -> HttpVersion {
        let negotiation = AlpnNegotiation {
            offered: self.supported.clone(),
            selected: Some(server_alpn.to_string()),
            supported: self.supported.clone(),
        };
        let version = negotiation
            .resolved_version()
            .unwrap_or(HttpVersion::Http11);
        self.last_negotiation = Some(negotiation);
        version
    }

    pub fn last_negotiation(&self) -> Option<&AlpnNegotiation> {
        self.last_negotiation.as_ref()
    }

    pub fn summary(&self) -> String {
        match &self.last_negotiation {
            Some(neg) => format!(
                "ALPN: offered={:?}, selected={:?}",
                neg.offered, neg.selected
            ),
            None => format!("ALPN: supported={:?}, not yet negotiated", self.supported),
        }
    }
}

impl Default for AlpnHandler {
    fn default() -> Self {
        Self::new()
    }
}
