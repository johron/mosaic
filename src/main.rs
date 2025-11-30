mod ui;
mod input;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use tui_textarea::Input;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug, Copy, Clone)]
struct Kilo {
    mode: Mode,
    should_quit: bool,
}

impl Kilo {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            should_quit: false,
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn quit(mut self){
        self.should_quit = true;
    }
}

enum Transition {
    Nop,
    Mode(Mode),
    Pending(Input),
    Quit,
}

fn main() -> Result<()> {
    let kilo = Kilo::new(Mode::Normal);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, kilo);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut kilo: Kilo) -> Result<()> {
    loop {
        /*match kilo.mode {
            Mode::Normal => {
                terminal.draw(|frame| {
                    normal::draw(frame, &kilo); // pass immutable state
                })?;
            },
            Mode::Insert => {
                terminal.draw(|frame| {
                    insert::draw(frame, &kilo); // pass immutable state
                })?;
            },
            Mode::Command => {
                terminal.draw(command::draw)?;
            },
        }*/

        terminal.draw(|frame| {
            ui::draw(frame, &kilo); // pass immutable state
        })?;

        input::handle(&mut kilo).expect("TODO: panic message");
        
        if kilo.should_quit {
            break Ok(());
        }
    }
}