use crate::app::MosId;
use crate::panel::panel::{Panel, PanelCtor};
use std::collections::HashMap;

pub struct PanelRegistry {
    panels: HashMap<String, (String, PanelCtor)>, // (plugin_id, panel_id, panel_ctor)
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
        }
    }

    pub fn register_panel_kind(&mut self, plugin_id: String, panel_id: String, panel_ctor: PanelCtor) {
        self.panels.insert(panel_id.to_string(), (plugin_id.to_string(), panel_ctor));
    }
    
    pub fn unregister_panels_by_plugin(&mut self, plugin_id: String) {
        self.panels.retain(|_, (p_id, _)| *p_id != plugin_id);
    }
    
    pub fn get_panels(&self) -> &HashMap<String, (String, PanelCtor)> {
        &self.panels
    }
    
    pub fn get_panel(&self, panel_id: &str) -> Option<&(String, PanelCtor)> {
        self.panels.get(panel_id)
    }
    
    pub fn get_panels_by_plugin(&self, plugin_id: &str) -> Vec<String> {
        self.panels.iter()
            .filter(|(_, (p_id, _))| p_id == plugin_id)
            .map(|(panel_id, _)| panel_id.to_string())
            .collect()
    } // need some way of instantiating the panel from id, maybe in plugin registry

    pub fn new_panel_instance(&self, panel_id: String) -> Option<Box<dyn Panel>> {
        self.panels.get(&panel_id).map(|(_, panel_ctor)| panel_ctor())
    }
}