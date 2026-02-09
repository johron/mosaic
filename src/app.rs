use crossterm::event::Event;
use ratatui::Frame;

pub struct Mos {
    pub should_quit: bool,
}

impl Mos {
    pub fn new() -> Self {
        Mos {
            should_quit: false,
        }
    }

    pub fn update(&mut self) {
        // Update app state here
    }

    pub fn handle_terminal_event(&mut self, event: Event) {
        // Handle terminal events here
    }

    pub fn render(&mut self, _frame: &mut Frame) {
        // Render UI here
    }
}