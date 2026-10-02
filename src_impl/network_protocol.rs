#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// HTTP protocol version with native support for all three variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpVersion {
    #[serde(rename = "HTTP/0.9")]
    Http09,
    #[serde(rename = "HTTP/1.0")]
    Http10,
    #[serde(rename = "HTTP/1.1")]
    Http11,
    #[serde(rename = "HTTP/2")]
    Http2,
    #[serde(rename = "HTTP/3")]
    Http3,
}

impl HttpVersion {
    pub const ALL: &'static [HttpVersion] = &[
        Self::Http09,
        Self::Http10,
        Self::Http11,
        Self::Http2,
        Self::Http3,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Http09 => "HTTP/0.9",
            Self::Http10 => "HTTP/1.0",
            Self::Http11 => "HTTP/1.1",
            Self::Http2 => "HTTP/2",
            Self::Http3 => "HTTP/3",
        }
    }

    pub fn from_protocol_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "HTTP/2" | "H2" | "HTTP/2.0" => Self::Http2,
            "HTTP/3" | "H3" | "HTTP/3.0" => Self::Http3,
            "HTTP/1.1" | "HTTP/1.1" => Self::Http11,
            "HTTP/1.0" => Self::Http10,
            "HTTP/0.9" => Self::Http09,
            _ => Self::Http11,
        }
    }

    pub fn from_reqwest_version(v: &str) -> Self {
        match v {
            "HTTP/2.0" | "HTTP/2" | "h2" => Self::Http2,
            "HTTP/3.0" | "HTTP/3" | "h3" => Self::Http3,
            "HTTP/1.1" => Self::Http11,
            "HTTP/1.0" => Self::Http10,
            _ => Self::Http11,
        }
    }

    pub fn is_http2_or_higher(&self) -> bool {
        matches!(self, Self::Http2 | Self::Http3)
    }

    pub fn supports_multiplexing(&self) -> bool {
        matches!(self, Self::Http2 | Self::Http3)
    }

    pub fn supports_server_push(&self) -> bool {
        matches!(self, Self::Http2)
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// ALPN protocol identifiers negotiated during TLS handshake
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlpnNegotiation {
    pub offered: Vec<String>,
    pub selected: Option<String>,
    pub supported: Vec<String>,
}

impl Default for AlpnNegotiation {
    fn default() -> Self {
        Self {
            offered: vec!["h2".into(), "http/1.1".into()],
            selected: None,
            supported: vec!["h2".into(), "h3".into(), "http/1.1".into()],
        }
    }
}

impl AlpnNegotiation {
    pub fn resolved_version(&self) -> Option<HttpVersion> {
        self.selected.as_ref().map(|s| match s.as_str() {
            "h2" => HttpVersion::Http2,
            "h3" => HttpVersion::Http3,
            "http/1.1" | "http/1.0" => HttpVersion::Http11,
            _ => HttpVersion::Http11,
        })
    }
}

/// Metadata about an individual HTTP/2 or HTTP/3 stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub stream_id: u32,
    pub connection_id: Uuid,
    pub protocol: HttpVersion,
    pub direction: StreamDirection,
    pub state: StreamState,
    pub priority: Option<u8>,
    pub dependency: Option<u32>,
    pub weight: Option<u8>,
    pub window_size: u32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamDirection {
    Outbound,
    Inbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamState {
    Idle,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed,
}

/// Metadata about an individual HTTP/2 frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub frame_type: HttpFrameType,
    pub stream_id: u32,
    pub flags: u8,
    pub length: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpFrameType {
    Data,
    Headers,
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    Goaway,
    WindowUpdate,
    Continuation,
    Unknown(u8),
}

impl HttpFrameType {
    pub fn from_code(code: u8) -> Self {
        match code {
            0x0 => Self::Data,
            0x1 => Self::Headers,
            0x2 => Self::Priority,
            0x3 => Self::RstStream,
            0x4 => Self::Settings,
            0x5 => Self::PushPromise,
            0x6 => Self::Ping,
            0x7 => Self::Goaway,
            0x8 => Self::WindowUpdate,
            0x9 => Self::Continuation,
            c => Self::Unknown(c),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Data => "DATA",
            Self::Headers => "HEADERS",
            Self::Priority => "PRIORITY",
            Self::RstStream => "RST_STREAM",
            Self::Settings => "SETTINGS",
            Self::PushPromise => "PUSH_PROMISE",
            Self::Ping => "PING",
            Self::Goaway => "GOAWAY",
            Self::WindowUpdate => "WINDOW_UPDATE",
            Self::Continuation => "CONTINUATION",
            Self::Unknown(_) => "UNKNOWN",
        }
    }
}

/// Complete negotiated protocol information for a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedProtocol {
    pub version: HttpVersion,
    pub alpn: Option<AlpnNegotiation>,
    pub connection_id: Uuid,
    pub streams: Vec<StreamMetadata>,
    pub frame_log: Vec<FrameMetadata>,
    pub server_push: Vec<ServerPushMetadata>,
    pub quic_connection_id: Option<Vec<u8>>,
    pub zero_rtt_accepted: Option<bool>,
}

impl NegotiatedProtocol {
    pub fn new(version: HttpVersion, connection_id: Uuid) -> Self {
        Self {
            version,
            alpn: None,
            connection_id,
            streams: Vec::new(),
            frame_log: Vec::new(),
            server_push: Vec::new(),
            quic_connection_id: None,
            zero_rtt_accepted: None,
        }
    }

    pub fn with_alpn(mut self, alpn: AlpnNegotiation) -> Self {
        self.alpn = Some(alpn);
        self
    }

    pub fn record_frame(&mut self, frame: FrameMetadata) {
        self.frame_log.push(frame);
    }

    pub fn record_stream(&mut self, stream: StreamMetadata) {
        self.streams.push(stream);
    }

    pub fn record_server_push(&mut self, push: ServerPushMetadata) {
        self.server_push.push(push);
    }
}

/// Metadata about HTTP/2 server push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPushMetadata {
    pub promised_stream_id: u32,
    pub parent_stream_id: u32,
    pub method: String,
    pub path: String,
    pub authority: String,
    pub scheme: String,
    pub headers: Vec<(String, String)>,
    pub accepted: bool,
    pub completed: bool,
}

/// Protocol configuration for HTTP/2 connections
#[derive(Debug, Clone)]
pub struct Http2Config {
    pub max_frame_size: u32,
    pub max_concurrent_streams: u32,
    pub initial_window_size: u32,
    pub enable_push: bool,
    pub max_header_list_size: u32,
    pub enable_prior_knowledge: bool,
}

impl Default for Http2Config {
    fn default() -> Self {
        Self {
            max_frame_size: 16384,
            max_concurrent_streams: 256,
            initial_window_size: 65535,
            enable_push: true,
            max_header_list_size: 16777216,
            enable_prior_knowledge: false,
        }
    }
}

/// Protocol configuration for HTTP/3 connections
#[derive(Debug, Clone)]
pub struct Http3Config {
    pub max_stream_data: u64,
    pub max_data: u64,
    pub idle_timeout_ms: u64,
    pub enable_zero_rtt: bool,
    pub quic_version: u32,
    pub enable_migration: bool,
}

impl Default for Http3Config {
    fn default() -> Self {
        Self {
            max_stream_data: 1_048_576,
            max_data: 10_485_760,
            idle_timeout_ms: 30000,
            enable_zero_rtt: false,
            quic_version: 1,
            enable_migration: false,
        }
    }
}

/// HTTP/2 specific error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Http2ErrorCode {
    NoError = 0x0,
    ProtocolError = 0x1,
    InternalError = 0x2,
    FlowControlError = 0x3,
    SettingsTimeout = 0x4,
    StreamClosed = 0x5,
    FrameSizeError = 0x6,
    RefusedStream = 0x7,
    Cancel = 0x8,
    CompressionError = 0x9,
    ConnectError = 0xa,
    EnhanceYourCalm = 0xb,
    InadequateSecurity = 0xc,
    Http11Required = 0xd,
}

impl Http2ErrorCode {
    pub fn from_u32(code: u32) -> Self {
        match code {
            0x0 => Self::NoError,
            0x1 => Self::ProtocolError,
            0x2 => Self::InternalError,
            0x3 => Self::FlowControlError,
            0x4 => Self::SettingsTimeout,
            0x5 => Self::StreamClosed,
            0x6 => Self::FrameSizeError,
            0x7 => Self::RefusedStream,
            0x8 => Self::Cancel,
            0x9 => Self::CompressionError,
            0xa => Self::ConnectError,
            0xb => Self::EnhanceYourCalm,
            0xc => Self::InadequateSecurity,
            0xd => Self::Http11Required,
            _ => Self::InternalError,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}
