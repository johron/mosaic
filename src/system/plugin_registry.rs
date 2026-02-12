use std::collections::HashMap;
use crate::app::MosId;
use crate::plugin::plugin::Plugin;
use crate::system::panel_registry::PanelRegistry;

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
    
    pub fn enable_plugins(&mut self, panel_registry: &mut PanelRegistry) {
        for plugin in &mut self.plugins {
            let registration = plugin.enable();
            for panel in registration.panel_kinds {
                panel_registry.register_panel(plugin.id(), panel.0, panel.1);
            }
        }
    }
}