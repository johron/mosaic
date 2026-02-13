use std::path::PathBuf;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ropey::Rope;
use crate::app::MosId;
use crate::event::event::Event;
use crate::panel::panel::{Panel};

pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub goal_column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize, goal_column: usize) -> Self {
        Self {
            line,
            column,
            goal_column,
        }
    }
}

pub struct EditorPanel {
    pub rope: Rope,
    pub cursors: Vec<Cursor>,
    pub file_path: Option<PathBuf>,
    pub scroll_offset: usize,
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            cursors: vec![Cursor::new(0, 0, 0)],
            file_path: None,
            scroll_offset: 0,
        }
    }

    fn highlight_line(&self, max_line: usize) -> Vec<Line<'static>> {
        //if let Some(syntax) = &self.editor.syntax {
        //    syntax.highlight(
        //        scroll_offset,
        //        max_line,
        //        &rope,
        //    )
        //} else {
        //    Self::no_highlight_line(scroll_offset, max_line, rope)
        //}
        self.no_highlight_line(max_line)
    }

    fn no_highlight_line(&self, max_line: usize) -> Vec<Line<'static>> {
        let mut lines_spans: Vec<Line> = Vec::new();

        for i in self.scroll_offset..max_line {
            let rope_line = self.rope.line(i);
            let text_line = rope_line.to_string();
            let spans = vec![Span::raw(text_line)];
            let mut line_spans = vec![Span::styled(format!("{:4} ", i), Style::default().fg(Color::Gray))]; // small gutter

            line_spans.extend(spans);
            lines_spans.push(Line::from(line_spans));
        }

        lines_spans
    }
}

impl Panel for EditorPanel {
    fn id(&self) -> MosId {
        MosId::new()
    }

    fn title(&self) -> &str {
        "Editor"
    }

    fn handle_event(&mut self, event: Event) {
        todo!()
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        //frame.render_widget(ratatui::widgets::Paragraph::new("self.rope.to_string()"), area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let height = std::cmp::max(1, chunks[0].height as usize - 1);

        let max_line = std::cmp::min(
            self.rope.len_lines(),
            self.scroll_offset.saturating_add(height),
        );

        let lines_spans: Vec<Line> = self.highlight_line(max_line);

        // Have to think about how I can to the multiple editor panels later, block should be set from outside, not in editor panel
        let paragraph = Paragraph::new(lines_spans);
        //.bg(Color::Red);
        //.block(block);

        frame.render_widget(paragraph, chunks[0]);

        for cursor in &self.cursors {
            let x = chunks[0].x + 5 + cursor.column as u16; // 5 for gutter
            let y = chunks[0].y + (cursor.line.saturating_sub(self.scroll_offset)) as u16;
            frame.render_widget(
                Paragraph::new("")
                    .style(Style::default()
                        .add_modifier(Modifier::REVERSED)
                        .fg(Color::White)),
                //.fg(if panel.active { Color::White } else { Color::DarkGray })),
                Rect::new(x, y, 1, 1),
            );
        }
    }
}