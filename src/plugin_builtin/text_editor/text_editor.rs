use crate::plugin::plugin::{Plugin, PluginRegistration};
use crate::plugin_builtin::text_editor::editor_panel::EditorPanel;

pub struct TextEditorPlugin {}

impl TextEditorPlugin {
    pub fn new() -> Self {
        Self {}
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
        PluginRegistration {
            panels: vec![EditorPanel::new()],
        }
    }

    fn disable(&mut self) -> PluginRegistration {
        todo!()
    }

    fn handle_event(&mut self, _event: crate::event::event::Event) -> Result<(), String> {
        Ok(())
    }
}