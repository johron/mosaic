use std::cmp::PartialEq;
use crate::event::event::Event;
use crate::plugin_builtin::mos_editor::mos_editor::MosEditorPlugin;
use crate::system::panel_registry::PanelRegistry;
use crate::system::plugin_registry::PluginRegistry;
use crate::workspace::workspace::Workspace;
use ratatui::Frame;
use uuid::Uuid;
use crate::system::lua_manager::LuaManager;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct MosId(Uuid);


#[derive(PartialEq)]
pub enum MosState {
    Panel, // Events can go to active panel, state is now handled by the active panel, e.g. a text editor has their own modes normal, insert..
    Floating, // Events only go to active floating panel
}

impl MosId {
    pub fn new() -> Self {
        MosId(Uuid::new_v4())
    }
}

pub struct Mos {
    pub state: MosState,
    pub should_quit: bool,
    pub active_workspace: usize,
    pub workspaces: Vec<Workspace>,
    pub panel_registry: PanelRegistry,
    pub plugin_registry: PluginRegistry,
    pub lua_manager: LuaManager,
}

impl Mos {
    pub fn new() -> Self {
        let mut plugin_registry = PluginRegistry::new();
        let mut panel_registry = PanelRegistry::new();
        let mut lua_manager = LuaManager::new();

        lua_manager.init().unwrap();
        lua_manager.load_plugin("test.lua").unwrap();

        let mut workspace = Workspace::new();

        // Register built-in plugins here, **temporary code**
        plugin_registry.register_plugin(Box::new(MosEditorPlugin::new()));

        plugin_registry.enable_plugins(&mut panel_registry);

        let text_editor_kind_id = panel_registry.get_panels_by_plugin(&plugin_registry.get_plugins()[0].id()).first().cloned();
        if let Some(kind_id) = text_editor_kind_id {
            let panel_instance = panel_registry.new_panel_instance(kind_id);
            if let Some(panel) = panel_instance {
                workspace.add_panel(panel);
            } else {
                eprintln!("Failed to create an instance of the Text Editor panel");
            }
        } else {
            eprintln!("Failed to register Text Editor Plugin");
        }

        Mos {
            state: MosState::Panel,
            should_quit: false,
            active_workspace: 0,
            workspaces: vec![workspace],
            panel_registry,
            plugin_registry,
            lua_manager
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_terminal_event(&mut self, event: crossterm::event::Event) {
        // Only handle key events for global and the current active panel.

        let mos_event = Event::from_crossterm_event(event);

        if let Some(ev) = mos_event {
            self.plugin_registry.handle_plugins_events(ev.clone());

            
            match self.state {
                MosState::Panel => {
                    let active_panel = self.workspaces[self.active_workspace].get_active_panel_mut();
                    if let Some(panel) = active_panel {
                        panel.handle_event(ev)
                    }
                },
                MosState::Floating => {
                    if let Some(floating) = self.workspaces[self.active_workspace].get_floating() {
                        floating.panel.handle_event(ev);
                    }
                }
            }
        }
    }

    pub fn render(&mut self, _frame: &mut Frame) {
        // Render the current workspace and its panels.
        let workspace = &mut self.workspaces[self.active_workspace];
        workspace.render(_frame);
    }
}