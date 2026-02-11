use ratatui::Frame;
use ratatui::layout::Rect;
use uuid::Uuid;
use crate::event::event::Event;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct PanelId(Uuid);

impl PanelId {
    pub fn new() -> Self {
        PanelId(Uuid::new_v4())
    }
}

pub trait Panel {
    fn id(&self) -> PanelId;
    fn title(&self) -> &str;
    fn handle_event(&mut self, event: Event);
    fn render(&self, frame: &mut Frame, area: Rect);
}