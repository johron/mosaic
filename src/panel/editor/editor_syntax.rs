use std::fs;
use std::path::PathBuf;
use ratatui::prelude::{Line, Span};
use ropey::Rope;
use serde::{Deserialize, Serialize};
use ratatui::style::{Color, Style};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyntaxConfig {
    pub keywords: Vec<String>,
    pub comment_delimiters: Vec<(String, String)>,
    pub string_delimiters: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct SyntaxIndexConfig {
    #[serde(flatten)]
    pub languages: std::collections::HashMap<String, Vec<String>>,
}

fn syntax_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("../../.."));
    path.push("mos");
    path.push("syntax_config.toml");
    path
}

fn syntax_folder() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("../../.."));
    path.push("mos");
    path.push("syntax");
    path
}

pub fn load_syntax_index() -> SyntaxIndexConfig {
    let path = syntax_config_path();

    if !path.exists() {
        eprintln!("Syntax config not found: {:?}", path);
        return SyntaxIndexConfig::default();
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to read syntax config: {err}");
            return SyntaxIndexConfig::default();
        }
    };

    toml::from_str::<SyntaxIndexConfig>(&content).unwrap_or_else(|err| {
        eprintln!("Failed to parse syntax config: {err}");
        SyntaxIndexConfig::default()
    })
}

pub fn load_language_syntax(language: &str) -> Option<SyntaxConfig> {
    let mut path = syntax_folder();
    path.push(format!("{}.toml", language));

    if !path.exists() {
        eprintln!("Syntax file not found for language: {}", language);
        return None;
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to read syntax file {}: {}", language, err);
            return None;
        }
    };

    match toml::from_str::<SyntaxConfig>(&content) {
        Ok(cfg) => Some(cfg),
        Err(err) => {
            eprintln!("Failed to parse syntax file {}: {}", language, err);
            None
        }
    }
}

pub fn syntax_for_extension(extension: &str, index: &SyntaxIndexConfig) -> Option<SyntaxConfig> {
    for (language, exts) in &index.languages {
        if exts.contains(&extension.to_string()) {
            return load_language_syntax(language);
        }
    }
    None
}

impl SyntaxConfig {
    pub(crate) fn highlight(&self, top_line: usize, max_line: usize, rope: &Rope) -> Vec<Line<'static>> {
        let mut lines_spans: Vec<Line> = Vec::new();

        let kw_style = Style::default().fg(Color::Rgb(199, 120, 70));
        let comment_style = Style::default().fg(Color::Gray);
        let string_style = Style::default().fg(Color::Rgb(106, 153, 85));
        let number_style = Style::default().fg(Color::Cyan);
        let gutter_style = Style::default().fg(Color::Gray);

        // state that can span multiple lines
        let mut in_comment_end: Option<String> = None;
        let mut in_string_end: Option<String> = None;

        for i in top_line..max_line {
            let rope_line = rope.line(i);
            let line = rope_line.to_string();
            let len = line.len();
            let mut spans: Vec<Span> = Vec::new();
            let mut pos: usize = 0;

            // If we are already inside a multi-line comment, try to close it on this line
            if let Some(end_delim) = in_comment_end.as_ref() {
                if let Some(rel_end) = line.find(end_delim) {
                    let end_pos = rel_end + end_delim.len();
                    let slice = &line[..end_pos];
                    spans.push(Span::styled(slice.to_string(), comment_style));
                    pos = end_pos;
                    in_comment_end = None;
                } else {
                    // whole line is comment
                    spans.push(Span::styled(line.clone(), comment_style));
                    let mut line_spans = vec![Span::styled(format!("{:4} ", i), gutter_style)];
                    line_spans.extend(spans);
                    lines_spans.push(Line::from(line_spans));
                    continue;
                }
            }

            // If we are already inside a multi-line string, try to close it on this line
            if in_string_end.is_some() && pos < len {
                if let Some(end_delim) = in_string_end.as_ref() {
                    if let Some(rel_end) = line[pos..].find(end_delim) {
                        let end_pos = pos + rel_end + end_delim.len();
                        let slice = &line[pos..end_pos];
                        spans.push(Span::styled(slice.to_string(), string_style));
                        pos = end_pos;
                        in_string_end = None;
                    } else {
                        // rest of line is string
                        let slice = &line[pos..];
                        spans.push(Span::styled(slice.to_string(), string_style));
                        let mut line_spans = vec![Span::styled(format!("{:4} ", i), gutter_style)];
                        line_spans.extend(spans);
                        lines_spans.push(Line::from(line_spans));
                        continue;
                    }
                }
            }

            while pos < len {
                let rest = &line[pos..];

                // comments (single-line or multi-line) - check each delimiter pair
                let mut handled_comment = false;
                for (start_delim, end_delim) in &self.comment_delimiters {
                    if rest.starts_with(start_delim) {
                        // single-line style where end delimiter is newline in config
                        if end_delim == "\n" {
                            spans.push(Span::styled(rest.to_string(), comment_style));
                            pos = len;
                            handled_comment = true;
                            break;
                        }

                        // multi-line style with explicit end delimiter
                        let after = &rest[start_delim.len()..];
                        if let Some(rel_end) = after.find(end_delim) {
                            let end_pos = pos + start_delim.len() + rel_end + end_delim.len();
                            let slice = &line[pos..end_pos];
                            spans.push(Span::styled(slice.to_string(), comment_style));
                            pos = end_pos;
                            handled_comment = true;
                            break;
                        } else {
                            // till end of line and set state to continue next lines
                            spans.push(Span::styled(rest.to_string(), comment_style));
                            in_comment_end = Some(end_delim.clone());
                            pos = len;
                            handled_comment = true;
                            break;
                        }
                    }
                }
                if handled_comment {
                    continue;
                }

                // strings (check each delimiter pair)
                let mut handled_string = false;
                for (s_start, s_end) in &self.string_delimiters {
                    if rest.starts_with(s_start) {
                        let after = &rest[s_start.len()..];
                        if let Some(rel_end) = after.find(s_end) {
                            let end_pos = pos + s_start.len() + rel_end + s_end.len();
                            let slice = &line[pos..end_pos];
                            spans.push(Span::styled(slice.to_string(), string_style));
                            pos = end_pos;
                            handled_string = true;
                            break;
                        } else {
                            // till end of line and remember to continue string
                            spans.push(Span::styled(rest.to_string(), string_style));
                            in_string_end = Some(s_end.clone());
                            pos = len;
                            handled_string = true;
                            break;
                        }
                    }
                }
                if handled_string {
                    continue;
                }

                // whitespace
                let mut ch_iter = rest.chars();
                if let Some(ch) = ch_iter.next() {
                    if ch.is_whitespace() {
                        let mut end = pos;
                        for (j, c) in line[pos..].char_indices() {
                            if !c.is_whitespace() {
                                end = pos + j;
                                break;
                            }
                            end = len;
                        }
                        let slice = &line[pos..end];
                        spans.push(Span::raw(slice.to_string()));
                        pos = end;
                        continue;
                    }

                    // numbers
                    if ch.is_ascii_digit() {
                        let mut end = pos;
                        for (j, c) in line[pos..].char_indices() {
                            if !(c.is_ascii_digit() || c == '.' || c == '_') {
                                end = pos + j;
                                break;
                            }
                            end = len;
                        }
                        let slice = &line[pos..end];
                        spans.push(Span::styled(slice.to_string(), number_style));
                        pos = end;
                        continue;
                    }

                    // word (identifier / keyword)
                    if ch.is_alphanumeric() || ch == '_' {
                        let mut end = pos;
                        for (j, c) in line[pos..].char_indices() {
                            if !(c.is_alphanumeric() || c == '_') {
                                end = pos + j;
                                break;
                            }
                            end = len;
                        }
                        let slice = &line[pos..end];
                        if self.keywords.contains(&slice.to_string()) {
                            spans.push(Span::styled(slice.to_string(), kw_style));
                        } else {
                            spans.push(Span::raw(slice.to_string()));
                        }
                        pos = end;
                        continue;
                    }

                    // any other single char (punctuation, etc.)
                    let ch_len = ch.len_utf8();
                    let slice = &line[pos..pos + ch_len];
                    spans.push(Span::raw(slice.to_string()));
                    pos += ch_len;
                } else {
                    break;
                }
            }

            let mut line_spans = vec![Span::styled(format!("{:4} ", i), gutter_style)]; // small gutter
            line_spans.extend(spans);
            lines_spans.push(Line::from(line_spans));
        }

        lines_spans
    }
}