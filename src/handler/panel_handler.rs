use std::collections::HashMap;

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
    Editor(/*EditorPanel*/),
    Explorer(/*ExplorerPanel*/),
}

#[derive(Debug, Clone)]
pub struct PanelHandler {
    panels: Vec<Panel>,
}

impl PanelHandler {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
        }
    }
}