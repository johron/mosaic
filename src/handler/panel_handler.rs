use ratatui::Frame;
use ratatui::layout::Rect;
use uuid::Uuid;
use crate::handler::config_handler::AppConfig;
use crate::Mode;
use crate::panel::editor::editor_logic::Cursor;
use crate::panel::editor::editor_syntax::{SyntaxConfig, SyntaxIndexConfig};

#[derive(Debug, Clone, PartialEq)]
pub enum PanelKind {
    Editor,
    Command,
    Split(Box<Panel>), // panel that can be split into multiple panels
    Plugin(String, String), // plugin name, panel name
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelData {
    Editor {
        rope: ropey::Rope,
        top_line: usize,
        cursors: Vec<Cursor>,
    },
    Command {
        content: String,
        result: Option<String>,
        history: Vec<String>,
        history_index: Option<usize>,
    },
    Split {
        panels: Vec<String>, // IDs of child panels
    },
    Plugin {
        data: std::collections::HashMap<String, String>,
    },
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub kind: PanelKind,
    pub title: String,
    pub active: bool,
    pub data: PanelData,

    id: String,
    config: AppConfig,
    mode: Mode,

    pub draw: fn(&mut Panel, frame: &mut Frame, area: Rect),
    pub event: fn(&mut Panel, event: &str, args: Vec<String>) -> Result<(), String>,
}

impl Panel {
    pub fn new(
        kind: PanelKind,
        title: String,
        data: PanelData,
        config: AppConfig,
        mode: Mode,
        draw: fn(&mut Panel, frame: &mut Frame, area: Rect),
        event: fn(&mut Panel, event: &str, args: Vec<String>) -> Result<(), String>,
    ) -> Self {
        Self {
            kind,
            title,
            active: false,
            data,
            id: Uuid::new_v4().to_string(),
            config,
            mode,
            draw,
            event: event,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_config(&mut self, config: AppConfig) {
        self.config = config;
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct PanelHandler {
    pub panels: Vec<Panel>,
    pub active_panel: Option<String>,
}

impl PanelHandler {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            active_panel: None,
        }
    }

    pub fn draw_panels(&mut self, frame: &mut Frame, area: Rect) {
        for panel in self.panels.iter_mut() {
            (panel.draw)(panel, frame, area);
        }
    }

    pub fn add_panel(&mut self, panel: Panel) -> Result<(), String> {
        for p in self.panels.iter() {
            if p.id == panel.id {
                return Err(format!("Panel with id {} already exists", panel.id));
            }
        }

        self.panels.push(panel);
        Ok(())
    }

    pub fn remove_panel(&mut self, id: &str) -> Result<(), String> {
        if let Some(pos) = self.panels.iter().position(|p| p.id == id) {
            self.panels.remove(pos);
            Ok(())
        } else {
            Err(format!("Panel with id {} not found", id))
        }
    }

    pub fn set_active_panel(&mut self, id: Option<String>) {
        if let Some(id) = &id {
            if self.panels.iter().any(|p| p.id == *id) {
                self.active_panel = Some(id.clone());
            } else {
                self.active_panel = None;
            }
        } else {
            self.active_panel = None;
        }
    }

    pub fn get_active_panel(&self) -> Option<&Panel> {
        if let Some(id) = &self.active_panel {
            self.panels.iter().find(|p| p.id == *id)
        } else {
            None
        }
    }

    pub fn get_active_panel_mut(&mut self) -> Option<&mut Panel> {
        if let Some(id) = &self.active_panel {
            self.panels.iter_mut().find(|p| p.id == *id)
        } else {
            None
        }
    }
}