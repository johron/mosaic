use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::Paragraph;
use ropey::Rope;
use crate::handler::panel_handler::{Panel, PanelData, PanelKind};
use crate::panel::new_editor::editor::EditorData;

pub fn draw_editor_panel(panel: &mut Panel, frame: &mut Frame, area: Rect) {
    if panel.kind != PanelKind::Editor {
        return;
    }

    let mut data = match &panel.data {
        PanelData::Editor { rope, top_line, cursors, mode } => {
            EditorData {
                rope: rope.clone(),
                scroll_offset: *top_line,
                cursors: cursors.clone(),
                mode: mode.clone(),
            }
        },
        _ => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let height = std::cmp::max(1, chunks[0].height as usize - 1);

    let max_line = std::cmp::min(
        data.rope.len_lines(),
        data.scroll_offset.saturating_add(height),
    );

    let lines_spans: Vec<Line> = highlight_line(data.scroll_offset, max_line, data.rope.clone());

    // Have to think about how I can to the multiple editor panels later, block should be set from outside, not in editor panel
    let paragraph = Paragraph::new(lines_spans);
    //.bg(Color::Red);
    //.block(block);

    frame.render_widget(paragraph, chunks[0]);

    for cursor in &data.cursors {
        let x = chunks[0].x + 5 + cursor.col as u16; // 5 for gutter
        let y = chunks[0].y + (cursor.line.saturating_sub(data.scroll_offset)) as u16;
        frame.render_widget(
            Paragraph::new("")
                .style(Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .fg(if panel.active { Color::White } else { Color::DarkGray })),
            Rect::new(x, y, 1, 1),
        );
    }

    panel.data = PanelData::Editor {
        rope: data.rope,
        top_line: data.scroll_offset,
        cursors: data.cursors,
        mode: data.mode,
    };
}

fn highlight_line(scroll_offset: usize, max_line: usize, rope: Rope) -> Vec<Line<'static>> {
    //if let Some(syntax) = &self.editor.syntax {
    //    syntax.highlight(
    //        scroll_offset,
    //        max_line,
    //        &rope,
    //    )
    //} else {
    //    Self::no_highlight_line(scroll_offset, max_line, rope)
    //}
    no_highlight_line(scroll_offset, max_line, rope)
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