use crate::app::MosId;
use crate::event::event::Event;
use crate::panel::panel::PanelCtor;
use crate::system::panel_registry::PanelRegistry;
use std::collections::HashMap;

pub struct PluginRegistration {
    pub panel_kinds: HashMap<MosId, PanelCtor>,
    // may contain more things
}

pub trait Plugin {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn description(&self) -> String;
    // fn dependencies(&self) -> Vec<PrettyMosKind>;

    // fn is_backend()
    // also/or have some kind of function subscription, so that they only get what the care about

    fn enable(&mut self, panel_registry: &mut PanelRegistry) -> Result<(), String>;
    fn disable(&mut self) -> PluginRegistration; // return the panels to be removed and unregistered, could probably just remove all panels with this plugin's id from hashmap
    fn handle_event(&mut self, event: Event) -> Result<(), String>;
}
