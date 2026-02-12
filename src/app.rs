use ratatui::Frame;
use uuid::Uuid;
use crate::event::event::Event;
use crate::plugin_builtin::mos_editor::mos_editor::MosEditorPlugin;
use crate::system::panel_registry::PanelRegistry;
use crate::system::plugin_registry::PluginRegistry;
use crate::workspace::workspace::Workspace;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct MosId(Uuid);

impl MosId {
    pub fn new() -> Self {
        MosId(Uuid::new_v4())
    }
}

pub struct Mos {
    pub should_quit: bool,
    pub active_workspace: usize,
    pub workspaces: Vec<Workspace>,
    pub panel_registry: PanelRegistry,
    pub plugin_registry: PluginRegistry,
}

impl Mos {
    pub fn new() -> Self {
        let mut plugin_registry = PluginRegistry::new();
        let mut panel_registry = PanelRegistry::new();

        // Register built-in plugins here, **temporary code**
        plugin_registry.register_plugin(Box::new(MosEditorPlugin::new()));

        plugin_registry.enable_plugins(&mut panel_registry);

        // more temporary code for demo editor panel
        //let text_editor_kind_id = panel_registry.get_panels_by_plugin(&plugin_registry.get_plugins()[0].id()).first().cloned();
        //if let Some(kind_id) = text_editor_kind_id {
        //    println!("Registered Text Editor Plugin with kind id: {:?}", kind_id);
//
        //    let panel_instance = panel_registry.new_panel_instance(&kind_id);
        //    if let Some(panel) = panel_instance {
        //        panel.ren
        //        println!("Successfully created an instance of the Text Editor panel");
        //    } else {
        //        eprintln!("Failed to create an instance of the Text Editor panel");
        //    }
        //} else {
        //    eprintln!("Failed to register Text Editor Plugin");
        //}

        Mos {
            should_quit: false,
            active_workspace: 0,
            workspaces: vec![Workspace::new()],
            panel_registry,
            plugin_registry,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_terminal_event(&mut self, event: crossterm::event::Event) {
        // Only handle key events for global and the current active panel.

        let mos_event = Event::from_crossterm_event(event);
        if let Some(ev) = mos_event {
            self.plugin_registry.handle_plugins_events(ev);
        }
    }

    pub fn render(&mut self, _frame: &mut Frame) {
        // Render the current workspace and its panels.
        let workspace = &mut self.workspaces[self.active_workspace];
        
    }
}