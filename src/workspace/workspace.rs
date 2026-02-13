use ratatui::Frame;
use crate::app::{Mos, MosId};
use crate::panel::panel::Panel;
use crate::system::panel_registry::PanelRegistry;
use crate::workspace::layout::{FloatingPanel, Layout};

pub struct Workspace {
    layout: Layout,
    floating_panels: Vec<FloatingPanel>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            layout: Layout::Tabs {
                tabs: Vec::new(),
                active: MosId::new(),
            },
            floating_panels: Vec::new(),
        }
    }

    pub fn add_panel(&mut self, panel: Box<dyn Panel>) {
        match &mut self.layout {
            Layout::Tabs { tabs, active } => {
                let panel_id = panel.id();
                tabs.push(panel);
                *active = panel_id; // Set the newly added panel as active
            }
            _ => {
                eprintln!("Currently only Tabs layout is supported for adding panels");
            }
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // chunks?

        let area = frame.size();
        self.layout.render(frame, area);
    }
}