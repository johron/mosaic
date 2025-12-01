use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;
use crate::{Mosaic, Mode};

pub fn draw(frame: &mut Frame, mosaic: &mut Mosaic) {
    mosaic.text_area.set_block(
        match mosaic.mode {
            Mode::Normal => {
                if mosaic.command.result.is_some() {
                    Block::new()
                        .title_bottom(format!("{}", mosaic.command.result.as_ref().unwrap()))
                        .title_alignment(Alignment::Left)
                } else {
                    Block::new()
                        .title_bottom(format!("{}", mosaic.mode))
                        .title_alignment(Alignment::Right)
                }
            },
            Mode::Insert => {
                Block::new()
            },
            Mode::Command => {
                Block::new()
                    .title_bottom(format!(": {}", mosaic.command.content))
            },
        }
    );
    frame.render_widget(&mosaic.text_area, frame.area());
}
