use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrpcCallType {
    Unary,
    ServerStreaming,
    ClientStreaming,
    BidiStreaming,
    Unknown,
}

impl GrpcCallType {
    pub fn label(&self) -> &'static str {
        match self {
            GrpcCallType::Unary => "Unary",
            GrpcCallType::ServerStreaming => "Server Streaming",
            GrpcCallType::ClientStreaming => "Client Streaming",
            GrpcCallType::BidiStreaming => "Bidirectional Streaming",
            GrpcCallType::Unknown => "Unknown",
        }
    }

    pub fn from_path(path: &str) -> Self {
        if path.to_lowercase().contains("stream") || path.to_lowercase().contains("subscribe") {
            GrpcCallType::ServerStreaming
        } else {
            GrpcCallType::Unary
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcService {
    pub name: String,
    pub package: Option<String>,
    pub methods: Vec<GrpcMethod>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcMethod {
    pub name: String,
    pub full_name: String,
    pub input_type: String,
    pub output_type: String,
    pub client_streaming: bool,
    pub server_streaming: bool,
}

impl GrpcMethod {
    pub fn call_type(&self) -> GrpcCallType {
        match (self.client_streaming, self.server_streaming) {
            (false, false) => GrpcCallType::Unary,
            (false, true) => GrpcCallType::ServerStreaming,
            (true, false) => GrpcCallType::ClientStreaming,
            (true, true) => GrpcCallType::BidiStreaming,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcFrame {
    pub compressed: bool,
    pub length: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcMessage {
    pub id: Uuid,
    pub service: Option<String>,
    pub method: Option<String>,
    pub call_type: GrpcCallType,
    pub request_frames: Vec<GrpcFrame>,
    pub response_frames: Vec<GrpcFrame>,
    pub metadata: Vec<(String, String)>,
    pub response_metadata: Vec<(String, String)>,
    pub trailers: Vec<(String, String)>,
    pub grpc_status: Option<i32>,
    pub grpc_message: Option<String>,
    pub fields: Vec<ProtobufField>,
    pub response_fields: Vec<ProtobufField>,
    pub url: Option<String>,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufField {
    pub field_number: u32,
    pub wire_type: ProtobufWireType,
    pub name: Option<String>,
    pub value: ProtobufValue,
    pub path: Vec<u32>,
    pub size_bytes: usize,
}

impl ProtobufField {
    pub fn is_sensitive(&self) -> bool {
        if let Some(ref name) = self.name {
            let lower = name.to_lowercase();
            lower.contains("password")
                || lower.contains("secret")
                || lower.contains("token")
                || lower.contains("key")
                || lower.contains("credential")
                || lower.contains("ssn")
                || lower.contains("credit")
                || lower == "pwd"
                || lower == "pass"
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtobufWireType {
    Varint,
    Fixed64,
    LengthDelimited,
    StartGroup,
    EndGroup,
    Fixed32,
    Unknown(u32),
}

impl ProtobufWireType {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0 => ProtobufWireType::Varint,
            1 => ProtobufWireType::Fixed64,
            2 => ProtobufWireType::LengthDelimited,
            3 => ProtobufWireType::StartGroup,
            4 => ProtobufWireType::EndGroup,
            5 => ProtobufWireType::Fixed32,
            n => ProtobufWireType::Unknown(n),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ProtobufWireType::Varint => "varint",
            ProtobufWireType::Fixed64 => "fixed64",
            ProtobufWireType::LengthDelimited => "length-delimited",
            ProtobufWireType::StartGroup => "start-group",
            ProtobufWireType::EndGroup => "end-group",
            ProtobufWireType::Fixed32 => "fixed32",
            ProtobufWireType::Unknown(_) => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProtobufValue {
    Varint(u64),
    Fixed64(u64),
    LengthDelimited(Vec<u8>),
    Fixed32(u32),
    Unknown(Vec<u8>),
}

impl ProtobufValue {
    pub fn as_string_lossy(&self) -> String {
        match self {
            ProtobufValue::LengthDelimited(data) => String::from_utf8_lossy(data).into_owned(),
            ProtobufValue::Varint(v) => v.to_string(),
            ProtobufValue::Fixed64(v) => v.to_string(),
            ProtobufValue::Fixed32(v) => v.to_string(),
            ProtobufValue::Unknown(data) => data
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(""),
        }
    }

    pub fn size_hint(&self) -> usize {
        match self {
            ProtobufValue::LengthDelimited(data) => data.len(),
            _ => 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcAnalysisFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
    pub service: Option<String>,
    pub method: Option<String>,
    pub evidence: String,
}

pub struct ProtoSchema {
    pub services: Vec<GrpcService>,
    pub messages: Vec<ProtoMessageDef>,
    pub package: Option<String>,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoMessageDef {
    pub name: String,
    pub full_name: String,
    pub fields: Vec<ProtoFieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoFieldDef {
    pub name: String,
    pub number: u32,
    pub field_type: String,
    pub label: String,
}
