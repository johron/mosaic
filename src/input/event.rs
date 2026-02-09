use crossterm::event::{KeyEvent, MouseEvent};

pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
