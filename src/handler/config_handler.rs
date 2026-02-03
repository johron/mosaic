use std::fs;
use std::path::PathBuf;
use config::{Config, File};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AppConfig {
    pub mos: MosConfig,
    pub editor: EditorConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mos: MosConfig::default(),
            editor: EditorConfig::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MosConfig {
    pub auto_save: bool,
    pub save_interval: usize,
}

impl Default for MosConfig {
    fn default() -> Self {
        Self {
            auto_save: true,
            save_interval: 1000, // in milliseconds (only if there are changes)
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EditorConfig {
    pub show_gutter: bool,
    pub tab_size: usize,

    pub shortcuts: EditorShortcuts,

    pub normal_mode: NormalModeConfig,
    pub insert_mode: InsertModeConfig,
    pub command_mode: CommandModeConfig,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            show_gutter: true,
            tab_size: 4,

            shortcuts: EditorShortcuts::default(),

            normal_mode: NormalModeConfig::default(),
            insert_mode: InsertModeConfig::default(),
            command_mode: CommandModeConfig::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct EditorShortcuts {
    pub mos_key: String,
    pub clear_cursors: String,
}

impl Default for EditorShortcuts {
    fn default() -> Self {
        Self {
            mos_key: String::from("f12"),
            clear_cursors: String::from("esc"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct NormalModeConfig {
    pub highlight_current_line: bool,
    pub shortcuts: NormalModeShortcuts,
}

impl Default for NormalModeConfig {
    fn default() -> Self {
        Self {
            highlight_current_line: true,
            shortcuts: NormalModeShortcuts::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct NormalModeShortcuts {
    pub enter_insert_mode: String,
    pub enter_command_mode: String,

    pub cursor_left: String,
    pub cursor_right: String,
    pub cursor_up: String,
    pub cursor_down: String,
}

impl Default for NormalModeShortcuts {
    fn default() -> Self {
        Self {
            enter_insert_mode: String::from("i"),
            enter_command_mode: String::from("q"),

            cursor_left: String::from("left | j"),
            cursor_right: String::from("right | ø"),
            cursor_up: String::from("up | k"),
            cursor_down: String::from("down | l"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct InsertModeConfig {
    pub shortcuts: InsertModeShortcuts,
}

impl Default for InsertModeConfig {
    fn default() -> Self {
        Self {
            shortcuts: InsertModeShortcuts::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct InsertModeShortcuts {
    pub cursor_left: String,
    pub cursor_right: String,
    pub cursor_up: String,
    pub cursor_down: String,

    pub add_cursor_above: String,
    pub add_cursor_below: String,

    pub skip_word_left: String,
    pub skip_word_right: String,
    pub skip_paragraph_up: String,
    pub skip_paragraph_down: String,

    pub scroll_up: String,
    pub scroll_down: String,
    //pub scroll_left: String,
    //pub scroll_right: String,
    
    pub newline: String,
    pub backspace: String,
    pub tab: String,
    pub reverse_tab: String,
}

impl Default for InsertModeShortcuts {
    fn default() -> Self {
        Self {
            //enter_normal_mode: String::from("f12"),
            
            cursor_left: String::from("left"),
            cursor_right: String::from("right"),
            cursor_up: String::from("up"),
            cursor_down: String::from("down"),

            add_cursor_above: String::from("control+alt+up"),
            add_cursor_below: String::from("control+alt+down"),

            skip_word_left: String::from("control+left | control+j"),
            skip_word_right: String::from("control+right | control+ø"),
            skip_paragraph_up: String::from("control+up | control+k"),
            skip_paragraph_down: String::from("control+down | control+l"),

            scroll_up: String::from("control+k | scroll_up"),
            scroll_down: String::from("control+l | scroll_down"),
            //scroll_left: String::from("control+h"),
            //scroll_right: String::from("control+ø"),
            
            newline: String::from("enter"),
            backspace: String::from("backspace"),
            tab: String::from("tab"),
            reverse_tab: String::from("shift+tab")
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CommandModeConfig {
    pub shortcuts: CommandModeShortcuts,
}

impl Default for CommandModeConfig {
    fn default() -> Self {
        Self {
            shortcuts: CommandModeShortcuts::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CommandModeShortcuts {
    pub move_left: String,
    pub move_right: String,

    pub skip_word_left: String,
    pub skip_word_right: String,
}

impl CommandModeShortcuts {
    pub fn default() -> Self {
        Self {
            //enter_normal_mode: String::from("f12"),
            
            move_left: String::from("left"),
            move_right: String::from("right"),

            skip_word_left: String::from("control+left | control+j"),
            skip_word_right: String::from("control+right | control+ø"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigHandler {
    pub config: AppConfig,
}

impl ConfigHandler {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    fn config_path(&mut self) -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("mos");
        path.push("config.toml");
        path
    }

    fn backup_path(&mut self) -> PathBuf {
        let mut path = self.config_path();
        path.set_extension("last_ok.toml");
        path
    }

    pub fn load_config_safe(&mut self) {
        let path = self.config_path();
        let backup = self.backup_path();

        if !path.exists() {
            self.write_default_config(&path);
            self.config = AppConfig::default();
        }

        match self.try_load(&path) {
            Ok(cfg) => {
                if let Ok(toml) = toml::to_string_pretty(&cfg) {
                    let _ = fs::write(&backup, toml);
                }
                self.config = cfg
            }

            Err(err) => {
                eprintln!("Config error: {err}");

                if backup.exists() {
                    if let Ok(cfg) = self.try_load(&backup) {
                        eprintln!("Using last known good config");
                        self.config = cfg;
                    }
                }

                eprintln!("Falling back to defaults");
                self.config = AppConfig::default()
            }
        }
    }

    fn try_load(&mut self, path: &PathBuf) -> Result<AppConfig, config::ConfigError> {
        Config::builder()
            .add_source(File::from(path.as_path()))
            .build()?
            .try_deserialize()
    }


    fn write_default_config(&mut self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let default = AppConfig::default();

        if let Ok(toml) = toml::to_string_pretty(&default) {
            let _ = fs::write(path, toml);
        }
    }
}