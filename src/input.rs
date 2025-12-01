use std::io::Error;
use std::ops::AddAssign;
use std::time::Instant;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, ModifierKeyCode};
use tui_textarea::{CursorMove, Input, Key};
use crate::{Mosaic, Mode, Command};
use crate::command;

pub fn handle(mosaic: &mut Mosaic) -> Result<(), Error> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            process_key(mosaic, key_event);
        }
    }

    Ok(())
}

fn process_key(mosaic: &mut Mosaic, key: KeyEvent) {
    match mosaic.mode {
        Mode::Normal => handle_normal_mode(mosaic, key),
        Mode::Insert => handle_insert_mode(mosaic, key),
        Mode::Command => handle_command_mode(mosaic, key),
    }
}

fn handle_normal_mode(mosaic: &mut Mosaic, key_event: KeyEvent) {
    if key_event.modifiers.is_empty() {
        match key_event.code {
            KeyCode::Esc => {
                mosaic.command.result = None;
            },
            KeyCode::Char('i') => mosaic.set_mode(Mode::Insert),
            KeyCode::Char(':') => {
                mosaic.command.result = None;
                mosaic.set_mode(Mode::Command)
            },

            KeyCode::Char('j') | KeyCode::Left => mosaic.text_area.move_cursor(CursorMove::Back),
            KeyCode::Char('k') | KeyCode::Up => mosaic.text_area.move_cursor(CursorMove::Up),
            KeyCode::Char('l') | KeyCode::Down => mosaic.text_area.move_cursor(CursorMove::Down),
            KeyCode::Char('ø') | KeyCode::Right => mosaic.text_area.move_cursor(CursorMove::Forward),

            _ => {}
        }
    } else {
        match key_event {
            KeyEvent { code: KeyCode::Char('j') | KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::WordBack)
            },
            KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::Up)
            },
            KeyEvent { code: KeyCode::Char('l') | KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::Down)
            },
            KeyEvent { code: KeyCode::Char('ø') | KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::WordForward)
            },
            _ => {

            }
        }
    }
}

fn handle_non_modifier(mosaic: &mut Mosaic, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => mosaic.set_mode(Mode::Normal),
        KeyCode::Char(c) => {
            mosaic.text_area.input(Input {
                key: Key::Char(c),
                ctrl: false,
                alt: false,
                shift: false,
            });
        },
        KeyCode::Left => mosaic.text_area.move_cursor(CursorMove::Back),
        KeyCode::Up => mosaic.text_area.move_cursor(CursorMove::Up),
        KeyCode::Down => mosaic.text_area.move_cursor(CursorMove::Down),
        KeyCode::Right => mosaic.text_area.move_cursor(CursorMove::Forward),

        KeyCode::Enter => mosaic.text_area.insert_newline(),

        KeyCode::Backspace => {
            mosaic.text_area.delete_char();
        },
        _ => {}
    }
}

fn handle_insert_mode(mosaic: &mut Mosaic, key_event: KeyEvent) {
    if key_event.modifiers.is_empty() {
        handle_non_modifier(mosaic, key_event);
    } else {
        match key_event {
            KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::WordBack)
            },
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::Top)
            },
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::Bottom)
            },
            KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                mosaic.text_area.move_cursor(CursorMove::WordForward)
            },
            _ => {
                // KeyEvent is alphabetic? do here
                handle_non_modifier(mosaic, key_event);
            }
        }
    }
}

fn handle_command_mode(mosaic: &mut Mosaic, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            mosaic.command.result = None;
            mosaic.set_mode(Mode::Normal)
        },
        KeyCode::Enter => {
            let res = command::handle_command(mosaic);

            mosaic.command = Command {
                content: String::new(),
                result: Some(res.unwrap_or_else(|e| format!("Error: {}", e))),
            };

            mosaic.set_mode(Mode::Normal);
        },
        KeyCode::Char(c) => {
            mosaic.command += c.to_string().as_str();
        },
        KeyCode::Backspace => {
            mosaic.command.pop();
        },
        _ => {}
    }
}
