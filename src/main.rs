mod handler;
mod panel;
mod global;

use crate::handler::command_handler::CommandHandler;
use crate::handler::config_handler::ConfigHandler;
use crate::handler::input_handler::InputHandler;
use crate::handler::shortcut_handler::ShortcutHandler;
use crate::handler::state_handler::{State, StateHandler};
use crate::panel::command::command_panel::FloatingCommandPanel;

#[cfg(not(windows))]
use crossterm::{
    event::KeyboardEnhancementFlags,
    event::PopKeyboardEnhancementFlags,
    event::PushKeyboardEnhancementFlags
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use std::fmt::Display;
use std::io::StdoutLock;
use std::ops::AddAssign;
use std::time::{Duration, Instant};
use std::{env, fmt, fs, io};
use crate::handler::panel_handler::PanelHandler;
use crate::panel::new_editor::editor::new_editor_panel;
use global::shortcuts::register_global_shortcuts;
use crate::global::commands::register_global_commands;
use crate::panel::new_editor::editor_shortcuts;

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
        self.result = None;
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
struct Mos{
    toast: Option<Toast>,

    //panel_handler: OldPanelHandler,
    panel_handler: PanelHandler,
    state_handler: StateHandler,
    config_handler: ConfigHandler,
    command_handler: CommandHandler,
    shortcut_handler: ShortcutHandler,
}

impl Mos {
    fn new() -> Self {
        Self {
            toast: None,

            //panel_handler: OldPanelHandler::new(Direction::Horizontal),
            panel_handler: PanelHandler::new(),
            state_handler: StateHandler::new(),
            config_handler: ConfigHandler::new(),
            command_handler: CommandHandler::new(),
            shortcut_handler: ShortcutHandler::new(),

        }
    }

    //fn open_in_current_editor(&mut self, file_path: &str) {
    //    if let Some(editor_panel) = self.panel_handler.get_current_editor_panel() {
    //        editor_panel.editor.open_file(file_path);
    //    }
    //}

    // burde heller være noe slik: fn open_file() som da lager en ny editor for å åpne filen,
    // også en funksjon som bare lager en ny tom editor, hvis ingen fil skal åpnes

    fn new_empty_editor(&mut self) {
        let editor = new_editor_panel(self.config_handler.config.clone());
        let res = self.panel_handler.add_panel(editor);
        if res.is_err() {
            self.show_toast(&format!("Error creating new editor: {}", res.err().unwrap()), Duration::from_secs(3));
        } else {
            let id = self.panel_handler.panels.last().unwrap().get_id().clone();
            self.panel_handler.set_active_panel(Some(id));
        }
    }

    fn show_toast(&mut self, message: &str, duration: Duration) {
        println!("{}", message);

        //let toast = Toast {
        //    message: message.to_string(),
        //    start_time: Instant::now(),
        //    duration,
        //};
//
        //self.toast = Some(toast);
    }

    fn init(&mut self) {
        self.new_empty_editor();

        //self.panel_handler.add_panel(
        //    OldPanel::new(String::from("editor_1"), OldPanelChild::Editor(EditorPanel::new()))
        //);
        //self.panel_handler.set_current_panel(Some(String::from("editor_1")));


        self.config_handler.load_config_safe();
        // Set state and editor config based on config ^

        register_global_commands(&mut self.command_handler);

        //self.editor.register_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);
    }
    
    fn register_shortcuts(&mut self) {
        register_global_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);
        editor_shortcuts::register_editor_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);
    }
    
    fn reload(&mut self) {
        self.config_handler.load_config_safe();
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;

    #[cfg(not(windows))] // keyboard enhancements don't work on windows
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture, PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    ))?;

    #[cfg(windows)]
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

    let mut mos = Mos::new();
    mos.init();
    //if let Some(path) = file_path.as_ref() {
    //    mos.open_in_current_editor(path);
    //}

    let res = run(&mut terminal, mos);

    disable_raw_mode()?;

    #[cfg(not(windows))]
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        PopKeyboardEnhancementFlags // dette ser ikke ut som at det fungerer.. kan ikke lukke nano etter å ha kjørt mos
    )?;

    #[cfg(windows)]
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<StdoutLock>>, mut mos: Mos) -> io::Result<()> {
    let mut input_handler = InputHandler::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            mos.panel_handler.draw_panels(frame, area);

            if mos.state_handler.state == State::Command || mos.state_handler.state == State::Panel {
                let height = 2;
                FloatingCommandPanel::draw(frame, Rect::new(0, area.height - height, area.width, height), &mos.state_handler);
            }
        })?;

        //if mos.toast.is_some() {
        //    let toast = mos.toast.as_ref().unwrap();
        //    if toast.start_time.elapsed() >= toast.duration {
        //        mos.toast = None;
        //    }
        //}

        input_handler.handle(&mut mos).expect("TODO: panic message");

        if mos.state_handler.should_quit {
            break Ok(());
        }
    }
}