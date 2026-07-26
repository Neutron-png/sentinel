#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsState {
    Connecting,
    Open,
    Closing,
    Closed,
    Error,
}

impl WsState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Connecting => "Connecting",
            Self::Open => "Open",
            Self::Closing => "Closing",
            Self::Closed => "Closed",
            Self::Error => "Error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsOpcode {
    Continuation,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl WsOpcode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x0 => Some(Self::Continuation),
            0x1 => Some(Self::Text),
            0x2 => Some(Self::Binary),
            0x8 => Some(Self::Close),
            0x9 => Some(Self::Ping),
            0xA => Some(Self::Pong),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Continuation => 0x0,
            Self::Text => 0x1,
            Self::Binary => 0x2,
            Self::Close => 0x8,
            Self::Ping => 0x9,
            Self::Pong => 0xA,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Continuation => "Cont",
            Self::Text => "Text",
            Self::Binary => "Bin",
            Self::Close => "Close",
            Self::Ping => "Ping",
            Self::Pong => "Pong",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsDirection {
    ClientToServer,
    ServerToClient,
}

#[derive(Debug, Clone)]
pub struct WsConnection {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub browser_tab_id: Option<Uuid>,
    pub url: String,
    pub state: WsState,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub subprotocol: Option<String>,
    pub extensions: Vec<String>,
}

impl WsConnection {
    pub fn new(url: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id: None,
            browser_tab_id: None,
            url: url.to_string(),
            state: WsState::Connecting,
            start_time: Utc::now(),
            end_time: None,
            subprotocol: None,
            extensions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct WsFrame {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub opcode: WsOpcode,
    pub payload: Vec<u8>,
    pub payload_length: u64,
    pub timestamp: DateTime<Utc>,
    pub direction: WsDirection,
    pub fin: bool,
    pub masked: bool,
}

impl WsFrame {
    pub fn new(
        connection_id: Uuid,
        opcode: WsOpcode,
        payload: Vec<u8>,
        direction: WsDirection,
        fin: bool,
        masked: bool,
    ) -> Self {
        let len = payload.len() as u64;
        Self {
            id: Uuid::new_v4(),
            connection_id,
            opcode,
            payload,
            payload_length: len,
            timestamp: Utc::now(),
            direction,
            fin,
            masked,
        }
    }

    pub fn payload_as_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.payload).ok()
    }

    pub fn payload_as_json(&self) -> Option<serde_json::Value> {
        self.payload_as_text()
            .and_then(|s| serde_json::from_str(s).ok())
    }
}

#[derive(Debug, Clone)]
pub struct WsHandshake {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub request_method: String,
    pub request_url: String,
    pub request_headers: Vec<(String, String)>,
    pub response_status: Option<u16>,
    pub response_headers: Vec<(String, String)>,
    pub subprotocol: Option<String>,
    pub extensions: Vec<String>,
    pub cookies: Vec<(String, String)>,
    pub tls_enabled: bool,
    pub timestamp: DateTime<Utc>,
}

impl WsHandshake {
    pub fn new(connection_id: Uuid, method: &str, url: &str) -> Self {
        WsHandshake {
            id: Uuid::new_v4(),
            connection_id,
            request_method: method.to_string(),
            request_url: url.to_string(),
            request_headers: Vec::new(),
            response_status: None,
            response_headers: Vec::new(),
            subprotocol: None,
            extensions: Vec::new(),
            cookies: Vec::new(),
            tls_enabled: url.starts_with("wss://"),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WsConversation {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub frames: Vec<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub frame_count: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

impl WsConversation {
    pub fn new(connection_id: Uuid) -> Self {
        WsConversation {
            id: Uuid::new_v4(),
            connection_id,
            frames: Vec::new(),
            started_at: Utc::now(),
            ended_at: None,
            frame_count: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
        }
    }

    pub fn add_frame(&mut self, frame_id: Uuid, byte_count: u64, _is_send: bool) {
        self.frames.push(frame_id);
        self.frame_count += 1;
        if _is_send {
            self.total_bytes_sent += byte_count;
        } else {
            self.total_bytes_received += byte_count;
        }
    }

    pub fn close(&mut self) {
        self.ended_at = Some(Utc::now());
    }
}
