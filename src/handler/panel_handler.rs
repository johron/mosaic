use std::ops::Index;
use ratatui::Frame;
use ratatui::layout::{Direction, Rect};
use crate::handler::panel_handler::Anchor::{BottomLeft, BottomRight, TopLeft, TopRight};
use crate::handler::state_handler::StateHandler;
use crate::panel::editor::editor_panel::EditorPanel;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft
}

impl Anchor {
    pub fn all() -> Vec<Anchor> {
        vec![TopLeft, TopRight, BottomRight, BottomLeft]
    }

    pub fn get_opposite(anchor: &Anchor) -> Anchor {
        match anchor {
            TopLeft => BottomRight,
            TopRight => BottomLeft,
            BottomRight => TopLeft,
            BottomLeft => TopRight,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    anchors: Vec<Anchor>
}

impl Geometry {
    pub fn new(anchors: Vec<Anchor>) -> Self {
        Self {
            anchors
        }
    }

    pub fn is_valid(&self) -> Result<String, String> { // check if it is valid by itself, not together with other elements, replace result with better that can be used to diagnose for program
        if self.anchors == Anchor::all() {
            return Ok(String::from("Valid"));
        }

        if self.anchors.len() == 1 {
            return Ok(String::from("Valid"));
        }

        if self.anchors.len() < 1 {
            return Err(String::from("Invalid, len() <= 0"));
        }

        if self.anchors.len() > 2 {
            return Err(String::from("Invalid, len() > 3"));
        }

        if self.anchors.len() == 2 {
            if self.anchors.contains(&Anchor::get_opposite(&self.anchors[0])) {
                return Err(String::from("Anchors may not be opposites"));
            }

            return Ok(String::from("Valid"));
        }

        Err(String::from("Invalid, unknown reason"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub id: String,
    pub child: PanelChild,
    pub geometry: Geometry
}

impl Panel {
    pub fn new(id: String, child: PanelChild, geometry: Geometry) -> Self {//, geometry: Geometry) -> Self {
        Self {
            id,
            child,
            geometry,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelChild {
    Editor(EditorPanel),
    Explorer(/*ExplorerPanel*/),
    Empty,
    SubHandler(PanelHandler)
}

#[derive(Debug, Clone, PartialEq)]
pub struct PanelHandler {
    pub children: Vec<Panel>,
    current_panel_id: Option<String>,
    direction: Direction
}

impl PanelHandler {
    pub fn new(direction: Direction) -> Self {
        Self {
            children: Vec::new(),
            current_panel_id: None,
            direction
        }
    }
    
    pub fn set_current_panel(&mut self, id: Option<String>) {
        self.current_panel_id = id;
    }
    
    pub fn add_panel(&mut self, panel: Panel) {
        self.children.push(panel);
    }
    
   pub fn get_current_panel(&mut self) -> Option<&mut Panel> {
       if let Some(id) = self.current_panel_id.clone() {
           self.get_panel(&id)
       } else {
           None
       }
   }
    
    pub fn get_current_editor_panel(&mut self) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_current_panel() {
            if let PanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
    }
    pub fn get_panel(&mut self, id: &str) -> Option<&mut Panel> {
        self.children.iter_mut().find(|panel| panel.id == id)
    }
    
    pub fn get_editor_panel(&mut self, id: &str) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_panel(id) {
            if let PanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
    }

    fn round_down(num: u16) -> u16 {
1
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let len = self.children.len() as u16;
        
        for (index, panel) in self.children.iter_mut().enumerate() {
            let index = index as u16;

            let rect = match self.direction {
                Direction::Horizontal => {
                    Rect::new(area.x + area.width/len * index, area.y, area.width/len, area.height)
                },
                Direction::Vertical => {
                    Rect::new(area.x, area.y + area.height/len * index, area.width, area.height/len)
                }
            };

            //println!("{}{:?}", index, rect);

            match panel.child {
                PanelChild::Editor(ref mut editor_panel) => {
                    editor_panel.draw(frame, rect);
                }
                _ => {}
            }
        }
    }
}