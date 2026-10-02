#![allow(dead_code)]

use crate::network::protocol::{AlpnNegotiation, HttpVersion};

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
        let version = negotiation.resolved_version().unwrap_or(HttpVersion::Http11);
        self.last_negotiation = Some(negotiation);
        version
    }

    pub fn last_negotiation(&self) -> Option<&AlpnNegotiation> {
        self.last_negotiation.as_ref()
    }

    pub fn detect_protocol_from_settings(&self, settings_frame: &[u8]) -> Option<HttpVersion> {
        if settings_frame.len() >= 6 {
            let frame_type = settings_frame[3];
            if frame_type == 0x04 {
                return Some(HttpVersion::Http2);
            }
        }
        None
    }

    pub fn summary(&self) -> String {
        match &self.last_negotiation {
            Some(neg) => format!(
                "ALPN: offered={:?}, selected={:?}",
                neg.offered, neg.selected
            ),
            None => format!(
                "ALPN: supported={:?}, not yet negotiated",
                self.supported
            ),
        }
    }
}

impl Default for AlpnHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negotiate_h2() {
        let mut handler = AlpnHandler::new();
        let version = handler.negotiate("h2");
        assert_eq!(version, HttpVersion::Http2);
    }

    #[test]
    fn test_negotiate_h3() {
        let mut handler = AlpnHandler::new().with_h3_support();
        let version = handler.negotiate("h3");
        assert_eq!(version, HttpVersion::Http3);
    }

    #[test]
    fn test_negotiate_http11() {
        let mut handler = AlpnHandler::new();
        let version = handler.negotiate("http/1.1");
        assert_eq!(version, HttpVersion::Http11);
    }

    #[test]
    fn test_detect_http2_from_settings() {
        let handler = AlpnHandler::new();
        let settings = vec![0, 0, 6, 0x04, 0, 0, 0, 0, 100];
        let version = handler.detect_protocol_from_settings(&settings);
        assert_eq!(version, Some(HttpVersion::Http2));
    }
}
