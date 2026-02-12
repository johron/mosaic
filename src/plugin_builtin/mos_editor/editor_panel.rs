use std::path::PathBuf;
use ratatui::Frame;
use ratatui::layout::Rect;
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
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            cursors: vec![Cursor::new(0, 0, 0)],
            file_path: None,
        }
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
        frame.render_widget(ratatui::widgets::Paragraph::new("self.rope.to_string()"), area);
    }
}