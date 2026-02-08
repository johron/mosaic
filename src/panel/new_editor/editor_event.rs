use crate::handler::panel_handler::{Panel, PanelData};
use crate::panel::new_editor::editor::{Mode};

pub fn handle_editor_event(panel: &mut Panel, event: &str, args: Vec<String>) -> Result<String, String> {
    match event {
        "input" => {
            // Handle key press events for the editor
            println!("Editor received input: {:?}", args);
            Ok(String::from("Input handled"))
        },
        "move_cursor"  => {
            Ok(String::from("Move cursor handled"))
        },
        "set_mode" => {
            if let Some(mode) = args.first() {
                let mode = Mode::from_string(mode.to_owned());
                match &mut panel.data {
                    PanelData::Editor { rope, top_line, cursors, mode: panel_mode } => {
                        if let Some(mode) = mode {
                            *panel_mode = mode;
                            Ok(format!("Mode set to {:?}", mode))
                        } else {
                            Err(format!("Invalid mode: {:?}", mode))
                        }
                    },
                    _ => Err(String::from("Panel data is not of type Editor")),
                }
            } else {
                Err(String::from("No mode specified"))
            }
        },
        "get_mode" => {
            match &panel.data {
                PanelData::Editor { rope, top_line, cursors, mode } => {
                    Ok(mode.to_string())
                },
                _ => Err(String::from("Panel data is not of type Editor")),
            }
        },
        _ => Err(format!("Unhandled event: {}", event)),
    }
}