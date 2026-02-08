use crate::{Command};

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Panel,
    Command,
    Terminal,
}

#[derive(Clone, Debug)]
pub struct StateHandler {
    pub state: State,
    pub should_quit: bool, // siden det blir så lite ting her så kan denne flyees til mos
    pub command: Command,  // dete kan nok fjernes siden paneldata har dette, og blir hvor dette lagres.
    // men jeg trenger jo direction her for når jeg skal splitte paneler og sånn
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            state: State::Panel,
            should_quit: false,
            command: Command::new(),
        }
    }
}