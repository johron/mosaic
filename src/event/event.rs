use crossterm::event::{KeyEvent, MouseEvent};

pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
pub enum Event {
    Input(InputEvent),
    Command(String, Vec<String>),
    Tick,
}