#![allow(dead_code)]

use crate::network::protocol::HttpVersion;

/// Protocol information available to scanner rules
#[derive(Debug, Clone)]
pub struct ProtocolContext {
    pub version: HttpVersion,
    pub alpn_selected: Option<String>,
    pub connection_id: Option<uuid::Uuid>,
    pub stream_id: Option<u32>,
    pub supports_multiplexing: bool,
    pub supports_server_push: bool,
    pub is_quic: bool,
}

impl ProtocolContext {
    pub fn for_version(version: HttpVersion) -> Self {
        Self {
            version,
            alpn_selected: None,
            connection_id: None,
            stream_id: None,
            supports_multiplexing: version.supports_multiplexing(),
            supports_server_push: version.supports_server_push(),
            is_quic: matches!(version, HttpVersion::Http3),
        }
    }

    pub fn from_negotiated(negotiated: &crate::network::protocol::NegotiatedProtocol) -> Self {
        Self {
            version: negotiated.version,
            alpn_selected: negotiated.alpn.as_ref().and_then(|a| a.selected.clone()),
            connection_id: Some(negotiated.connection_id),
            stream_id: negotiated.streams.last().map(|s| s.stream_id),
            supports_multiplexing: negotiated.version.supports_multiplexing(),
            supports_server_push: negotiated.version.supports_server_push(),
            is_quic: matches!(negotiated.version, HttpVersion::Http3),
        }
    }
}
