use crate::handler::config_handler::ConfigHandler;
use crate::handler::shortcut_handler::ShortcutHandler;
use crate::Mos;
use crate::panel::new_editor::editor::new_editor_panel;

pub fn register_global_shortcuts(shortcut_handler: &mut ShortcutHandler, config_handler: &ConfigHandler) {
    let mos = &config_handler.config.mos;

    shortcut_handler.register(String::from("mos.new_editor"), format!("{}+{}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.new_editor), new_editor);
    shortcut_handler.register(String::from("mos.panel_quit"), format!("{}+{}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_quit), panel_quit);
    shortcut_handler.register(String::from("mos.panel_left"), format!("{}+{}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_left), panel_left);
    shortcut_handler.register(String::from("mos.panel_right"), format!("{}+{}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_right), panel_right);
}

fn new_editor(mos: &mut Mos) -> Result<String, String> {
    let editor = new_editor_panel(mos.config_handler.config.clone());
    let res = mos.panel_handler.add_panel(editor);
    if res.is_err() {
        Err(format!("Error creating new editor: {}", res.err().unwrap()))
    } else {
        let id = mos.panel_handler.panels.last().unwrap().get_id().clone();
        mos.panel_handler.set_active_panel(Some(id));
        Ok(String::from("New editor panel created"))
    }
}

fn panel_quit(mos: &mut Mos) -> Result<String, String> {
    let active_panel = mos.panel_handler.get_active_panel();
    if active_panel.is_none() {
        return Err(String::from("No active panel to quit"));
    }

    mos.panel_handler.remove_panel(active_panel.unwrap().get_id().as_str())?;

    Ok(String::from("Quit command executed"))
}

// panel_left changes active panel to the left of the current active panel, if there is one
fn panel_left(mos: &mut Mos) -> Result<String, String> {
    let active_panel = mos.panel_handler.get_active_panel();
    if active_panel.is_none() {
        return Err(String::from("No active panel to move left from"));
    }

    let active_id = active_panel.unwrap().get_id();
    let panels = &mos.panel_handler.panels;
    let pos = panels.iter().position(|p| p.get_id() == active_id);
    if pos.is_none() {
        return Err(String::from("Active panel not found in panel list"));
    }

    let pos = pos.unwrap();
    if pos == 0 {
        return Err(String::from("Already at leftmost panel"));
    }

    let new_active_id = panels[pos - 1].get_id().clone();
    mos.panel_handler.set_active_panel(Some(new_active_id));

    Ok(String::from("Moved active panel left"))
}

fn panel_right(mos: &mut Mos) -> Result<String, String> {
    let active_panel = mos.panel_handler.get_active_panel();
    if active_panel.is_none() {
        return Err(String::from("No active panel to move right from"));
    }

    let active_id = active_panel.unwrap().get_id();
    let panels = &mos.panel_handler.panels;
    let pos = panels.iter().position(|p| p.get_id() == active_id);
    if pos.is_none() {
        return Err(String::from("Active panel not found in panel list"));
    }

    let pos = pos.unwrap();
    if pos == panels.len() - 1 {
        return Err(String::from("Already at rightmost panel"));
    }

    let new_active_id = panels[pos + 1].get_id().clone();
    mos.panel_handler.set_active_panel(Some(new_active_id));

    Ok(String::from("Moved active panel right"))
}