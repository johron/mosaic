use crate::plugin::plugin::Plugin;
use crate::system::panel_registry::PanelRegistry;

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
    panel_registry: PanelRegistry,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            panel_registry: PanelRegistry::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
    
    pub fn enable_plugins(&mut self) {
        for plugin in &mut self.plugins {
            let registration = plugin.enable();
            for panel in registration.panels {
                self.panel_registry.register_panel(panel);
            }
        }
    }
}