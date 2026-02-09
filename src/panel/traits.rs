use std::any::Any;
use ratatui::{Frame, layout::Rect};
use crate::input::event::InputEvent;

pub trait PanelData: Send + Sync {
    fn serialize(&self) -> serde_json::Value;
    fn deserialize(&mut self, data: serde_json::Value);
    fn as_any(&self) -> &dyn Any;
}

pub trait PanelController {
    fn handle_input(
        &mut self,
        input: &InputEvent,
        data: &mut dyn PanelData,
    );

    fn update(&mut self, data: &mut dyn PanelData);

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        data: &dyn PanelData,
    );
}
