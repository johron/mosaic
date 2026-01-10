use crate::panel::editor::editor_panel::EditorPanel;
use ratatui::layout::{Direction, Rect};
use ratatui::Frame;

#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub id: String,
    pub child: PanelChild,
}

impl Panel {
    pub fn new(id: String, child: PanelChild) -> Self {
        Self {
            id,
            child,
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
                },
                PanelChild::SubHandler(ref mut panel_handler) => {
                    panel_handler.draw(frame, rect);
                },
                _ => {}
            }
        }
    }
}