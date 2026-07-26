#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Unloaded,
    Loaded,
    Initialized,
    Enabled,
    Disabled,
    Failed,
}

impl PluginState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unloaded => "Unloaded",
            Self::Loaded => "Loaded",
            Self::Initialized => "Initialized",
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginLifecycleManager {
    state: PluginState,
}

impl PluginLifecycleManager {
    pub fn new() -> Self {
        Self {
            state: PluginState::Unloaded,
        }
    }

    pub fn state(&self) -> PluginState {
        self.state
    }
    pub fn load(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Loaded;
        Ok(())
    }
    pub fn initialize(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Initialized;
        Ok(())
    }
    pub fn enable(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Enabled;
        Ok(())
    }
    pub fn disable(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Disabled;
        Ok(())
    }
    pub fn reload(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Loaded;
        self.initialize()?;
        self.enable()
    }
    pub fn unload(&mut self) -> Result<(), super::errors::SdkError> {
        self.state = PluginState::Unloaded;
        Ok(())
    }
}
