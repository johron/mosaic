use std::cmp::min;
use ratatui::widgets::Block;
use ropey::Rope;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Cursor {
    pub(crate) line: usize,
    pub(crate) col: usize,
}

#[derive(Debug)]
pub(crate) struct Editor<'a> {
    pub(crate) rope: Rope,
    pub(crate) cursors: Vec<Cursor>,
    pub(crate) file_path: Option<String>,
    show_gutter: bool,
    pub(crate) block: Block<'a>,
}

pub enum CursorMove {
    Back,
    Forward,
    Up,
    Down,

    WordBack,
    WordForward,
}

impl<'a> Editor<'a> {
    pub(crate) fn new(initial: &str, file_path: Option<String>) -> Self {
        let rope = Rope::from_str(initial);
        let mut cursors = vec![Cursor { line: 0, col: 0 }];
        cursors[0] = Self::clamp_cursor(&rope, cursors[0].clone());
        Self {
            rope,
            cursors,
            file_path,
            show_gutter: true,
            block: Block::default(),
        }
    }

    fn line_visible_len(&self, line: usize) -> usize {
        let len = self.rope.line(line).len_chars();
        if len == 0 {
            return 0;
        }
        let start = self.rope.line_to_char(line);
        // safe because len > 0
        let last = self.rope.char(start + len - 1);
        if last == '\n' {
            len - 1
        } else {
            len
        }
    }

    fn cursor_abs_pos(&self, cur: &Cursor) -> usize {
        self.rope.line_to_char(cur.line) + cur.col
    }

    fn char_under_cursor(&self, cur: &Cursor) -> Option<char> {
        let vis_len = self.line_visible_len(cur.line);
        if cur.col < vis_len {
            let pos = self.cursor_abs_pos(cur);
            Some(self.rope.char(pos))
        } else {
            None
        }
    }

    fn clamp_cursor(rope: &Rope, mut c: Cursor) -> Cursor {
        let line_count = rope.len_lines();
        if c.line >= line_count.saturating_sub(1) + 1 {
            c.line = line_count.saturating_sub(1);
        }
        let line_len = {
            // compute visible len using rope methods
            let len = rope.line(c.line).len_chars();
            if len == 0 {
                0
            } else {
                let start = rope.line_to_char(c.line);
                let last = rope.char(start + len - 1);
                if last == '\n' { len - 1 } else { len }
            }
        };
        if c.col > line_len {
            c.col = line_len;
        }
        c
    }

    pub(crate) fn input(&mut self, ch: char) {
        // Insert at each cursor. To avoid offsets messing up, convert to absolute char indices,
        // sort descending and insert in that order.
        let mut positions: Vec<usize> = self
            .cursors
            .iter()
            .map(|cur| {
                let line_start = self.rope.line_to_char(cur.line);
                line_start + cur.col
            })
            .collect();

        // sort unique descending (if two cursors at same pos, insert once per cursor still OK,
        // but we keep them separate so each receives a char).
        let mut pos_with_idx: Vec<(usize, usize)> =
            positions.iter().copied().enumerate().map(|(i, p)| (p, i)).collect();
        pos_with_idx.sort_by(|a, b| b.0.cmp(&a.0)); // descending by position

        for (pos, _idx) in pos_with_idx {
            self.rope.insert_char(pos, ch);
        }

        // After insert, advance all cursors' columns by 1 (for simplicity).
        for cur in &mut self.cursors {
            if ch == '\n' {
                cur.line += 1;
                cur.col = 0;
            } else {
                cur.col += 1;
            }

            *cur = Self::clamp_cursor(&self.rope, cur.clone());
        }
    }

    pub fn backspace(&mut self) {
        // Delete character before each cursor. We must compute absolute positions and process descending.
        let mut positions: Vec<usize> = self
            .cursors
            .iter()
            .map(|cur| {
                let line_start = self.rope.line_to_char(cur.line);
                line_start + cur.col
            })
            .collect();

        // For each position, if > 0 remove char at pos-1.
        positions.sort_unstable();
        positions.dedup(); // avoid duplicate deletions at same byte pos
        positions.reverse(); // delete descending
        for pos in positions {
            if pos > 0 {
                self.rope.remove(pos - 1..pos);
            }
        }

        let mut idx = 0;
        while idx < self.cursors.len() {
            // read current cursor state with a short immutable borrow
            let (col, line) = {
                let c = &self.cursors[idx];
                (c.col, c.line)
            };

            if col > 0 {
                let cur = &mut self.cursors[idx];
                cur.col -= 1;
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            } else if line > 0 {
                // compute visible length of previous line using only `self.rope`
                let prev_line = line - 1;
                let new_col = {
                    let len = self.rope.line(prev_line).len_chars();
                    if len == 0 {
                        0
                    } else {
                        let start = self.rope.line_to_char(prev_line);
                        let last = self.rope.char(start + len - 1);
                        if last == '\n' { len - 1 } else { len }
                    }
                };
                let cur = &mut self.cursors[idx];
                cur.line = prev_line;
                cur.col = new_col;
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            } else {
                let cur = &mut self.cursors[idx];
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            }

            idx += 1;
        }
    }

    pub fn move_cursor(&mut self, direction: CursorMove) {
        let idx = 0; // for now, move only the first cursor

        match direction {
            CursorMove::Back => {
                if self.cursors[idx].col > 0 {
                    self.cursors[idx].col -= 1;
                } else if self.cursors[idx].line > 0 {
                    self.cursors[idx].line -= 1;
                    self.cursors[idx].col = self.line_visible_len(self.cursors[idx].line);
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
            CursorMove::Forward => {
                let line_len = self.line_visible_len(self.cursors[idx].line);
                if self.cursors[idx].col < line_len {
                    self.cursors[idx].col += 1;
                } else if self.cursors[idx].line + 1 < self.rope.len_lines() {
                    self.cursors[idx].line += 1;
                    self.cursors[idx].col = 0;
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
            CursorMove::Up => {
                if self.cursors[idx].line > 0 {
                    self.cursors[idx].line -= 1;
                    let line_len = self.line_visible_len(self.cursors[idx].line);
                    self.cursors[idx].col = min(self.cursors[idx].col, line_len);
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
            CursorMove::Down => {
                if self.cursors[idx].line + 1 < self.rope.len_lines() {
                    self.cursors[idx].line += 1;
                    let line_len = self.line_visible_len(self.cursors[idx].line);
                    self.cursors[idx].col = min(self.cursors[idx].col, line_len);
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
            CursorMove::WordBack => {
                let mut pos = self.cursor_abs_pos(&self.cursors[idx]);
                if pos == 0 {
                    // already at start
                } else {
                    pos -= 1;
                    while pos > 0 && self.rope.char(pos).is_whitespace() {
                        pos -= 1;
                    }
                    while pos > 0 && !self.rope.char(pos - 1).is_whitespace() {
                        pos -= 1;
                    }
                    let line = self.rope.char_to_line(pos);
                    let col = pos - self.rope.line_to_char(line);
                    // ensure col doesn't exceed visible len
                    self.cursors[idx].line = line;
                    self.cursors[idx].col = min(col, self.line_visible_len(line));
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
            CursorMove::WordForward => {
                let total = self.rope.len_chars();
                let mut pos = self.cursor_abs_pos(&self.cursors[idx]);
                if pos < total {
                    if self.rope.char(pos).is_whitespace() {
                        while pos < total && self.rope.char(pos).is_whitespace() {
                            pos += 1;
                        }
                    } else {
                        while pos < total && !self.rope.char(pos).is_whitespace() {
                            pos += 1;
                        }
                        while pos < total && self.rope.char(pos).is_whitespace() {
                            pos += 1;
                        }
                    }
                    let line = self.rope.char_to_line(pos);
                    let col = pos - self.rope.line_to_char(line);
                    self.cursors[idx].line = line;
                    self.cursors[idx].col = min(col, self.line_visible_len(line));
                }
                self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
            }
        }
    }

    pub fn tab(&mut self) {
        for _ in 0..4 {
            self.input(' ');
        }
    }

    fn add_cursor_at(&mut self, line: usize, col: usize) {
        let mut cur = Cursor { line, col };
        cur = Self::clamp_cursor(&self.rope, cur);
        // avoid same cursor twice
        if !self.cursors.contains(&cur) {
            self.cursors.push(cur);
        }
    }

    pub(crate) fn set_block(&mut self, block: Block<'a>) {
        self.block = block;
    }

    fn toggle_gutter(&mut self) {
        self.show_gutter = !self.show_gutter;
    }
}