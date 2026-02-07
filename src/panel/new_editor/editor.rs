use ropey::Rope;
use crate::handler::config_handler::AppConfig;
use crate::handler::panel_handler::{Panel, PanelData, PanelKind};
use crate::Mode;
use crate::panel::new_editor::editor_draw::draw_editor_panel;
use crate::panel::new_editor::editor_event::handle_editor_event;

pub struct Editor {
    pub rope: Rope,
    pub scroll_offset: usize,
    pub cursors: Vec<Cursor>,
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
        },
        config,
        Mode::Normal,
        draw_editor_panel,
        handle_editor_event,
    )
}