use std::collections::HashMap;
use crate::app::MosId;
use crate::panel::panel::{Panel, PanelCtor};

pub struct PanelRegistry {
    panels: HashMap<MosId, (MosId, PanelCtor)>, // (plugin_id, panel_id, panel_ctor)
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
        }
    }

    pub fn register_panel_kind(&mut self, plugin_id: MosId, panel_id: MosId, panel_ctor: PanelCtor) {
        self.panels.insert(panel_id, (plugin_id, panel_ctor));
    }
    
    pub fn unregister_panels_by_plugin(&mut self, plugin_id: &MosId) {
        self.panels.retain(|_, (p_id, _)| p_id != plugin_id);
    }
    
    pub fn get_panels(&self) -> &HashMap<MosId, (MosId, PanelCtor)> {
        &self.panels
    }
    
    pub fn get_panels_by_plugin(&self, plugin_id: &MosId) -> Vec<MosId> {
        self.panels.iter()
            .filter(|(_, (p_id, _))| p_id == plugin_id)
            .map(|(panel_id, _)| *panel_id)
            .collect()
    } // need some way of instantiating the panel from id, maybe in plugin registry

    pub fn new_panel_instance(&self, panel_id: &MosId) -> Option<Box<dyn Panel>> {
        self.panels.get(panel_id).map(|(_, panel_ctor)| panel_ctor())
    }
}