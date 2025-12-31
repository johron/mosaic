use crate::Mosaic;

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub name: String,
    pub input: String,
    pub handler: fn(&mut Mosaic) -> Result<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ShortcutHandler {
    shortcuts: Vec<Shortcut>,
}

impl ShortcutHandler {
    pub fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
        }
    }

    pub fn register_shortcut(&mut self, name: String, input: String, handler: fn(&mut Mosaic) -> Result<String, String>) {
        let shortcut = Shortcut {
            name,
            input,
            handler,
        };

        self.shortcuts.push(shortcut);
    }

    pub fn get_shortcuts(&self) -> &Vec<Shortcut> {
        &self.shortcuts
    }

    pub fn handle_shortcut(&self, input: &str, mosaic: &mut Mosaic) -> Option<Result<String, String>> {
        for shortcut in &self.shortcuts {
            if shortcut.input == input {
                return Some((shortcut.handler)(mosaic));
            }
        }
        None
    }
}