use crate::panel::panel::PanelId;

pub enum Axis {
    Horizontal,
    Vertical,
}

pub enum Layout {
    Split {
        axis: Axis,
        //ratio: f32,
        children: Vec<Layout>,
    },
    Tabs{
        tabs: Vec<PanelId>,
        active: usize,
    },
}