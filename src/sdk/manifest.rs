#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub sdk_version: String,
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub min_sentinel_version: String,
    pub max_sentinel_version: Option<String>,
    pub permissions: Vec<String>,
    pub dependencies: Vec<PluginDependency>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Network,
    Filesystem,
    Browser,
    Reports,
    Evidence,
    Scanner,
    Authentication,
    Workflow,
    Settings,
}

impl Permission {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "network" => Some(Self::Network),
            "filesystem" => Some(Self::Filesystem),
            "browser" => Some(Self::Browser),
            "reports" => Some(Self::Reports),
            "evidence" => Some(Self::Evidence),
            "scanner" => Some(Self::Scanner),
            "authentication" => Some(Self::Authentication),
            "workflow" => Some(Self::Workflow),
            "settings" => Some(Self::Settings),
            _ => None,
        }
    }
}
