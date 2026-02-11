use crate::event::event::Event;
use crate::panel::panel::Panel;

pub struct PluginRegistration {
    pub panels: Vec<Box<dyn Panel>>,
    // may contain more things
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;

    fn enable(&mut self) -> PluginRegistration;
    fn disable(&mut self) -> PluginRegistration; // return the panels to be removed and unregistered
    fn handle_event(&mut self, event: Event) -> Result<(), String>;
    
}
