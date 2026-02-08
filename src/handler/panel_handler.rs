use std::collections::HashMap;
use ratatui::Frame;
use ratatui::layout::{Direction, Rect};
use uuid::Uuid;
use crate::handler::config_handler::{AppConfig};
use crate::panel::new_editor::editor::{Cursor, Mode};

#[derive(Debug, Clone, PartialEq)]
pub enum PanelKind {
    Editor,
    Command,
    Split(Box<Panel>), // panel that can be split into multiple panels
    Plugin(String, String), // plugin name, panel name
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelData { // I really really dont like how this works, i want it modular not like this. In Mos it should be possible to just disable Editor or command or something.
    Editor {
        rope: ropey::Rope,
        top_line: usize,
        cursors: Vec<Cursor>,
        mode: Mode,
    },
    Command {
        content: String,
        result: Option<String>,
        history: Vec<String>,
        history_index: Option<usize>,
    },
    Split {
        panels: Vec<String>, // IDs of child panels, I'll need helper functions to figure out where in the chain this panel id is and stuff. and also for knowing which panel is active and should be colored as so
    },
    Plugin {
        data: HashMap<String, String>, // Når jeg får Lua støtten implementert kan jeg muligens bruke noe LuaValue her, vet ikke?, Add some helper functions for this in lua
    },
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Panel { // continue comment from PanelData, maybe arbitrary struct for paneldata if that's possible
    pub kind: PanelKind,
    pub title: String,
    pub active: bool,
    pub data: PanelData,

    id: String,
    config: AppConfig,

    pub draw: fn(&mut Panel, frame: &mut Frame, area: Rect),
    pub event: fn(&mut Panel, event: &str, args: Vec<String>) -> Result<String, String>,
}

impl Panel {
    pub fn new(
        kind: PanelKind,
        title: String,
        data: PanelData,
        config: AppConfig,
        draw: fn(&mut Panel, frame: &mut Frame, area: Rect),
        event: fn(&mut Panel, event: &str, args: Vec<String>) -> Result<String, String>, // Data operations for the panel, anything that modifies the data goes here
    ) -> Self {
        Self {
            kind,
            title,
            active: false,
            data,
            id: Uuid::new_v4().to_string(),
            config,
            draw,
            event,
        }
    }



    pub fn set_config(&mut self, config: AppConfig) {
        self.config = config;
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct PanelHandler {
    pub panels: Vec<Panel>,
    pub direction: Direction,
    pub active_panel: Option<String>,
}

impl PanelHandler {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            direction: Direction::Horizontal,
            active_panel: None,
        }
    }

    pub fn draw_panels(&mut self, frame: &mut Frame, area: Rect) {
        let len = self.panels.len() as u16;

        for (index, panel) in self.panels.iter_mut().enumerate() {
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

            match panel.kind {
                PanelKind::Editor | PanelKind::Command | PanelKind::Plugin(_, _) => {
                    (panel.draw)(panel, frame, rect);
                },
                PanelKind::Split(_) => {
                    todo!("Implement split panel drawing");
                }
            }
        }
    }

    pub fn add_panel(&mut self, mut panel: Panel) -> Result<(), String> {
        for p in self.panels.iter() {
            if p.id == panel.id {
                return Err(format!("Panel with id {} already exists", panel.id));
            }
        }

        self.panels.push(panel);
        Ok(())
    }

    pub fn remove_panel(&mut self, id: &str) -> Result<(), String> {
        if let Some(pos) = self.panels.iter().position(|p| p.id == id) {
            // if the panel being removed is the active panel, set active_panel to the next panel in the list, or None if there are no more panels
            if let Some(active_id) = &self.active_panel {
                if active_id == id {
                    if self.panels.len() > 1 {
                        let next_pos = if pos == self.panels.len() - 1 { 0 } else { pos + 1 };
                        self.set_active_panel(Some(self.panels[next_pos].id.clone()))
                    } else {
                        self.set_active_panel(None)
                    }
                }
            }

            self.panels.remove(pos);

            Ok(())
        } else {
            Err(format!("Panel with id {} not found", id))
        }
    }

    pub fn set_active_panel(&mut self, id: Option<String>) {
        if let Some(id) = &id {
            if self.panels.iter().any(|p| p.id == *id) {
                self.active_panel = Some(id.clone());
            } else {
                self.active_panel = None;
            }
        } else {
            self.active_panel = None;
        }

        self.iter_set_active();
    }

    pub fn iter_set_active(&mut self) {
        for panel in self.panels.iter_mut() {
            panel.active = Some(panel.id.clone()) == self.active_panel;
        }
    }

    pub fn get_active_panel(&self) -> Option<&Panel> {
        if let Some(id) = &self.active_panel {
            self.panels.iter().find(|p| p.id == *id)
        } else {
            None
        }
    }

    pub fn get_active_panel_mut(&mut self) -> Option<&mut Panel> {
        if let Some(id) = &self.active_panel {
            self.panels.iter_mut().find(|p| p.id == *id)
        } else {
            None
        }
    }
}