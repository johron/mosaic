use crate::{Command, Mode};

#[derive(Clone, Debug)]
pub struct StateHandler {
    pub should_quit: bool, // siden det blir så lite ting her så kan denne flyees til mos
    pub mode: Mode,        // --||--
    pub command: Command,  // dete kan nok fjernes siden paneldata har dette, og blir hvor dette lagres.
    // men jeg trenger jo direction her for når jeg skal splitte paneler og sånn
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