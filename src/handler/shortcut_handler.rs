use crate::Mos;

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub name: String,
    pub input: String,
    pub handler: fn(&mut Mos) -> Result<String, String>, // burde sikkert ligge til noe current_panel: Option<String (panel id)>
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

    pub fn register(&mut self, name: String, input: String, handler: fn(&mut Mos) -> Result<String, String>) {
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
}