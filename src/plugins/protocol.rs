#![allow(dead_code)]

use crate::network::protocol::{FrameMetadata, HttpVersion, NegotiatedProtocol, StreamMetadata};

/// API exposed to plugins for protocol inspection
pub struct ProtocolAccess {
    pub version: HttpVersion,
    pub alpn_selected: Option<String>,
    pub connection_id: Option<uuid::Uuid>,
    pub streams: Vec<StreamMetadata>,
    pub frames: Vec<FrameMetadata>,
    pub server_push_count: usize,
    pub zero_rtt_accepted: Option<bool>,
}

impl ProtocolAccess {
    pub fn from_negotiated(negotiated: &NegotiatedProtocol) -> Self {
        Self {
            version: negotiated.version,
            alpn_selected: negotiated.alpn.as_ref().and_then(|a| a.selected.clone()),
            connection_id: Some(negotiated.connection_id),
            streams: negotiated.streams.clone(),
            frames: negotiated.frame_log.clone(),
            server_push_count: negotiated.server_push.len(),
            zero_rtt_accepted: negotiated.zero_rtt_accepted,
        }
    }

    pub fn version_label(&self) -> &'static str {
        self.version.label()
    }

    pub fn is_http2_or_higher(&self) -> bool {
        self.version.is_http2_or_higher()
    }

    pub fn frame_count_by_type(&self) -> Vec<(String, usize)> {
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for frame in &self.frames {
            *counts
                .entry(frame.frame_type.label().to_string())
                .or_default() += 1;
        }
        let mut result: Vec<_> = counts.into_iter().collect();
        result.sort_by_key(|b| std::cmp::Reverse(b.1));
        result
    }

    pub fn stream_count(&self) -> usize {
        self.streams.len()
    }

    pub fn alpn_info(&self) -> Option<&str> {
        self.alpn_selected.as_deref()
    }
}

/// Additional plugin capabilities for protocol inspection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCapability {
    Http2StreamInspection,
    Http2FrameCapture,
    Http3QuicInspection,
    AlpnControl,
    ServerPushDetection,
}

impl ProtocolCapability {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Http2StreamInspection => "HTTP/2 Stream Inspection",
            Self::Http2FrameCapture => "HTTP/2 Frame Capture",
            Self::Http3QuicInspection => "HTTP/3 QUIC Inspection",
            Self::AlpnControl => "ALPN Control",
            Self::ServerPushDetection => "Server Push Detection",
        }
    }
}
