use std::path::Path;

use anyhow::Context;

use crate::plugins::api::PluginManifest;

pub struct PluginLoader;

impl PluginLoader {
    pub fn discover(dir: &Path) -> anyhow::Result<Vec<PluginManifest>> {
        if !dir.exists() {
            std::fs::create_dir_all(dir).context("Failed to create plugins directory")?;
            return Ok(Vec::new());
        }

        let mut manifests = Vec::new();
        let entries = std::fs::read_dir(dir).context("Failed to read plugins directory")?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(manifest) = serde_json::from_str::<PluginManifest>(&content) {
                        manifests.push(manifest);
                    }
                }
            }
        }

        Ok(manifests)
    }
}
