use std::collections::HashMap;
use ratatui::crossterm::style::Color;
use ratatui::text::{Line, Span};
use ropey::Rope;

#[derive(Debug, Clone)]
pub struct SyntaxHandler {
    pub configs: Vec<SyntaxConfig>,
    pub syntax_entry_config: Vec<SyntaxEntryConfig>
}

impl SyntaxHandler {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            syntax_entry_config: Vec::new(),
        }
    }

    pub fn get_syntax_by_extension(&self, extension: &str) -> Option<&SyntaxConfig> {
        for config in &self.configs {
            if config.extension == extension {
                return Some(config);
            }
        }
        None
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

    pub fn load_syntaxes(&mut self) { // HashMap<extension, filename>
        // each syntax config is stored in a separate file in the `./config/syntaxes` directory
        self.load_syntax_entries();

        // use syntax_entry_config to load syntaxes to get syntax_highlighting
        let syntax_highlighting: Vec<(String, String)> = self
            .syntax_entry_config
            .iter()
            .map(|entry| (entry.extension.clone(), entry.filename.clone()))
            .collect();

        let syntaxes_dir = std::path::Path::new("./config/syntaxes");
        if !syntaxes_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(syntaxes_dir) {
                eprintln!("Failed to create syntaxes directory: {}", e);
                return;
            }
        }

        for (extension, filename) in syntax_highlighting {
            let file_path = syntaxes_dir.join(format!("{}.toml", filename));
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
                self.write_default_syntax(&extension, &filename);
            }
        }
    }

    pub fn load_syntax_entries(&mut self) {
        let config_dir = std::path::Path::new("./config");
        if !config_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(config_dir) {
                eprintln!("Failed to create config directory: {}", e);
                return;
            }
        }

        let file_path = config_dir.join("syntax_entries.toml");
        if !file_path.exists() {
            // create an empty array as a sensible default
            if let Err(e) = std::fs::write(&file_path, "\n") {
                eprintln!("Failed to write default syntax entries `{}`: {}", file_path.display(), e);
            }
            return;
        }

        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                // try parse as a plain array of tables first
                if let Ok(entries) = toml::from_str::<Vec<SyntaxEntryConfig>>(&content) {
                    self.syntax_entry_config = entries;
                    return;
                }

                // fall back to parsing a table with a key `syntax_entries = [ ... ]`
                match toml::from_str::<toml::Value>(&content) {
                    Ok(val) => {
                        if let Some(tbl) = val.get("syntax_entries") {
                            match tbl.clone().try_into::<Vec<SyntaxEntryConfig>>() {
                                Ok(entries) => self.syntax_entry_config = entries,
                                Err(e) => eprintln!("Failed to deserialize `syntax_entries` in `{}`: {}", file_path.display(), e),
                            }
                        } else {
                            eprintln!("No `syntax_entries` array found in `{}`", file_path.display());
                        }
                    }
                    Err(e) => eprintln!("Failed to parse syntax entries `{}`: {}", file_path.display(), e),
                }
            }
            Err(e) => eprintln!("Failed to read syntax entries `{}`: {}", file_path.display(), e),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct SyntaxEntryConfig {
    pub extension: String,
    pub filename: String,
}

impl SyntaxEntryConfig {
    pub fn new(extension: &str, filename: &str) -> Self {
        Self {
            extension: extension.to_string(),
            filename: filename.to_string(),
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

    pub(crate) fn highlight(&self, top_line: usize, max_line: usize, rope: &Rope) -> Vec<Line<'static>> {
        let mut lines_spans: Vec<Line> = Vec::new();

        use ratatui::style::{Color, Style};

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