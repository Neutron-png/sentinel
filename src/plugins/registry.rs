use crate::plugins::api::{Plugin, PluginMetadata};

#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

#[allow(dead_code)]
impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Plugin>> {
        self.plugins.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Plugin>> {
        self.plugins.iter_mut()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins
            .iter()
            .find(|p| p.metadata().id == id)
            .map(|v| v.as_ref())
    }

    pub fn metadata_list(&self) -> Vec<&PluginMetadata> {
        self.plugins.iter().map(|p| p.metadata()).collect()
    }

    pub fn unload_all(&mut self) -> Vec<Box<dyn Plugin>> {
        std::mem::take(&mut self.plugins)
    }
}
