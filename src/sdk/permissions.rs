#![allow(dead_code)]

use std::collections::HashSet;

use crate::sdk::manifest::PluginManifest;

pub struct PermissionManager {
    granted: HashSet<String>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            granted: HashSet::new(),
        }
    }

    pub fn grant(&mut self, permission: &str) {
        self.granted.insert(permission.to_string());
    }

    pub fn revoke(&mut self, permission: &str) {
        self.granted.remove(permission);
    }

    pub fn has(&self, permission: &str) -> bool {
        self.granted.contains(permission)
    }

    pub fn check_manifest(&self, manifest: &PluginManifest) -> bool {
        manifest.permissions.iter().all(|p| self.has(p))
    }

    pub fn has_all(&self, permissions: &[String]) -> bool {
        permissions.iter().all(|p| self.has(p))
    }
}
