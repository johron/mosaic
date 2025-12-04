use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use regex::Regex;

pub fn highlight_line(line: &str, rust_kw: &Regex, num_re: &Regex) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < line.len() {
        let remainder = &line[i..];
        if let Some(mat) = rust_kw.find(remainder) {
            if mat.start() > 0 {
                spans.push(Span::raw(remainder[..mat.start()].to_string()));
            }
            spans.push(Span::styled(
                mat.as_str().to_string(),
                Style::default().add_modifier(Modifier::BOLD).fg(ratatui::style::Color::Rgb(199, 120, 70)),
            ));
            i += mat.end();
            continue;
        }
        if let Some(mat) = num_re.find(remainder) {
            if mat.start() > 0 {
                spans.push(Span::raw(remainder[..mat.start()].to_string()));
            }
            spans.push(Span::styled(mat.as_str().to_string(), Style::default().add_modifier(Modifier::ITALIC)));
            i += mat.end();
            continue;
        }

        if remainder.starts_with('"') {
            if let Some(end_quote_pos) = remainder[1..].find('"') {
                let str_literal = &remainder[..=end_quote_pos + 1];
                let string_style = Style::default().fg(ratatui::style::Color::Rgb(106, 153, 85));
                let todo_re = Regex::new("(?i)todo").unwrap();
                let mut last = 0usize;
                for mat in todo_re.find_iter(str_literal) {
                    if mat.start() > last {
                        spans.push(Span::styled(str_literal[last..mat.start()].to_string(), string_style));
                    }
                    spans.push(Span::styled(
                        str_literal[mat.start()..mat.end()].to_string(),
                        Style::default().bg(ratatui::style::Color::Rgb(121, 174, 60)).fg(ratatui::style::Color::Black),
                    ));
                    last = mat.end();
                }
                if last == 0 {
                    spans.push(Span::styled(str_literal.to_string(), string_style));
                } else if last < str_literal.len() {
                    spans.push(Span::styled(str_literal[last..].to_string(), string_style));
                }
                i += str_literal.len();
                continue;
            }
        }

        // user requested search/find highlighting
        let search_query = "test"; // just example
        if !search_query.is_empty() {
            if let Some(pos) = remainder.find(search_query) {
                if pos > 0 {
                    spans.push(Span::raw(remainder[..pos].to_string()));
                }
                spans.push(Span::styled(
                    search_query.to_string(),
                    Style::default().bg(ratatui::style::Color::Yellow).fg(ratatui::style::Color::Black),
                ));
                i += pos + search_query.len();
                continue;
            }
        }

        // nothing matched at start, push one char and continue
        let ch = remainder.chars().next().unwrap();
        spans.push(Span::raw(ch.to_string()));
        i += ch.len_utf8();
    }
    spans
}