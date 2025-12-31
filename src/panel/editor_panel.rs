use crate::editor::Editor;
use crate::handler::state_handler::StateHandler;
use crate::handler::syntax_handler::SyntaxHandler;
use crate::Mode;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Position};
use ratatui::prelude::{Color, Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use ropey::Rope;

#[derive(Clone, Debug)]
pub struct EditorPanel {
    pub editor: Editor,
    pub syntax_handler: SyntaxHandler
}

impl EditorPanel {
    pub fn new() -> Self {
        let mut syntax_handler = SyntaxHandler::new();
        syntax_handler.load_syntaxes();

        Self {
            editor: Editor::new(None, None),
            syntax_handler,
        }
    }
    
    pub fn draw(&mut self, frame: &mut Frame, state_handler: &mut StateHandler) {
        let block = match state_handler.mode {
            Mode::Normal => {
                if state_handler.command.result.is_some() {
                    Block::new()
                        .title_bottom(format!("{}", state_handler.command.result.as_ref().unwrap()))
                        .title_alignment(Alignment::Left)
                } else {
                    Block::new()
                        .title_bottom(format!("{}", state_handler.mode))
                        .title_alignment(Alignment::Right)
                }
            },
            Mode::Insert => {
                Block::new()
            },
            Mode::Command => {
                Block::new()
                    .title_bottom(format!("/{}", state_handler.command.content))
            },
        };
        
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        let top_line = self.editor.top_line;
        let height = chunks[0].height as usize - 1;

        self.editor.height = height;


        let max_line = std::cmp::min(
            self.editor.rope.len_lines(),
            top_line.saturating_add(height),
        );

        let lines_spans: Vec<Line> = self.highlight_line(top_line, max_line, self.editor.rope.clone());

        // Have to think about how I can to the multiple editor panels later, block should be set from outside, not in editor panel
        let paragraph = Paragraph::new(lines_spans)
            .block(block);

        frame.render_widget(paragraph, chunks[0]);

        // render cursors
        for cursor in &self.editor.cursors {
            let cursor_x = chunks[0].x + 5 + cursor.col as u16; // 5 for gutter
            let cursor_y = chunks[0].y + (cursor.line.saturating_sub(top_line)) as u16;
            frame.set_cursor_position(Position::new(cursor_x, cursor_y));
        }
    }

    fn highlight_line(&mut self, top_line: usize, max_line: usize, rope: Rope) -> Vec<Line> {
        if let Some(syntax) = self.syntax_handler.get_syntax_by_extension(self.editor.get_file_extension().unwrap_or(String::from("")).as_str()) {
            syntax.highlight(
                top_line,
                max_line,
                &rope,
            )
        } else {
            let mut lines_spans: Vec<Line> = Vec::new();

            for i in top_line..max_line {
                let rope_line = self.editor.rope.line(i);
                let text_line = rope_line.to_string();
                // convert text_line to spans, do not highlight for now
                let spans = vec![Span::raw(text_line)];
                let mut line_spans = vec![Span::styled(format!("{:4} ", i), ratatui::style::Style::default().fg(Color::Gray))]; // small gutter

                line_spans.extend(spans);
                lines_spans.push(Line::from(line_spans));
            }

            lines_spans
        }
    }
}