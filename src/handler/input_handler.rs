use crate::{Command, Mode, Mos};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;
use std::time::{Duration, Instant};
use crate::handler::panel_handler::PanelKind;

#[derive(Clone, Debug)]
pub struct InputHandler {
    mos_key_as_mod_pressed: bool,
    mos_key_as_mod_timestamp: Option<Instant>,
}

impl InputHandler {
    pub(crate) fn new() -> InputHandler {
        Self {
            mos_key_as_mod_pressed: false,
            mos_key_as_mod_timestamp: None,
        }
    }

    pub(crate) fn handle(&mut self, mos: &mut Mos) -> Result<(), Error> {
        if event::poll(Duration::from_millis(10))? {
            let key_events = Self::collect_simultaneous_key_events(30)?;
            if !key_events.is_empty() {
                mos.toast = None;

                self.process_key_events(mos, key_events).expect("TODO: panic message");
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

    fn process_key_events(&mut self, mos: &mut Mos, keys: Vec<KeyEvent>) -> Result<String, String> {
        let config = mos.config_handler.config.clone();
        let mos_key = config.mos.shortcuts.mos_key.clone();
        let mos_key_as_mod = config.mos.shortcuts.mos_key_as_mod.clone();

        // Check if mos_key is pressed (single key press to normal mode)
        if keys.len() == 1 {
            let key = keys[0];
            let key_str = key.code.to_string().replace(" ", "").to_lowercase();
            let modifier_str = key.modifiers.to_string().to_lowercase();

            let full_key = if modifier_str.is_empty() {
                key_str.clone()
            } else {
                format!("{}+{}", modifier_str.replace(" ", ""), key_str)
            };

            if full_key == mos_key.to_lowercase() {
                mos.state_handler.mode = Mode::Normal;
                return Ok(String::from("Switched to Normal mode"));
            }

            // Check if mos_key_as_mod is pressed
            if full_key == mos_key_as_mod.to_lowercase() {
                self.mos_key_as_mod_pressed = true;
                self.mos_key_as_mod_timestamp = Some(Instant::now());
                return Ok(String::from("mos_key_as_mod activated"));
            }
        }

        // If mos_key_as_mod was pressed, combine it with current keys
        let mut pressed: Vec<String> = vec![];

        if self.mos_key_as_mod_pressed {
            // Add mos_key_as_mod as a prefix to all current keys
            let mod_parts: Vec<String> = mos_key_as_mod.split('+').map(|s| s.to_lowercase()).collect();
            pressed.extend(mod_parts);
            self.mos_key_as_mod_pressed = false;
            self.mos_key_as_mod_timestamp = None;
        }

        // In Insert mode, handle regular character input directly without shortcut matching
        if mos.state_handler.mode == Mode::Insert {
            if let Some(first) = keys.first() {
                // Check if this is a regular character without modifiers (except shift)
                if let KeyCode::Char(_) = first.code {
                    if first.modifiers.is_empty() || first.modifiers.contains(KeyModifiers::SHIFT) {
                        // This is regular text input, handle it directly
                        return self.handle_input_mode(mos, *first);
                    }
                }
            }
        }

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

        for shortcut in mos.shortcut_handler.get_shortcuts() {
            let mode = format!("mode.{}", mos.state_handler.mode.clone().to_string().to_lowercase());

            if shortcut.name.starts_with(mode.as_str()) || !shortcut.name.starts_with("mode.") {
                let mut input: Vec<String> = shortcut.input.split('|').map(String::from).collect();
                input.sort();

                for s in input {
                    let mut split: Vec<String> = s.replace(' ', "").split('+').map(String::from).collect();
                    split.sort();

                    if split == pressed {
                        return (shortcut.handler)(mos);
                    }
                }
            }
        }

        if let Some(first) = keys.first() {
            match mos.state_handler.mode {
                Mode::Insert => self.handle_input_mode(mos, *first),
                Mode::Command => Self::handle_command_mode(mos, *first),
                _ => Ok(String::from("Input is unmapped")),
            }
        } else {
            Ok(String::from("No input"))
        }
    }

    //fn handle_input_mode(&mut self, mos: &mut Mos, key_event: KeyEvent) -> Result<String, String> {
    //    if mos.panel_handler.get_current_editor_panel().is_none() {
    //        return Err(String::from("No active editor"))
    //    }
//
    //    let editor = &mut mos.panel_handler.get_current_editor_panel().unwrap().editor;
//
    //    match key_event.code {
    //        KeyCode::Char(c) => {
    //            if key_event.modifiers.is_empty() || key_event.modifiers.contains(KeyModifiers::SHIFT) {
    //                editor.input(c)
    //            }
    //        },
    //        _ => {
    //            return Ok(String::from("Unmapped input"));
    //        }
    //    }
//
    //    Ok(String::from("Inputted"))
    //}

    fn handle_input_mode(&mut self, mos: &mut Mos, key_event: KeyEvent) -> Result<String, String> {
        let panel = mos.panel_handler.get_active_panel_mut();
        if panel.is_none() {
            return Err(String::from("No active panel"));
        }

        let panel = panel.unwrap();
        if panel.kind != PanelKind::Editor {
            return Err(String::from("Active panel is not an editor"));
        }

        match key_event.code {
            KeyCode::Char(c) => {
                if key_event.modifiers.is_empty() || key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    // Call the editor's input event with the character
                    let res = (panel.event)(panel, "input", vec![c.to_string()]);
                    res.map(|_| String::from("Inputted character"))
                } else {
                    Ok(String::from("Modifier keys are not supported in Insert mode"))
                }
            },
            _ => {
                Ok(String::from("Unmapped input"))
            }
        }


    }

    fn handle_command_mode(mos: &mut Mos, key: KeyEvent) -> Result<String, String> {
        match key.code {
            KeyCode::Enter => {
                let res = Self::handle_command(mos);

                mos.state_handler.command = Command {
                    content: String::new(),
                    result: Some(res.unwrap_or_else(|e| format!("Error: {}", e))),
                };

                mos.state_handler.mode = Mode::Normal;
            },
            KeyCode::Char(c) => {
                mos.state_handler.command += c.to_string().as_str();
            },
            KeyCode::Backspace => {
                mos.state_handler.command.pop();
            },
            _ => {
                return Ok(String::from("Unmapped input"));
            }
        }

        Ok(String::from("Inputted command"))
    }

    fn handle_command(mos: &mut Mos) -> Result<String, String> {
        let args = mos.state_handler.command.content.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

        let commands = mos.command_handler.get_commands("@");

        if args.is_empty() || args[0].is_empty() {
            return Err(String::from("No command provided"));
        }

        if let Some(cmds) = commands {
            if let Some(command) = cmds.iter().find(|cmd| cmd.name == args[0]) {
                (command.handler)(mos, args)
            } else {
                Err(format!("Unknown command: {}", args[0]))
            }
        } else {
            Err(String::from("No command namespace found"))
        }
    }
}