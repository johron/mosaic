use crate::{Command, Mode, Mosaic};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;

use std::time::{Duration};

#[derive(Clone, Debug)]
pub struct InputHandler {}
impl InputHandler {
    pub(crate) fn new() -> InputHandler {
        Self {}
    }

    pub(crate) fn handle(mosaic: &mut Mosaic) -> Result<(), Error> {
        if event::poll(Duration::from_millis(10))? {
            let key_events = Self::collect_simultaneous_key_events(30)?;
            if !key_events.is_empty() {
                mosaic.toast = None;

                Self::process_key_events(mosaic, key_events).expect("TODO: panic message");
            }
            //if let Event::Mouse(mouse_event) = event::read()? {
            //    // process mouse event
            //}
        }

        Ok(())
    }

    fn collect_simultaneous_key_events(window_ms: u64) -> Result<Vec<KeyEvent>, Error> {
        let start = std::time::Instant::now();
        let mut events: Vec<KeyEvent> = vec![];

        if event::poll(Duration::from_millis(window_ms))? {
            if let Event::Key(k) = event::read()? {
                events.push(k);
            }
        } else {
            return Ok(events);
        }

        while start.elapsed() < Duration::from_millis(window_ms) {
            if event::poll(Duration::from_millis(1))? {
                if let Event::Key(k) = event::read()? {
                    events.push(k);
                } else {
                    let _ = event::read()?;
                }
            } else {
                break;
            }
        }

        Ok(events)
    }

    fn process_key_events(mosaic: &mut Mosaic, keys: Vec<KeyEvent>) -> Result<String, String> {
        // In Insert mode, handle regular character input directly without shortcut matching
        if mosaic.state_handler.mode == Mode::Insert {
            if let Some(first) = keys.first() {
                // Check if this is a regular character without modifiers (except shift)
                if let KeyCode::Char(_) = first.code {
                    if first.modifiers.is_empty() || first.modifiers.contains(KeyModifiers::SHIFT) {
                        // This is regular text input, handle it directly
                        return Self::handle_input_mode(mosaic, *first);
                    }
                }
            }
        }

        let mut pressed: Vec<String> = vec![];

        for key in keys.iter() {
            let modifier = key.modifiers.to_string();
            let char = key.code.to_string();

            if !char.is_empty() {
                let new_char = match char.clone().replace(" ", "").to_lowercase().as_str() {
                    "backtab" => "tab",
                    _ => char.as_str(),
                };
                pressed.push(new_char.to_lowercase());
            }

            if !modifier.is_empty() {
                let mods = modifier.split('+');
                for modi in mods {
                    pressed.push(modi.to_lowercase().replace(' ', ""));
                }
            }
        }

        pressed.sort();
        pressed.dedup();

        for shortcut in mosaic.shortcut_handler.get_shortcuts() {
            let mode = format!("mode.{}", mosaic.state_handler.mode.clone().to_string().to_lowercase());

            if shortcut.name.starts_with(mode.as_str()) || !shortcut.name.starts_with("mode.") {
                let mut input: Vec<String> = shortcut.input.split('|').map(String::from).collect();
                input.sort();

                for s in input {
                    let mut split: Vec<String> = s.replace(' ', "").split('+').map(String::from).collect();
                    split.sort();

                    if split == pressed {
                        return (shortcut.handler)(mosaic);
                    }
                }
            }
        }

        if let Some(first) = keys.first() {
            match mosaic.state_handler.mode {
                Mode::Insert => Self::handle_input_mode(mosaic, *first),
                Mode::Command => Self::handle_command_mode(mosaic, *first),
                _ => Ok(String::from("Input is unmapped")),
            }
        } else {
            Ok(String::from("No input"))
        }
    }

    fn handle_input_mode(mosaic: &mut Mosaic, key_event: KeyEvent) -> Result<String, String> {
        if mosaic.panel_handler.get_current_editor_panel().is_none() {
            return Err(String::from("No active editor"))
        }

        let editor = &mut mosaic.panel_handler.get_current_editor_panel().unwrap().editor;

        match key_event.code {
            KeyCode::Char(c) => {
                if key_event.modifiers.is_empty() || key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    editor.input(c)
                }
            },
            _ => {
                return Ok(String::from("Unmapped input"));
            }
        }

        Ok(String::from("Inputted"))
    }

    fn handle_command_mode(mosaic: &mut Mosaic, key: KeyEvent) -> Result<String, String> {
        match key.code {
            KeyCode::Enter => {
                let res = Self::handle_command(mosaic);

                mosaic.state_handler.command = Command {
                    content: String::new(),
                    result: Some(res.unwrap_or_else(|e| format!("Error: {}", e))),
                };

                mosaic.state_handler.mode = Mode::Normal;
            },
            KeyCode::Char(c) => {
                mosaic.state_handler.command += c.to_string().as_str();
            },
            KeyCode::Backspace => {
                mosaic.state_handler.command.pop();
            },
            _ => {
                return Ok(String::from("Unmapped input"));
            }
        }

        Ok(String::from("Inputted command"))
    }

    fn handle_command(mosaic: &mut Mosaic) -> Result<String, String> {
        let args = mosaic.state_handler.command.content.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

        let commands = mosaic.command_handler.get_commands("@");

        if args.is_empty() || args[0].is_empty() {
            return Err(String::from("No command provided"));
        }

        if let Some(cmds) = commands {
            if let Some(command) = cmds.iter().find(|cmd| cmd.name == args[0]) {
                (command.handler)(mosaic, args)
            } else {
                Err(format!("Unknown command: {}", args[0]))
            }
        } else {
            Err(String::from("No command namespace found"))
        }
    }
}