use std::any::Any;
use std::path::PathBuf;
use ropey::Rope;
use serde_json::Value;
use crate::panel::traits::PanelData;

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

pub struct TextEditorData {
    pub rope: Rope,
    pub cursors: Vec<Cursor>,
    pub file_path: Option<PathBuf>,
}

impl PanelData for TextEditorData {
    fn serialize(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    fn deserialize(&mut self, data: Value) {
        *self = serde_json::from_value(data).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TextEditorController;