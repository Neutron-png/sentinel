use std::path::Path;

use anyhow::{Context, Result};

use crate::plugins::api::{Plugin, PluginManifest, API_VERSION};
use crate::plugins::loader::PluginLoader;
use crate::plugins::registry::PluginRegistry;

pub struct PluginManager {
    registry: PluginRegistry,
    discovered: Vec<PluginManifest>,
}

#[allow(dead_code)]
impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            discovered: Vec::new(),
        }
    }

    pub fn discover(&mut self, plugins_dir: &Path) -> Result<usize> {
        self.discovered = PluginLoader::discover(plugins_dir)?;
        Ok(self.discovered.len())
    }

    pub fn validate_manifests(&self) -> Vec<(&PluginManifest, Option<String>)> {
        self.discovered
            .iter()
            .map(|m| {
                let err = if m.metadata.id.is_empty() {
                    Some("Missing plugin id".into())
                } else if m.metadata.name.is_empty() {
                    Some("Missing plugin name".into())
                } else if m.metadata.supported_api_version != API_VERSION {
                    Some(format!(
                        "API version mismatch: plugin requires {}, core supports {}",
                        m.metadata.supported_api_version, API_VERSION
                    ))
                } else {
                    None
                };
                (m, err)
            })
            .collect()
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.registry.register(plugin);
    }

    pub fn register_compatible(&mut self, mut plugin: Box<dyn Plugin>) -> Result<()> {
        let meta = plugin.metadata().clone();
        if meta.supported_api_version != API_VERSION {
            anyhow::bail!(
                "Plugin '{}' requires API {}, core is {}",
                meta.name,
                meta.supported_api_version,
                API_VERSION
            );
        }
        plugin
            .initialize()
            .with_context(|| format!("Failed to initialize plugin '{}'", meta.name))?;
        self.registry.register(plugin);
        Ok(())
    }

    pub fn initialize_all(&mut self) -> Vec<(String, Result<()>)> {
        let mut results = Vec::new();
        for plugin in self.registry.iter_mut() {
            let name = plugin.metadata().name.clone();
            let result = plugin.initialize();
            results.push((name, result));
        }
        results.retain(|(_, r)| r.is_err());
        results
    }

    pub fn shutdown_all(&mut self) -> Vec<(String, Result<()>)> {
        let mut results = Vec::new();
        for plugin in self.registry.iter_mut() {
            let name = plugin.metadata().name.clone();
            let result = plugin.shutdown();
            results.push((name, result));
        }
        results.retain(|(_, r)| r.is_err());
        results
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn discovered_manifests(&self) -> &[PluginManifest] {
        &self.discovered
    }

    pub fn plugin_count(&self) -> usize {
        self.registry.count()
    }
}
