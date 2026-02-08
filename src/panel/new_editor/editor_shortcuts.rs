use crate::handler::config_handler::ConfigHandler;
use crate::handler::shortcut_handler::ShortcutHandler;
use crate::panel::editor::editor_logic::CursorDirection;
use crate::{Mos};
use crate::panel::new_editor::editor::Mode;

pub fn register_editor_shortcuts(shortcut_handler: &mut ShortcutHandler, config_handler: &ConfigHandler) {
    let editor = &config_handler.config.editor;
    let mos = &config_handler.config.mos;

    // Editor
    shortcut_handler.register(String::from("editor.enter_normal_mode"), mos.shortcuts.mos_key.clone(), enter_normal_mode);
    shortcut_handler.register(String::from("editor.clear_cursors"), editor.shortcuts.clear_cursors.clone(), clear_cursors);

    // Normal
    shortcut_handler.register(String::from("mode.normal.enter_insert_mode"), editor.normal_mode.shortcuts.enter_insert_mode.clone(), enter_insert_mode);
    shortcut_handler.register(String::from("mode.normal.enter_command_mode"), editor.normal_mode.shortcuts.enter_command_mode.clone(), enter_command_mode);

    shortcut_handler.register(String::from("mode.normal.cursor_left"), editor.normal_mode.shortcuts.cursor_left.clone(), cursor_left);
    shortcut_handler.register(String::from("mode.normal.cursor_up"), editor.normal_mode.shortcuts.cursor_up.clone(), cursor_up);
    shortcut_handler.register(String::from("mode.normal.cursor_down"), editor.normal_mode.shortcuts.cursor_down.clone(), cursor_down);
    shortcut_handler.register(String::from("mode.normal.cursor_right"), editor.normal_mode.shortcuts.cursor_right.clone(), cursor_right);

    // Command

    // Insert
    shortcut_handler.register(String::from("mode.insert.newline"), editor.insert_mode.shortcuts.newline.clone(), newline);
    shortcut_handler.register(String::from("mode.insert.backspace"), editor.insert_mode.shortcuts.backspace.clone(), backspace);
    shortcut_handler.register(String::from("mode.insert.tab"), editor.insert_mode.shortcuts.tab.clone(), tab);
    shortcut_handler.register(String::from("mode.insert.reverse_tab"), editor.insert_mode.shortcuts.reverse_tab.clone(), reverse_tab);

    shortcut_handler.register(String::from("mode.insert.cursor_left"), editor.insert_mode.shortcuts.cursor_left.clone(), cursor_left);
    shortcut_handler.register(String::from("mode.insert.cursor_up"), editor.insert_mode.shortcuts.cursor_up.clone(), cursor_up);
    shortcut_handler.register(String::from("mode.insert.cursor_down"), editor.insert_mode.shortcuts.cursor_down.clone(), cursor_down);
    shortcut_handler.register(String::from("mode.insert.cursor_right"), editor.insert_mode.shortcuts.cursor_right.clone(), cursor_right);
    shortcut_handler.register(String::from("mode.insert.skip_word_left"), editor.insert_mode.shortcuts.skip_word_left.clone(), skip_word_left);
    shortcut_handler.register(String::from("mode.insert.skip_word_right"), editor.insert_mode.shortcuts.skip_word_right.clone(), skip_word_right);

    shortcut_handler.register(String::from("mode.insert.add_cursor_below"), editor.insert_mode.shortcuts.add_cursor_below.clone(), add_cursor_below);
    shortcut_handler.register(String::from("mode.insert.add_cursor_above"), editor.insert_mode.shortcuts.add_cursor_above.clone(), add_cursor_above);
}

fn enter_mode(mos: &mut Mos, mode: Mode) -> Result<String, String> {
    let active_panel = mos.panel_handler.get_active_panel_mut();
    if active_panel.is_none() {
        return Err(String::from("No active editor"))
    }
    
    let active_panel = active_panel.unwrap();
    
    let res = (active_panel.event)(active_panel, "set_mode", vec![mode.to_string()]);
    if res.is_err() {
        return Err(format!("Failed to enter {} mode: {}", mode.to_string(), res.err().unwrap()));
    }
    
    Ok(format!("Entered {} mode", mode.to_string()))
}

fn enter_normal_mode(mos: &mut Mos) -> Result<String, String> {
    enter_mode(mos, Mode::Normal)
}

fn enter_insert_mode(mos: &mut Mos) -> Result<String, String> {
    enter_mode(mos, Mode::Insert)
}

fn enter_command_mode(mos: &mut Mos) -> Result<String, String> {
    enter_mode(mos, Mode::Command)
}

fn newline(mos: &mut Mos) -> Result<String, String> {
    if mos.panel_handler.get_current_editor_panel().is_none() {
        return Err(String::from("No active editor"))
    }

    let editor = &mut mos.panel_handler.get_current_editor_panel().unwrap().editor;

    let current_top_line = editor.rope.get_line(editor.cursors[0].line).unwrap().to_string();
    let mut preceding_whitespace = String::new();
    for c in current_top_line.chars() {
        if !c.is_whitespace() {
            break
        }
        if c == '\n' {
            break
        }
        preceding_whitespace.push(c);
    }
    editor.input('\n');
    for c in preceding_whitespace.chars() {
        editor.input(c);
    }

    Ok(String::from("Newline"))
}

fn backspace(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.backspace();
    Ok(String::from("Backspace"))
}

fn tab(mos: &mut Mos) -> Result<String, String> {
    let tab_size = mos.config_handler.config.editor.tab_size;
    for _ in 0..tab_size {
        mos.panel_handler.get_current_editor_panel().unwrap().editor.input(' ');
    }
    Ok(String::from("Tab"))
}

fn reverse_tab(mos: &mut Mos) -> Result<String, String> { // TODO: Make sure that it is the preceding spaces that are actually remove
    if mos.panel_handler.get_current_editor_panel().is_none() {
        return Err(String::from("No active editor"))
    }

    let editor = &mut mos.panel_handler.get_current_editor_panel().unwrap().editor;
    let tab_size = mos.config_handler.config.editor.tab_size;

    let current_top_line = editor.rope.get_line(editor.cursors[0].line).unwrap().to_string();
    let mut preceding_whitespace = String::new();
    for c in current_top_line.chars() {
        if !c.is_whitespace() {
            break;
        }
        preceding_whitespace.push(c);
    }

    let mut to_remove = tab_size;
    if preceding_whitespace.len() < tab_size {
        to_remove = preceding_whitespace.len();
    }

    for _ in 0..to_remove {
        editor.backspace();
    }

    Ok(String::from("Reverse tab"))
}

fn cursor_left(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::Left);
    Ok(String::from("Move left"))
}

fn cursor_up(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::Up);
    Ok(String::from("Move up"))
}

fn cursor_down(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::Down);
    Ok(String::from("Move down"))
}

fn cursor_right(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::Right);
    Ok(String::from("Move right"))
}

fn skip_word_left(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::WordLeft);
    Ok(String::from("Skip word left"))
}

fn skip_word_right(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.move_cursor(CursorDirection::WordRight);
    Ok(String::from("Skip word right"))
}

fn add_cursor_below(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.add_cursor_below();
    Ok(String::from("Added cursor below"))
}

fn add_cursor_above(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.add_cursor_above();
    Ok(String::from("Added cursor above"))
}

fn clear_cursors(mos: &mut Mos) -> Result<String, String> {
    mos.panel_handler.get_current_editor_panel().unwrap().editor.clear_cursors();
    Ok(String::from("Cleared extra cursors"))
}