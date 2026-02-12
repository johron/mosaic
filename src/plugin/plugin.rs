use std::collections::HashMap;
use crate::app::MosId;
use crate::event::event::Event;
use crate::panel::panel::{Panel, PanelCtor};

pub struct PluginRegistration {
    pub panel_kinds: HashMap<MosId, PanelCtor>,
    // may contain more things
}

pub trait Plugin {
    fn id(&self) -> MosId;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;

    fn enable(&mut self) -> PluginRegistration;
    fn disable(&mut self) -> PluginRegistration; // return the panels to be removed and unregistered, could probably just remove all panels with this plugin's id from hashmap
    fn handle_event(&mut self, event: Event) -> Result<(), String>;
}
