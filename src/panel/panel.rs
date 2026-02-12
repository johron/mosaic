use ratatui::Frame;
use ratatui::layout::Rect;
use uuid::Uuid;
use crate::app::MosId;
use crate::event::event::Event;

pub type PanelCtor = fn() -> Box<dyn Panel>;

pub trait Panel {
    fn id(&self) -> MosId;
    fn title(&self) -> &str;
    fn handle_event(&mut self, event: Event);
    fn render(&self, frame: &mut Frame, area: Rect);
}