use crate::app::MosId;
use crate::panel::panel::PanelCtor;
use crate::plugin::plugin::{Plugin, PluginRegistration};
use crate::plugin_builtin::text_editor::editor_panel::EditorPanel;

pub struct TextEditorPlugin {
    pub panel_kinds: Vec<MosId>,
}

impl TextEditorPlugin {
    pub fn new() -> Self {
    }
}

impl Plugin for TextEditorPlugin {
    fn name(&self) -> &str {
        "Text Editor"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "The built-in text editor plugin for Mos"
    }

    fn enable(&mut self) -> PluginRegistration {
        let panel_id = MosId::new();
        self.panel_kinds.push(panel_id.clone());
        PluginRegistration {
            panel_kinds: vec![(panel_id, PanelCtor::try_from(EditorPanel)],
        }
    }

    fn disable(&mut self) -> PluginRegistration {
        todo!()
    }

    fn handle_event(&mut self, _event: crate::event::event::Event) -> Result<(), String> {
        Ok(())
    }

}