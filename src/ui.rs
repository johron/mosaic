use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;
use crate::{Kilo, Mode};

pub fn draw(frame: &mut Frame, kilo: &Kilo) {
    let mode_label = match kilo.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Command => "COMMAND",
    };

    let status = Paragraph::new(mode_label)
        .block(Block::default().borders(Borders::TOP))
        .alignment(Alignment::Center);

    frame.render_widget(status.clone(), frame.area());
    frame.render_widget(TextArea::new(vec!["test".to_string()]).widget(), frame.area());
}
