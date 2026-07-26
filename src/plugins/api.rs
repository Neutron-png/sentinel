use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub supported_api_version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum PluginCapability {
    WorkflowExtension,
    EvidenceProvider,
    Importer,
    Exporter,
    Command,
    Integration,
    JwtDetection,
    JwtAnalysis,
    JwtManipulation,
    OAuthDetection,
    OAuthSessionAccess,
    OAuthTokenInspection,
    OidcMetadataAccess,
    GrpcDetection,
    GrpcMessageInspection,
    GrpcServiceEnumeration,
    GrpcProtoImport,
    WebSocketFrameAccess,
    WebSocketFrameInjection,
    WebSocketConversationReplay,
}

#[allow(dead_code)]
impl PluginCapability {
    pub fn label(&self) -> &'static str {
        match self {
            Self::WorkflowExtension => "Workflow Extension",
            Self::EvidenceProvider => "Evidence Provider",
            Self::Importer => "Importer",
            Self::Exporter => "Exporter",
            Self::Command => "Command",
            Self::Integration => "Integration",
            Self::JwtDetection => "JWT Detection",
            Self::JwtAnalysis => "JWT Analysis",
            Self::JwtManipulation => "JWT Manipulation",
            Self::OAuthDetection => "OAuth Detection",
            Self::OAuthSessionAccess => "OAuth Session Access",
            Self::OAuthTokenInspection => "OAuth Token Inspection",
            Self::OidcMetadataAccess => "OIDC Metadata Access",
            Self::GrpcDetection => "gRPC Detection",
            Self::GrpcMessageInspection => "gRPC Message Inspection",
            Self::GrpcServiceEnumeration => "gRPC Service Enumeration",
            Self::GrpcProtoImport => "gRPC Proto Import",
            Self::WebSocketFrameAccess => "WebSocket Frame Access",
            Self::WebSocketFrameInjection => "WebSocket Frame Injection",
            Self::WebSocketConversationReplay => "WebSocket Conversation Replay",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "WorkflowExtension" => Some(Self::WorkflowExtension),
            "EvidenceProvider" => Some(Self::EvidenceProvider),
            "Importer" => Some(Self::Importer),
            "Exporter" => Some(Self::Exporter),
            "Command" => Some(Self::Command),
            "Integration" => Some(Self::Integration),
            "JwtDetection" => Some(Self::JwtDetection),
            "JwtAnalysis" => Some(Self::JwtAnalysis),
            "JwtManipulation" => Some(Self::JwtManipulation),
            "OAuthDetection" => Some(Self::OAuthDetection),
            "OAuthSessionAccess" => Some(Self::OAuthSessionAccess),
            "OAuthTokenInspection" => Some(Self::OAuthTokenInspection),
            "OidcMetadataAccess" => Some(Self::OidcMetadataAccess),
            "GrpcDetection" => Some(Self::GrpcDetection),
            "GrpcMessageInspection" => Some(Self::GrpcMessageInspection),
            "GrpcServiceEnumeration" => Some(Self::GrpcServiceEnumeration),
            "GrpcProtoImport" => Some(Self::GrpcProtoImport),
            "WebSocketFrameAccess" => Some(Self::WebSocketFrameAccess),
            "WebSocketFrameInjection" => Some(Self::WebSocketFrameInjection),
            "WebSocketConversationReplay" => Some(Self::WebSocketConversationReplay),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub metadata: PluginMetadata,
    pub capabilities: Vec<String>,
}

pub const API_VERSION: &str = "1.0";

#[allow(dead_code)]
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn capabilities(&self) -> &[PluginCapability];
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}
