use std::io::Error;
use crate::Mosaic;

pub(crate) fn handle_command(mosaic: &mut Mosaic) -> Result<String, Error> {
    match mosaic.command.content.as_str() {
        "q" => {
            mosaic.quit();
            Ok(String::from("Quit command executed"))
        },
        "s" => {
            // Placeholder for save functionality
            Ok(String::from("Save command executed"))
        }
        _ => {
            Ok(String::from("Error: Unknown command"))
        }
    }
}