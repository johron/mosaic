use std::collections::HashMap;
use ratatui::crossterm::style::Color;
use ratatui::text::Span;

#[derive(Debug, Clone)]
pub struct SyntaxHandler {
    pub configs: Vec<SyntaxConfig>,
}

impl SyntaxHandler {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    fn write_default_syntax(&mut self, extension: &str, filename: &str) {
        let syntaxes_dir = std::path::Path::new("./config/syntaxes");
        if !syntaxes_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(syntaxes_dir) {
                eprintln!("Failed to create syntaxes directory: {}", e);
                return;
            }
        }

        let file_path = syntaxes_dir.join(filename.to_owned() + ".toml");
        if !file_path.exists() {
            let default_config = SyntaxConfig::new(extension);
            match toml::to_string_pretty(&default_config) {
                Ok(toml_string) => {
                    if let Err(e) = std::fs::write(&file_path, toml_string) {
                        eprintln!("Failed to write default syntax config to `{}`: {}", file_path.display(), e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to serialize default syntax config for `{}`: {}", file_path.display(), e);
                }
            }
        }

    }

    pub fn load_syntaxes(&mut self, syntax_highlighting: HashMap<String, String>) { // HashMap<extension, filename>
        // each syntax config is stored in a separate file in the "syntaxes" directory

        let syntaxes_dir = std::path::Path::new("./config/syntaxes");
        if !syntaxes_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(syntaxes_dir) {
                eprintln!("Failed to create syntaxes directory: {}", e);
                return;
            }
        }

        for (extension, filename) in syntax_highlighting.iter() {
            let file_path = syntaxes_dir.join(filename.to_owned() + ".toml");
            if file_path.exists() {
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => {
                        match toml::from_str::<SyntaxConfig>(&content) {
                            Ok(config) => {
                                self.configs.push(config);
                            },
                            Err(e) => {
                                eprintln!("Failed to parse syntax config `{}`: {}", file_path.display(), e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read syntax config `{}`: {}", file_path.display(), e);
                    }
                }
            } else {
                // write default syntax config if file doesn't exist
                self.write_default_syntax(extension, filename);
            }
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct SyntaxConfig {
    pub extension: String,
    pub keywords: Vec<String>,
    pub comment_delimiters: Vec<(String, String)>,
    pub string_delimiters: Vec<(String, String)>,
}

impl SyntaxConfig {
    fn new(extension: &str) -> Self {
        Self {
            extension: extension.to_string(),
            keywords: Vec::new(),
            comment_delimiters: Vec::new(),
            string_delimiters: Vec::new(),
        }
    }

    pub(crate) fn highlight_line(&self, line: String) -> Vec<Span<'static>> {
        let mut spans: Vec<Span> = Vec::new();
        let mut pos: usize = 0;
        let len = line.len();

        use ratatui::style::{Color, Style};
        let kw_style = Style::default().fg(Color::Rgb(199, 120, 70));
        let comment_style = Style::default().fg(Color::Gray);
        let string_style = Style::default().fg(Color::Rgb(106, 153, 85));
        let number_style = Style::default().fg(Color::Cyan);

        while pos < len {
            let rest = &line[pos..];

            // single-line comment
            // comments (single-line or multi-line) - check each delimiter pair
            let mut handled_comment = false;
            for (start_delim, end_delim) in &self.comment_delimiters {
                if rest.starts_with(start_delim) {
                    // single-line style where end delimiter is newline in config
                    if end_delim == "\n" {
                        spans.push(Span::styled(rest.to_string(), comment_style));
                        return spans;
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
                        // till end of line
                        spans.push(Span::styled(rest.to_string(), comment_style));
                        return spans;
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
                        // till end of line
                        spans.push(Span::styled(rest.to_string(), string_style));
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
                    for (i, c) in line[pos..].char_indices() {
                        if !c.is_whitespace() {
                            end = pos + i;
                            break;
                        }
                        // if we reached the end without breaking, set end to len
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
                    for (i, c) in line[pos..].char_indices() {
                        if !(c.is_ascii_digit() || c == '.' || c == '_') {
                            end = pos + i;
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
                    for (i, c) in line[pos..].char_indices() {
                        if !(c.is_alphanumeric() || c == '_') {
                            end = pos + i;
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

        spans
    }
}