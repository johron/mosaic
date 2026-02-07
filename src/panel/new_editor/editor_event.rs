use crate::handler::panel_handler::Panel;

pub fn handle_editor_event(panel: &mut Panel, event: &str, args: Vec<String>) -> Result<(), String> {
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