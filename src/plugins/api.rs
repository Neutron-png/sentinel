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
