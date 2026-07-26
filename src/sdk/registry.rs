#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    ActiveScanRule,
    PassiveScanRule,
    PayloadProvider,
    AuthenticationProvider,
    WorkflowStep,
    ReportTemplate,
    BrowserExtension,
    UiPanel,
    ContextMenu,
    SidebarModule,
    CliCommand,
    Importer,
    Exporter,
}

impl PluginType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active_scan_rule" => Some(Self::ActiveScanRule),
            "passive_scan_rule" => Some(Self::PassiveScanRule),
            "payload_provider" => Some(Self::PayloadProvider),
            "auth_provider" => Some(Self::AuthenticationProvider),
            "workflow_step" => Some(Self::WorkflowStep),
            "report_template" => Some(Self::ReportTemplate),
            "browser_extension" => Some(Self::BrowserExtension),
            "ui_panel" => Some(Self::UiPanel),
            "context_menu" => Some(Self::ContextMenu),
            "sidebar" => Some(Self::SidebarModule),
            "cli_command" => Some(Self::CliCommand),
            "importer" => Some(Self::Importer),
            "exporter" => Some(Self::Exporter),
            _ => None,
        }
    }
}

pub struct CapabilityRegistry {
    capabilities: Vec<(PluginType, String)>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }
    pub fn register(&mut self, ptype: PluginType, plugin_id: &str) {
        self.capabilities.push((ptype, plugin_id.to_string()));
    }
    pub fn find_providers(&self, ptype: PluginType) -> Vec<&String> {
        self.capabilities
            .iter()
            .filter(|(t, _)| *t == ptype)
            .map(|(_, id)| id)
            .collect()
    }
}
