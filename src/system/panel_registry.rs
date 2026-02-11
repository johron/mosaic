use crate::panel::panel::Panel;

pub struct PanelRegistry {
    panels: Vec<Box<dyn Panel>>,
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
        }
    }

    pub fn register_panel(&mut self, panel: Box<dyn Panel>) {
        self.panels.push(panel);
    }

    pub fn get_panels(&self) -> &Vec<Box<dyn Panel>> {
        &self.panels
    }

    pub
}