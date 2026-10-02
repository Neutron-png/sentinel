#![allow(dead_code)]

use crate::network::protocol::{HttpVersion, NegotiatedProtocol};
// chrono used elsewhere
use uuid::Uuid;

/// Protocol-specific evidence metadata
#[derive(Debug, Clone)]
pub struct ProtocolEvidence {
    pub protocol_version: HttpVersion,
    pub alpn_selected: Option<String>,
    pub connection_id: Option<Uuid>,
    pub stream_id: Option<u32>,
    pub frame_log_json: Option<String>,
    pub quic_version: Option<u32>,
    pub zero_rtt_accepted: Option<bool>,
    pub timing: ProtocolTiming,
}

impl ProtocolEvidence {
    pub fn from_negotiated(negotiated: &NegotiatedProtocol) -> Self {
        let frame_log_json = if negotiated.frame_log.is_empty() {
            None
        } else {
            serde_json::to_string(&negotiated.frame_log).ok()
        };

        Self {
            protocol_version: negotiated.version,
            alpn_selected: negotiated.alpn.as_ref().and_then(|a| a.selected.clone()),
            connection_id: Some(negotiated.connection_id),
            stream_id: negotiated.streams.last().map(|s| s.stream_id),
            frame_log_json,
            quic_version: None,
            zero_rtt_accepted: negotiated.zero_rtt_accepted,
            timing: ProtocolTiming::default(),
        }
    }

    pub fn from_version(version: HttpVersion) -> Self {
        Self {
            protocol_version: version,
            alpn_selected: None,
            connection_id: None,
            stream_id: None,
            frame_log_json: None,
            quic_version: None,
            zero_rtt_accepted: None,
            timing: ProtocolTiming::default(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} ALPN:{} Stream:{}",
            self.protocol_version,
            self.alpn_selected.as_deref().unwrap_or("none"),
            self.stream_id
                .map(|s| s.to_string())
                .unwrap_or_else(|| "none".into())
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProtocolTiming {
    pub tls_handshake_ms: Option<u64>,
    pub alpn_negotiation_ms: Option<u64>,
    pub first_byte_ms: Option<u64>,
    pub quic_connect_ms: Option<u64>,
}
