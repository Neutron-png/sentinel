#![allow(dead_code)]

use std::collections::HashMap;

use crate::sdk::errors::SdkError;
use crate::sdk::lifecycle::PluginLifecycleManager;
use crate::sdk::manifest::PluginManifest;

pub struct PluginEntry {
    pub id: String,
    pub manifest: PluginManifest,
    pub lifecycle: PluginLifecycleManager,
}

pub struct PluginLoader {
    plugins: HashMap<String, PluginEntry>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn load_from_manifest(&mut self, manifest: PluginManifest) -> Result<(), SdkError> {
        let id = manifest.id.clone();
        if self.plugins.contains_key(&id) {
            return Err(SdkError::Registry(format!("Plugin '{id}' already loaded")));
        }
        self.plugins.insert(
            id.clone(),
            PluginEntry {
                id,
                manifest,
                lifecycle: PluginLifecycleManager::new(),
            },
        );
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&PluginEntry> {
        self.plugins.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut PluginEntry> {
        self.plugins.get_mut(id)
    }
    pub fn all(&self) -> Vec<&PluginEntry> {
        self.plugins.values().collect()
    }
    pub fn remove(&mut self, id: &str) -> Option<PluginEntry> {
        self.plugins.remove(id)
    }

    pub fn initialize_all(&mut self) -> Vec<Result<(), SdkError>> {
        let ids: Vec<String> = self.plugins.keys().cloned().collect();
        let mut results = Vec::new();
        for id in &ids {
            if let Some(entry) = self.plugins.get_mut(id) {
                results.push(entry.lifecycle.initialize());
            }
        }
        results
    }
}
