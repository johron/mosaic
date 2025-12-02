use std::io::Error;
use crate::Mosaic;

pub(crate) fn handle_command(mosaic: &mut Mosaic) -> Result<String, Error> {
    let args = mosaic.command.content.as_str().split(' ').collect::<Vec<_>>();

    match args[0] {
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