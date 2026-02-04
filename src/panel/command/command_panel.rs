use crate::handler::state_handler::StateHandler;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FloatingCommandPanel {}

impl FloatingCommandPanel {
    pub fn draw(frame: &mut Frame, size: Rect, state_handler: &StateHandler) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        let line: Option<Line> = match &state_handler.mode {
            crate::Mode::Normal => {
                if state_handler.command.result.is_none() {
                    return;
                }

                Some(Line::from(
                    state_handler.command.result.as_ref().unwrap().to_owned()
                ))
            }
            crate::Mode::Command => {
                Some(Line::from(
                    "/".to_owned() + &state_handler.command.content
                ))
            },
            _ => {
                None
            },
        };

        let paragraph = Paragraph::new(line.unwrap())
            .style(Style::default()
                .fg(Color::White))
            .block(Block::default()
                .borders(Borders::TOP)
            );

        frame.render_widget(paragraph, chunks[0]);
    }
}