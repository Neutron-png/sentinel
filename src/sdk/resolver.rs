#![allow(dead_code)]

use crate::sdk::manifest::PluginManifest;

pub fn resolve_dependencies(plugins: &[PluginManifest]) -> Result<Vec<&str>, (String, String)> {
    let ids: Vec<&str> = plugins.iter().map(|p| p.id.as_str()).collect();
    for plugin in plugins {
        for dep in &plugin.dependencies {
            if dep.required && !ids.contains(&dep.plugin_id.as_str()) {
                return Err((
                    plugin.id.clone(),
                    format!("Missing required dependency: {}", dep.plugin_id),
                ));
            }
        }
    }
    if has_cycles(plugins) {
        return Err(("cycle".into(), "Cyclic dependency detected".into()));
    }
    Ok(ids)
}

fn has_cycles(plugins: &[PluginManifest]) -> bool {
    let mut visited = std::collections::HashSet::new();
    for p in plugins {
        if detect_cycle(
            p,
            plugins,
            &mut visited,
            &mut std::collections::HashSet::new(),
        ) {
            return true;
        }
    }
    false
}

fn detect_cycle(
    plugin: &PluginManifest,
    all: &[PluginManifest],
    visited: &mut std::collections::HashSet<String>,
    stack: &mut std::collections::HashSet<String>,
) -> bool {
    let id = plugin.id.clone();
    if stack.contains(&id) {
        return true;
    }
    if visited.contains(&id) {
        return false;
    }
    visited.insert(id.clone());
    stack.insert(id.clone());
    for dep in &plugin.dependencies {
        if let Some(p) = all.iter().find(|p| p.id == dep.plugin_id) {
            if detect_cycle(p, all, visited, stack) {
                return true;
            }
        }
    }
    stack.remove(&id);
    false
}
