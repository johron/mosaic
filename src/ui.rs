use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crate::{Kilo, Mode};

pub fn draw(frame: &mut Frame, kilo: &Kilo) {
    let mode_label = match kilo.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Command => "COMMAND",
    };

    let status = Paragraph::new(mode_label)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    let area = frame.size();
    frame.render_widget(status, area);
}
