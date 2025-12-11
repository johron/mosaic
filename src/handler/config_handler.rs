use std::io::Write;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub mosaic: MosaicConfig,
    pub editor: EditorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mosaic: MosaicConfig::default(),
            editor: EditorConfig::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MosaicConfig {
    pub auto_save: bool,
    pub save_interval: usize,
}

impl Default for MosaicConfig {
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
    pub enter_normal_mode: String,
}

impl Default for EditorShortcuts {
    fn default() -> Self {
        Self {
            enter_normal_mode: String::from("esc"),
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

    pub move_left: String,
    pub move_right: String,
    pub move_up: String,
    pub move_down: String,
}

impl Default for NormalModeShortcuts {
    fn default() -> Self {
        Self {
            enter_insert_mode: String::from("i"),
            enter_command_mode: String::from("q"),

            move_left: String::from("left | j"),
            move_right: String::from("right | ø"),
            move_up: String::from("up | k"),
            move_down: String::from("down | l"),
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
    pub move_left: String,
    pub move_right: String,
    pub move_up: String,
    pub move_down: String,

    pub skip_word_left: String,
    pub skip_word_right: String,
    pub skip_paragraph_up: String,
    pub skip_paragraph_down: String,

    pub scroll_up: String,
    pub scroll_down: String,
    //pub scroll_left: String,
    //pub scroll_right: String,
}

impl Default for InsertModeShortcuts {
    fn default() -> Self {
        Self {
            move_left: String::from("left"),
            move_right: String::from("right"),
            move_up: String::from("up"),
            move_down: String::from("down"),

            skip_word_left: String::from("ctrl+left | ctrl+j"),
            skip_word_right: String::from("ctrl+right | ctrl+ø"),
            skip_paragraph_up: String::from("ctrl+up | ctrl+k"),
            skip_paragraph_down: String::from("ctrl+down | ctrl+l"),

            scroll_up: String::from("ctrl+k | scroll_up"),
            scroll_down: String::from("ctrl+l | scroll_down"),
            //scroll_left: String::from("ctrl+h"),
            //scroll_right: String::from("ctrl+ø"),
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
            move_left: String::from("left"),
            move_right: String::from("right"),

            skip_word_left: String::from("ctrl+left | ctrl+j"),
            skip_word_right: String::from("ctrl+right | ctrl+ø"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigHandler {
    pub config: Config,
}

impl ConfigHandler {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn load_config(&mut self) {
        let path = std::path::Path::new("./config/mosaic_config.toml");

        // ensure config directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Failed to create config directory `{}`: {}", parent.display(), e);
                    return;
                }
            }
        }

        // helper to write the default config to disk
        fn write_default(path: &std::path::Path, cfg: &Config) -> Result<(), std::io::Error> {
            let toml_str = toml::to_string_pretty(cfg).unwrap_or_else(|e| {
                eprintln!("Failed to serialize default config: {}", e);
                String::new()
            });
            let mut file = std::fs::File::create(path)?;
            file.write_all(toml_str.as_bytes())?;
            Ok(())
        }

        if path.exists() {
            match std::fs::read_to_string(path) {
                Ok(contents) => match toml::from_str::<Config>(&contents) {
                    Ok(parsed) => {
                        self.config = parsed;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse config `{}`: {}. Rewriting default config.", path.display(), e);
                        if let Err(e) = write_default(path, &self.config) {
                            eprintln!("Failed to write default config to `{}`: {}", path.display(), e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read config `{}`: {}. Rewriting default config.", path.display(), e);
                    if let Err(e) = write_default(path, &self.config) {
                        eprintln!("Failed to write default config to `{}`: {}", path.display(), e);
                    }
                }
            }
        } else {
            // file doesn't exist: write default config
            if let Err(e) = write_default(path, &self.config) {
                eprintln!("Failed to create config `{}`: {}", path.display(), e);
            }
        }
    }

    pub fn reload(&mut self) {
        self.load_config();
    }
}