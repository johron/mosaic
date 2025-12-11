use std::collections::HashMap;
use crate::panel::editor_panel::EditorPanel;

#[derive(Debug, Clone)]
pub struct Panel {
    pub id: String,
    pub child: PanelChild,
    pub position: (u16, u16),
    pub size: (u16, u16),
}

impl Panel {
    pub fn new(id: String, child: PanelChild) -> Self {
        Self {
            id,
            child,
            position: (0, 0),
            size: (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PanelChild {
    Editor(EditorPanel),
    Explorer(/*ExplorerPanel*/),
}

#[derive(Debug, Clone)]
pub struct PanelHandler {
    pub(crate) panels: Vec<Panel>,
    current_panel_id: Option<String>,
}

impl PanelHandler {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            current_panel_id: None,
        }
    }
    
    pub fn add_panel(&mut self, panel: Panel) {
        self.panels.push(panel);
    }
    
   pub fn get_current_panel(&mut self) -> Option<&mut Panel> {
       if let Some(id) = self.current_panel_id.clone() {
           self.get_panel(&id)
       } else {
           None
       }
   }
    
    pub fn get_current_editor_panel(&mut self) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_current_panel() {
            if let PanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
    }
    pub fn get_panel(&mut self, id: &str) -> Option<&mut Panel> {
        self.panels.iter_mut().find(|panel| panel.id == id)
    }
    
    pub fn get_editor_panel(&mut self, id: &str) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_panel(id) {
            if let PanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
    }
}