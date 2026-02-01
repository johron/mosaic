use ropey::Rope;
use crate::{Mode, Mosaic};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,

    WordRight,
    WordLeft,
}

#[derive(Debug, Clone)]
enum Edit {
    Insert { at: usize, text: String },
    Delete { range: std::ops::Range<usize> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Editor {
    pub(crate) rope: Rope,
    pub(crate) cursors: Vec<Cursor>,

    // view state
    pub(crate) top_line: usize,
    pub(crate) height: usize,
    pub(crate) insert_inactive: bool,
    show_gutter: bool,

    file_path: Option<String>,
}

impl Editor {
    pub(crate) fn new(initial: Option<&str>, file_path: Option<String>) -> Self {
        let initial = initial.unwrap_or("");
        let rope = Rope::from_str(initial);
        let cursors = vec![Cursor { line: 0, col: 0 }];
        Self {
            rope,
            cursors,
            file_path,
            show_gutter: true,
            top_line: 0,
            height: 0,
            insert_inactive: true,
        }
    }

    pub fn open_file(&mut self, file_path: &str) {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            self.rope = Rope::from_str(&content);
            self.file_path = Some(file_path.to_string());
            self.cursors = vec![Cursor { line: 0, col: 0 }];
            self.top_line = 0;
        }
    }

    pub fn get_file_extension(&self) -> Option<String> {
        if let Some(ref path) = self.file_path {
            if let Some(ext) = std::path::Path::new(path).extension() {
                return Some(ext.to_string_lossy().to_string());
            }
        }
        None
    }

    fn cursor_to_char(&self, c: Cursor) -> usize {
        self.rope.line_to_char(c.line) + c.col
    }

    fn clamp_cursor(rope: &Rope, mut c: Cursor) -> Cursor {
        let max_line = rope.len_lines().saturating_sub(1);
        c.line = c.line.min(max_line);

        let line_len = rope.line(c.line).len_chars();
        let visible = if line_len > 0 && rope.char(rope.line_to_char(c.line) + line_len - 1) == '\n' {
            line_len - 1
        } else {
            line_len
        };

        c.col = c.col.min(visible);
        c
    }

    fn normalize_geometry(&mut self) {
        self.cursors = self.cursors
            .iter()
            .map(|&c| Editor::clamp_cursor(&self.rope, c))
            .collect();

        self.cursors.sort();
    }

    fn dedup_cursors(&mut self) {
        self.cursors.dedup();
    }

    fn apply_edits(&mut self, mut edits: Vec<Edit>) {
        edits.sort_by(|a, b| {
            let pa = match a { Edit::Insert { at, .. } => *at, Edit::Delete { range } => range.start };
            let pb = match b { Edit::Insert { at, .. } => *at, Edit::Delete { range } => range.start };
            pb.cmp(&pa)
        });

        for edit in edits {
            match edit {
                Edit::Insert { at, text } => {
                    self.rope.insert(at, &text);
                }
                Edit::Delete { range } => {
                    self.rope.remove(range);
                }
            }
        }
    }

    pub fn input(&mut self, ch: char) {
        self.normalize_geometry();

        let mut positions: Vec<usize> =
            self.cursors.iter().map(|&c| self.cursor_to_char(c)).collect();

        let mut edits: Vec<Edit> = positions
            .iter()
            .map(|&p| Edit::Insert {
                at: p,
                text: ch.to_string(),
            })
            .collect();

        edits.sort_by(|a, b| {
            let pa = match a {
                Edit::Insert { at, .. } => *at,
                Edit::Delete { range } => range.start,
            };
            let pb = match b {
                Edit::Insert { at, .. } => *at,
                Edit::Delete { range } => range.start,
            };
            pb.cmp(&pa)
        });

        for edit in &edits {
            match edit {
                Edit::Insert { at, text } => {
                    self.rope.insert(*at, text);
                }
                Edit::Delete { range } => {
                    self.rope.remove(range.clone());
                }
            }
        }

        for edit in &edits {
            for pos in &mut positions {
                match edit {
                    Edit::Insert { at, text } => {
                        if *pos >= *at {
                            *pos += text.chars().count();
                        }
                    }
                    Edit::Delete { range } => {
                        if *pos > range.end {
                            *pos -= range.end - range.start;
                        } else if *pos >= range.start {
                            *pos = range.start;
                        }
                    }
                }
            }
        }

        self.cursors = positions
            .into_iter()
            .map(|pos| {
                let line = self.rope.char_to_line(pos);
                let col = pos - self.rope.line_to_char(line);
                Cursor { line, col }
            })
            .collect();

        self.normalize_geometry();
        self.dedup_cursors();
    }

    pub fn backspace(&mut self) {
        self.normalize_geometry();

        let mut positions: Vec<usize> =
            self.cursors.iter().map(|&c| self.cursor_to_char(c)).collect();

        let mut edits: Vec<Edit> = positions
            .iter()
            .filter_map(|&pos| {
                if pos > 0 {
                    Some(Edit::Delete {
                        range: pos - 1..pos,
                    })
                } else {
                    None
                }
            })
            .collect();

        edits.sort_by(|a, b| {
            let sa = match a { Edit::Delete { range } => range.start, _ => 0 };
            let sb = match b { Edit::Delete { range } => range.start, _ => 0 };
            sb.cmp(&sa)
        });
        edits.dedup_by(|a, b| match (a, b) {
            (Edit::Delete { range: r1 }, Edit::Delete { range: r2 }) => r1 == r2,
            _ => false,
        });

        self.apply_edits(edits.clone());

        for edit in &edits {
            for pos in &mut positions {
                match edit {
                    Edit::Delete { range } => {
                        if *pos > range.end {
                            *pos -= range.end - range.start;
                        } else if *pos >= range.start {
                            *pos = range.start;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.cursors = positions
            .into_iter()
            .map(|pos| {
                let line = self.rope.char_to_line(pos);
                let col = pos - self.rope.line_to_char(line);
                Cursor { line, col }
            })
            .collect();

        self.normalize_geometry();
    }


    pub fn tab(&mut self) {
        for _ in 0..4 {
            self.input(' ');
        }
    }

    fn line_visible_len_rope(rope: &Rope, line: usize) -> usize {
        if line >= rope.len_lines() {
            return 0;
        }

        let line = rope.line(line);
        let len = line.len_chars();

        if len == 0 {
            0
        } else if line.char(len - 1) == '\n' {
            len - 1
        } else {
            len
        }
    }

    fn squash_out_of_bounds_cursors(&mut self) {
        let last_line = self.rope.len_lines().saturating_sub(1);

        let mut has_top = false;
        let mut has_bottom = false;

        self.cursors.retain(|c| {
            if c.line == 0 {
                if has_top {
                    false
                } else {
                    has_top = true;
                    true
                }
            } else if c.line == last_line {
                if has_bottom {
                    false
                } else {
                    has_bottom = true;
                    true
                }
            } else {
                true
            }
        });
    }

    pub fn move_cursor(&mut self, dir: CursorDirection) {
        let rope = &self.rope; // immutable borrow ends here, not inside loop
        let line_count = rope.len_lines();

        for c in &mut self.cursors {
            match dir {
                CursorDirection::Left => {
                    if c.col > 0 {
                        c.col -= 1;
                    } else if c.line > 0 {
                        c.line -= 1;
                        c.col = Editor::line_visible_len_rope(rope, c.line);
                    }
                }

                CursorDirection::Right => {
                    let len = Editor::line_visible_len_rope(rope, c.line);
                    if c.col < len {
                        c.col += 1;
                    } else if c.line + 1 < line_count {
                        c.line += 1;
                        c.col = 0;
                    }
                }

                CursorDirection::Up => {
                    if c.line > 0 {
                        c.line -= 1;
                        c.col = c.col.min(Editor::line_visible_len_rope(rope, c.line));
                    }
                }

                CursorDirection::Down => {
                    if c.line + 1 < line_count {
                        c.line += 1;
                        c.col = c.col.min(Editor::line_visible_len_rope(rope, c.line));
                    }
                }

                _ => {}
            }
        }

        self.normalize_geometry();
        self.dedup_cursors();
        self.squash_out_of_bounds_cursors();
    }

    pub fn add_cursor_below(&mut self) {
        let rope = &self.rope;
        let line_count = rope.len_lines();

        let mut new_cursors = Vec::new();

        for &c in &self.cursors {
            let target_line = c.line + 1;
            if target_line >= line_count {
                continue;
            }

            let max_col = Editor::line_visible_len_rope(rope, target_line);
            let new_col = c.col.min(max_col);

            new_cursors.push(Cursor {
                line: target_line,
                col: new_col,
            });
        }

        self.cursors.extend(new_cursors);
        self.normalize_geometry();
        self.dedup_cursors();
        self.squash_out_of_bounds_cursors();
    }

    pub fn add_cursor_above(&mut self) {
        let rope = &self.rope;

        let mut new_cursors = Vec::new();

        for &c in &self.cursors {
            if c.line == 0 {
                continue;
            }

            let target_line = c.line - 1;
            let max_col = Editor::line_visible_len_rope(rope, target_line);
            let new_col = c.col.min(max_col);

            new_cursors.push(Cursor {
                line: target_line,
                col: new_col,
            });
        }

        self.cursors.extend(new_cursors);
        self.normalize_geometry();
        self.dedup_cursors();
        self.squash_out_of_bounds_cursors();
    }

    pub fn clear_cursors(&mut self) {
        // reduce to only one remaining cursor
        self.normalize_geometry();
        let primary = self
            .cursors
            .first()
            .cloned()
            .unwrap_or(Cursor { line: 0, col: 0 });
        self.cursors.clear();
        self.cursors.push(primary);
    }
}