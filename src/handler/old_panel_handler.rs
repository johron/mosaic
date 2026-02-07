use crate::panel::editor::editor_panel::EditorPanel;
use ratatui::layout::{Direction, Rect};
use ratatui::Frame;

#[derive(Debug, Clone, PartialEq)]
pub struct OldPanel {
    pub id: String,
    pub child: OldPanelChild,
}

impl OldPanel {
    pub fn new(id: String, child: OldPanelChild) -> Self {
        Self {
            id,
            child,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OldPanelChild {
    Editor(EditorPanel),
    SubHandler(OldPanelHandler),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OldPanelHandler {
    pub children: Vec<OldPanel>,
    pub current_panel: Option<String>,
    direction: Direction
}

impl OldPanelHandler {
    pub fn new(direction: Direction) -> Self {
        Self {
            children: Vec::new(),
            current_panel: None,
            direction
        }
    }
    
    pub fn set_current_panel(&mut self, id: Option<String>) {
        self.current_panel = id;
    }
    
    pub fn add_panel(&mut self, panel: OldPanel) {
        self.children.push(panel);
    }

    pub fn remove_panel(&mut self, id: &str) {
        self.children.retain(|p| p.id != id);
    }

    pub fn set_current_panel_relative(&mut self, offset: i32) {
        if self.children.is_empty() {
            return;
        }

        if let Some(current_id) = &self.current_panel {
            if let Some(index) = self.children.iter().position(|p| &p.id == current_id) {
                let new_index = (index as i32 + offset).max(0) as usize;
                let new_index = new_index.min(self.children.len() - 1);
                if new_index < self.children.len() {
                    self.current_panel = Some(self.children[new_index].id.clone());
                }
            }
        } else if !self.children.is_empty() {
            self.current_panel = Some(self.children[0].id.clone());
        }
    }

   pub fn get_current_panel(&mut self) -> Option<&mut OldPanel> {
       if let Some(id) = self.current_panel.clone() {
           self.get_panel(&id)
       } else {
           None
       }
   }
    
    pub fn get_current_editor_panel(&mut self) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_current_panel() {
            if let OldPanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
    }
    pub fn get_panel(&mut self, id: &str) -> Option<&mut OldPanel> {
        self.children.iter_mut().find(|panel| panel.id == id)
    }
    
    pub fn get_editor_panel(&mut self, id: &str) -> Option<&mut EditorPanel> {
        if let Some(panel) = self.get_panel(id) {
            if let OldPanelChild::Editor(ref mut editor_panel) = panel.child {
                return Some(editor_panel);
            }
        }
        None
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
                OldPanelChild::Editor(ref mut editor_panel) => {
                    editor_panel.draw(frame, rect);
                },
                OldPanelChild::SubHandler(ref mut panel_handler) => {
                    panel_handler.draw(frame, rect);
                },
                _ => {}
            }
        }
    }
}