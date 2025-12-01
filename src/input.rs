use std::io::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::{Kilo, Mode};

pub fn handle(kilo: &mut Kilo) -> Result<(), Error> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            process_key(kilo, key_event);
        }
    }

    Ok(())
}

fn process_key(kilo: &mut Kilo, key: KeyEvent) {
    match kilo.mode {
        Mode::Normal => handle_normal_mode(kilo, key),
        Mode::Insert => handle_insert_mode(kilo, key),
        Mode::Command => handle_command_mode(kilo, key),
    }
}

fn handle_normal_mode(kilo: &mut Kilo, key: KeyEvent) {
    match key.code {
        KeyCode::Char('i') => kilo.mode = Mode::Insert,
        KeyCode::Char(':') => kilo.mode = Mode::Command,
        KeyCode::Char('q') => kilo.should_quit = true,
        _ => {}
    }
}

fn handle_insert_mode(kilo: &mut Kilo, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => kilo.mode = Mode::Normal,
        KeyCode::Char(c) => {
            // TODO: add char to buffer
            //println!("Inserting: {}", c);
        }
        KeyCode::Backspace => {
            // TODO: delete char
        }
        _ => {}
    }
}

fn handle_command_mode(kilo: &mut Kilo, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => kilo.mode = Mode::Normal,
        KeyCode::Enter => {
            // TODO: parse command
            kilo.mode = Mode::Normal;
        }
        KeyCode::Char(c) => {
            // TODO: append to command buffer
        }
        KeyCode::Backspace => {
            // TODO: pop from command buffer
        }
        _ => {}
    }
}
