mod highlight;

use crate::ui::highlight::highlight_line;
use crate::{Mode, Mosaic};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    prelude::*,
    widgets::Block,
};
use regex::Regex;

pub fn draw(frame: &mut Frame, mosaic: &mut Mosaic) {
    mosaic.editor.set_block(
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
                    .title_bottom(format!("/{}", mosaic.command.content))
            },
        }
    );

    //frame.render_widget(&mosaic.editors[mosaic.current_editor].text_area, frame.area());
    let rust_keywords = Regex::new(r"^(fn|let|mut|struct|enum|impl|for|while|loop|if|else|match|use|pub|mod|crate)\b").unwrap();
    let number_re = Regex::new(r"^\d+").unwrap();

    let size = frame.area();
    // layout: whole area for editor
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    // render lines as Spans
    let top_line = mosaic.editor.top_line;
    let mut lines_spans: Vec<Line> = Vec::new();
    let height = chunks[0].height as usize - 1;

    mosaic.editor.height = height;

    let max_line = std::cmp::min(
        mosaic.editor.rope.len_lines(),
        top_line.saturating_add(height),
    );
    
    for i in top_line..max_line {
        let rope_line = mosaic.editor.rope.line(i);
        let text_line = rope_line.to_string();
        let spans = highlight_line(&text_line, &rust_keywords, &number_re);
        let mut line_spans = vec![Span::raw(format!("{:4} ", i))]; // small gutter

        line_spans.extend(spans);
        lines_spans.push(Line::from(line_spans));
    }

    let paragraph = Paragraph::new(lines_spans)
        .block(mosaic.editor.block.clone());

    frame.render_widget(paragraph, chunks[0]);

    if let Some(toast) = &mosaic.toast {
        let (toast_paragraph, toast_area) = draw_toast(frame.area(), &toast.message);
        frame.render_widget(toast_paragraph, toast_area);
    }

    // render cursors
    for cursor in &mosaic.editor.cursors {
        let cursor_x = chunks[0].x + 5 + cursor.col as u16; // 5 for gutter
        let cursor_y = chunks[0].y + (cursor.line.saturating_sub(top_line)) as u16;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }
}

fn draw_toast(area: Rect, message: &str) -> (Paragraph, Rect) {
    let size = area;
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Mosaic")
        .title_alignment(Alignment::Center);
    let paragraph = Paragraph::new(message.to_string())
        .block(block)
        .alignment(Alignment::Center);

    let area = Rect {
        x: size.x + size.width / 4,
        y: size.y + size.height / 3,
        width: size.width / 2,
        height: size.height / 6,
    };

    (paragraph, area)
}