use crate::app::{Mos, MosId};

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
    Tabs {
        tabs: Vec<MosId>,
        active: MosId,
    },
}