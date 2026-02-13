use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Rect};
use crate::app::{Mos, MosId};
use crate::panel::panel::Panel;
use crate::system::panel_registry::PanelRegistry;

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
        tabs: Vec<Box<dyn Panel>>,
        active: MosId,
    },
}

impl Layout {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self {
            Layout::Split { axis, children } => {
                let constraints = vec![Constraint::Percentage(100 / children.len() as u16); children.len()];
                let chunks = match axis {
                    Axis::Horizontal => ratatui::layout::Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(constraints)
                        .split(area),
                    Axis::Vertical => ratatui::layout::Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(constraints)
                        .split(area),
                };
                for (child, chunk) in children.iter().zip(chunks.iter()) {
                    child.render(frame, *chunk);
                }
            }
            Layout::Tabs { tabs, active } => {
                //println!("Rendering Tabs layout with {} tabs, active tab id: {:?}", tabs.len(), active);
                if let Some(active_panel) = tabs.iter().find(|panel| panel.id() == *active) {
                    active_panel.render(frame, area);
                } else if !tabs.is_empty() {
                    // If active panel is not found, render the first tab as fallback
                    tabs[0].render(frame, area);
                } else {
                    // No tabs to render, maybe render a placeholder or do nothing
                    println!("No tabs to render in Tabs layout");
                }
            }
        }
    }
}

pub enum Anchor {
    Top(Offset),
    Bottom(Offset),
    Left(Offset),
    Right(Offset),
    TopLeft(Offset),
    TopRight(Offset),
    BottomLeft(Offset),
    BottomRight(Offset),
}

pub enum Offset {
    Absolute(i32, i32, i32, i32), // left, top, right, bottom
    Relative(f32, f32, f32, f32), // left, top, right, bottom as percentage of parent size
}

pub struct FloatingPanel {
    id: MosId,
    anchor: Anchor,
}