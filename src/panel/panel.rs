use uuid::Uuid;
use crate::panel::traits::{PanelController, PanelData};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct PanelId(Uuid);

pub struct Panel {
    pub id: PanelId,
    pub title: String,
    pub data: Box<dyn PanelData>,
    pub controller: Box<dyn PanelController>,
}
