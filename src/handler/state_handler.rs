use crate::{Command, Mode};

#[derive(Clone, Debug)]
pub struct StateHandler {
    pub should_quit: bool,
    pub mode: Mode,
    pub command: Command,
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            command: Command::new(),
        }
    }
}