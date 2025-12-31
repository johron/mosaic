//mod ui;
mod input;
mod editor;
mod handler;
mod panel;

use crate::handler::command_handler::CommandHandler;
use crate::handler::config_handler::ConfigHandler;
use crate::handler::panel_handler::{Panel, PanelChild, PanelHandler};
use crate::handler::shortcut_handler::ShortcutHandler;
use crate::handler::state_handler::StateHandler;
use crate::panel::editor_panel::EditorPanel;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;
use std::fmt::Display;
use std::io::{BufRead, StdoutLock};
use std::ops::AddAssign;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{env, fmt, fs, io};

#[derive(Debug, Copy, Clone)]
enum Mode {
    Normal,
    Insert,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Command => write!(f, "COMMAND"),
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    content: String,
    result: Option<String>,
}

impl Command {
    fn new() -> Self {
        Self {
            content: String::new(),
            result: None,
        }
    }

    fn clear(&mut self) {
        self.content.clear();
    }

    fn pop(&mut self) {
        self.content.pop();
    }
}

impl AddAssign<&str> for Command {
    fn add_assign(&mut self, rhs: &str) {
        self.content.push_str(rhs);
    }
}

#[derive(Debug, Clone)]
struct Toast {
    message: String,
    start_time: Instant,
    duration: Duration,
}

#[derive(Debug, Clone)]
struct Mosaic{
    toast: Option<Toast>,

    panel_handler: PanelHandler,
    state_handler: StateHandler,
    config_handler: ConfigHandler,
    command_handler: CommandHandler,
    shortcut_handler: ShortcutHandler,
}

impl Mosaic {
    fn new() -> Self {
        Self {
            toast: None,

            panel_handler: PanelHandler::new(),
            state_handler: StateHandler::new(),
            config_handler: ConfigHandler::new(),
            command_handler: CommandHandler::new(),
            shortcut_handler: ShortcutHandler::new(),
        }
    }

    fn open_in_current_editor(&mut self, file_path: &str) {
        if let Some(editor_panel) = self.panel_handler.get_current_editor_panel() {
            editor_panel.editor.open_file(file_path);
        }
    }

    fn show_toast(&mut self, message: &str, duration: Duration) {
        let toast = Toast {
            message: message.to_string(),
            start_time: Instant::now(),
            duration,
        };

        self.toast = Some(toast);
    }

    fn init(&mut self) {
        self.panel_handler.add_panel(
            Panel::new(String::from("editor_1"), PanelChild::Editor(EditorPanel::new()))
        );
        self.panel_handler.set_current_panel(Some(String::from("editor_1")));

        self.config_handler.load_config();
        // Set state and editor config based on config ^

        self.register_commands();

        //self.editor.register_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);
    }

    fn register_commands(&mut self) {
        //self.editor.register_commands(&mut self.command_handler, &mut self.config_handler);

       self.command_handler.register(String::from("q"), "@", |mosaic, _args| {
            mosaic.state_handler.should_quit = true;
           Ok(String::from("Quit command executed"))
       });
    }
    
    fn reload(&mut self) {
        self.config_handler.load_config();
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut file_path: Option<String> = None;
    let mut initial_content = String::new();
    
    if let Some(arg1) = env::args().nth(1) {
        file_path = Some(arg1.clone());
        match fs::read_to_string(&arg1) {
            Ok(content) => {
                initial_content = content;
            }
            Err(_) => {
                initial_content = String::new();
            }
        }
    }

    let mut mosaic = Mosaic::new();
    mosaic.init();

    if let Some(path) = file_path.as_ref() {
        mosaic.open_in_current_editor(path);
    }

    let res = run(&mut terminal, mosaic);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<StdoutLock>>, mut mosaic: Mosaic) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            //ui::draw(frame, &mut mosaic); // pass immutable state

            for panel in &mut mosaic.panel_handler.panels {
                match &mut panel.child {
                    PanelChild::Editor(editor_panel) => {
                        editor_panel.draw(frame, &mut mosaic.state_handler);
                    }
                    _ => {
                    }
                }
            }
        })?;

        //if mosaic.toast.is_some() {
        //    let toast = mosaic.toast.as_ref().unwrap();
        //    if toast.start_time.elapsed() >= toast.duration {
        //        mosaic.toast = None;
        //    }
        //}

        input::handle(&mut mosaic).expect("TODO: panic message");

        if mosaic.state_handler.should_quit {
            break Ok(());
        }
    }
}