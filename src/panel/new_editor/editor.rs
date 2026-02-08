use ropey::Rope;
use crate::handler::config_handler::{AppConfig};
use crate::handler::panel_handler::{Panel, PanelData, PanelKind};
use crate::panel::new_editor::editor_draw::draw_editor_panel;
use crate::panel::new_editor::editor_event::handle_editor_event;

pub struct EditorData {
    pub rope: Rope,
    pub scroll_offset: usize,
    pub cursors: Vec<Cursor>,
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    // Terminal
    // Select
    // Search & Replace
    // Explorer/Files
}

impl Mode {
    pub fn from_string(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" => Some(Mode::Normal),
            "insert" => Some(Mode::Insert),
            "command" => Some(Mode::Command),
            _ => None,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Mode::Normal => String::from("Normal"),
            Mode::Insert => String::from("Insert"),
            Mode::Command => String::from("Command"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
    pub goal_col: usize,
}

pub fn new_editor_panel(config: AppConfig) -> Panel {
    Panel::new(
        PanelKind::Editor,
        String::from("unnamed"), // TODO: set title to file name
        PanelData::Editor {
            rope: Rope::new(),
            top_line: 0,
            cursors: vec![Cursor { line: 0, col: 0, goal_col: 0 }],
            mode: Mode::Normal,
        },
        config,
        draw_editor_panel,
        handle_editor_event,
    )
}