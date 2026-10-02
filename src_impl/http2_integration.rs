#![allow(dead_code)]

use crate::http2::engine::Http2Engine;
use crate::http2::alpn::AlpnHandler;
use crate::http2::frames::FrameHandler;
use crate::network::protocol::{HttpVersion, NegotiatedProtocol, StreamMetadata, StreamDirection, StreamState};
use uuid::Uuid;

pub struct Http2Integration {
    engine: Http2Engine,
    alpn_handler: AlpnHandler,
    frame_handler: FrameHandler,
    current_connection_id: Option<Uuid>,
    stream_counter: u32,
}

impl Http2Integration {
    pub fn new(config: crate::network::protocol::Http2Config) -> Self {
        Self {
            engine: Http2Engine::new(config),
            alpn_handler: AlpnHandler::new(),
            frame_handler: FrameHandler::new(),
            current_connection_id: None,
            stream_counter: 0,
        }
    }

    pub fn with_h3_support(mut self) -> Self {
        self.alpn_handler = self.alpn_handler.with_h3_support();
        self
    }

    pub fn engine(&self) -> &Http2Engine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut Http2Engine {
        &mut self.engine
    }

    pub fn negotiate_connection(&mut self, authority: &str, server_alpn: &str) -> NegotiatedProtocol {
        let version = self.alpn_handler.negotiate(server_alpn);
        let connection_id = Uuid::new_v4();
        self.current_connection_id = Some(connection_id);

        match version {
            HttpVersion::Http2 => {
                let negotiated = NegotiatedProtocol::new(version, connection_id);
                self.engine.create_connection(authority.to_string(), &negotiated);
                negotiated
            }
            _ => {
                let mut negotiated = NegotiatedProtocol::new(version, connection_id);
                if let Some(alpn) = self.alpn_handler.last_negotiation() {
                    negotiated.alpn = Some(alpn.clone());
                }
                negotiated
            }
        }
    }

    pub fn detect_protocol(&self, first_bytes: &[u8]) -> HttpVersion {
        // Check for HTTP/2 connection preface
        if first_bytes.starts_with(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n") {
            return HttpVersion::Http2;
        }
        // Check for QUIC (HTTP/3) - QUIC initial packet starts with header form bit 1,
        // but for detection we check if it looks like HTTP/1.x first
        if first_bytes.len() >= 1 && (first_bytes[0] & 0x80) != 0 {
            // QUIC long header - likely HTTP/3
            return HttpVersion::Http3;
        }
        HttpVersion::Http11
    }

    pub fn create_stream_metadata(&mut self, direction: StreamDirection) -> StreamMetadata {
        self.stream_counter += 1;
        let stream_id = self.stream_counter;
        StreamMetadata {
            stream_id,
            connection_id: self.current_connection_id.unwrap_or_else(Uuid::new_v4),
            protocol: HttpVersion::Http2,
            direction,
            state: StreamState::Idle,
            priority: None,
            dependency: None,
            weight: None,
            window_size: 65535,
            bytes_sent: 0,
            bytes_received: 0,
            created_at: chrono::Utc::now(),
            closed_at: None,
        }
    }

    pub fn alpn_summary(&self) -> String {
        self.alpn_handler.summary()
    }

    pub fn frame_handler(&mut self) -> &mut FrameHandler {
        &mut self.frame_handler
    }
}

impl Default for Http2Integration {
    fn default() -> Self {
        Self::new(crate::network::protocol::Http2Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_http2_preface() {
        let integration = Http2Integration::new(Default::default());
        let preface = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
        assert_eq!(integration.detect_protocol(preface), HttpVersion::Http2);
    }

    #[test]
    fn test_detect_http11() {
        let integration = Http2Integration::new(Default::default());
        let request = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        assert_eq!(integration.detect_protocol(request), HttpVersion::Http11);
    }

    #[test]
    fn test_create_stream_metadata() {
        let mut integration = Http2Integration::new(Default::default());
        let meta = integration.create_stream_metadata(StreamDirection::Outbound);
        assert_eq!(meta.stream_id, 1);
        assert_eq!(meta.protocol, HttpVersion::Http2);
    }
}
