use crate::app::MosId;
use crate::plugin::plugin::{Plugin, PluginRegistration};
use crate::plugin_builtin::mos_editor::editor_panel::EditorPanel;
use crate::system::panel_registry::PanelRegistry;

pub struct MosEditorPlugin {
    pub id: MosId,
}

impl MosEditorPlugin {
    pub fn new() -> Self {
        Self {
            id: MosId::new(),
        }
    }
}

impl Plugin for MosEditorPlugin {
    fn id(&self) -> MosId {
        self.id
    }

    fn name(&self) -> &str {
        "MosEditor"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "The built-in text editor plugin for Mos"
    }

    fn enable(&mut self, panel_registry: &mut PanelRegistry) -> Result<(), String> {
        println!("(built-in) [{}] Enabled with plugin-id {:?}", self.name(), self.id());

        panel_registry.register_panel_kind(self.id(), MosId::new(), || Box::new(EditorPanel::new()));

        Ok(())
    }

    fn disable(&mut self) -> PluginRegistration {
        todo!()
    }

    fn handle_event(&mut self, _event: crate::event::event::Event) -> Result<(), String> {
        println!("Text Editor Plugin received event: {:?}", _event);
        Ok(())
    }

}