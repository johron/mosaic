use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use crate::handler::config_handler::AppConfig;
use crate::handler::panel_handler::{Panel, PanelKind};
use crate::Mode;

pub fn new_editor_panel(config: AppConfig) -> Panel {
    Panel::new(
        PanelKind::Editor,
        "unnamed".to_string(),
        config,
        Mode::Normal,
        draw_editor_panel,
        handle_editor_event,
    )
}

fn draw_editor_panel(panel: &mut Panel, frame: &mut Frame, area: Rect) {
    // draw some simple placeholder text
    let paragraph = Paragraph::new("Editor Panel - Content goes here");
    frame.render_widget(paragraph, area);
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