use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::MosId;
use crate::workspace::layout::Layout;

pub struct Workspace {
    layout: Layout,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            layout: Layout::Tabs {
                tabs: Vec::new(),
                active: MosId::new(),
            },
        }
    }
}