use crate::panel::panel::Panel;
use crate::plugin::plugin::{Plugin, PluginRegistration};
use crate::plugin_builtin::mos_editor::editor_panel::EditorPanel;
use crate::system::panel_registry::PanelRegistry;

pub struct MosEditorPlugin {}

impl MosEditorPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for MosEditorPlugin {
    fn id(&self) -> String {
        String::from("mos_builtin_editor")
    }

    fn name(&self) -> String {
        String::from("MosEditor")
    }

    fn version(&self) -> String {
        String::from("0.1.0")
    }

    fn description(&self) -> String {
        String::from("The built-in text editor plugin for Mos")
    }

    fn enable(&mut self, panel_registry: &mut PanelRegistry) -> Result<(), String> {
        //println!("(built-in) [{}] Enabled with plugin-id {:?}", self.name(), self.id()); -> go to log instead of screen

        panel_registry.register_panel_kind(self.id(), String::from("editor_panel"), || Box::new(EditorPanel::new()));

        Ok(())
    }

    fn disable(&mut self) -> PluginRegistration {
        todo!()
    }

    fn handle_event(&mut self, _event: crate::event::event::Event) -> Result<(), String> {
        //println!("Text Editor Plugin received event: {:?}", _event);
        Ok(())
    }

}