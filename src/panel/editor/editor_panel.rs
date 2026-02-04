use crate::panel::editor::editor_logic::Editor;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Span};
use ratatui::style::Modifier;
use ratatui::style::{Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ropey::Rope;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorPanel {
    pub editor: Editor,
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(None, None),
        }
    }
    
    pub fn draw(&mut self, frame: &mut Frame, size: Rect) {//, state_handler: &mut StateHandler) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        let top_line = self.editor.top_line;
        let height = std::cmp::max(1, chunks[0].height as usize - 1);
        //let height = size.height as usize - 1;

        self.editor.height = height;

        let max_line = std::cmp::min(
            self.editor.rope.len_lines(),
            top_line.saturating_add(height),
        );

        let lines_spans: Vec<Line> = self.highlight_line(top_line, max_line, self.editor.rope.clone());

        // Have to think about how I can to the multiple editor panels later, block should be set from outside, not in editor panel
        let paragraph = Paragraph::new(lines_spans);
            //.bg(Color::Red);
            //.block(block);

        frame.render_widget(paragraph, chunks[0]);

        let is_insert_inactive = matches!(self.editor.insert_inactive, true);

        for cursor in &self.editor.cursors {
            let x = chunks[0].x + 5 + cursor.col as u16; // 5 for gutter
            let y = chunks[0].y + (cursor.line.saturating_sub(top_line)) as u16;
            frame.render_widget(
                Paragraph::new("")
                    .style(Style::default()
                        .add_modifier(Modifier::REVERSED)
                        .fg(if is_insert_inactive { Color::DarkGray } else { Color::White })),
                Rect::new(x, y, 1, 1),
            );
        }
    }

    fn highlight_line(&mut self, top_line: usize, max_line: usize, rope: Rope) -> Vec<Line> {
        if let Some(syntax) = &self.editor.syntax {
            syntax.highlight(
                top_line,
                max_line,
                &rope,
            )
        } else {
            Self::no_highlight_line(top_line, max_line, rope)
        }
    }

    fn no_highlight_line(top_line: usize, max_line: usize, rope: Rope) -> Vec<Line<'static>> {
        let mut lines_spans: Vec<Line> = Vec::new();

        for i in top_line..max_line {
            let rope_line = rope.line(i);
            let text_line = rope_line.to_string();
            let spans = vec![Span::raw(text_line)];
            let mut line_spans = vec![Span::styled(format!("{:4} ", i), Style::default().fg(Color::Gray))]; // small gutter

            line_spans.extend(spans);
            lines_spans.push(Line::from(line_spans));
        }

        lines_spans
    }
}