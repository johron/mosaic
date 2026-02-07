use ropey::Rope;
use crate::handler::config_handler::AppConfig;
use crate::handler::panel_handler::{Panel, PanelData, PanelKind};
use crate::Mode;
use crate::panel::editor::editor_logic::Cursor;
use crate::panel::new_editor::editor_draw::draw_editor_panel;

pub struct Editor {
    pub rope: Rope,
    pub scroll_offset: usize,
    pub cursors: Vec<Cursor>, // for multiple cursors
}

pub fn new_editor_panel(config: AppConfig) -> Panel {
    Panel::new(
        PanelKind::Editor,
        String::from("unnamed"),
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

fn handle_editor_event(panel: &mut Panel, event: &str, args: Vec<String>) -> Result<(), String> {
    match event {
        "input" => {
            // Handle key press events for the editor
            println!("Editor received input: {:?}", args);
            Ok(())
        },
        "move_cursor"  => {
            Ok(())
        }
        _ => Err(format!("Unhandled event: {}", event)),
    }
}