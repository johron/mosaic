mod handler;
mod panel;
mod input;

use crate::handler::command_handler::CommandHandler;
use crate::handler::config_handler::ConfigHandler;
use crate::handler::input_handler::InputHandler;
use crate::handler::shortcut_handler::ShortcutHandler;
use crate::handler::state_handler::StateHandler;
use crate::panel::command::command_panel::FloatingCommandPanel;

#[cfg(not(windows))]
use crossterm::{
    event::PushKeyboardEnhancementFlags,
    event::KeyboardEnhancementFlags,
    event::PopKeyboardEnhancementFlags
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Direction, Rect};
use ratatui::Terminal;
use std::fmt::Display;
use std::io::StdoutLock;
use std::ops::AddAssign;
use std::time::{Duration, Instant};
use std::{env, fmt, fs, io};
use crate::handler::panel_handler::PanelHandler;
use crate::panel::new_editor::editor::new_editor_panel;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Command,
    // Terminal
    // Select
    // Search & Replace
    // Explorer/Files
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal  => write!(f, "NORMAL"),
            Self::Insert  => write!(f, "INSERT"),
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

        self.register_commands();
        self.register_shortcuts();

        //self.editor.register_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);
    }

    fn register_commands(&mut self) {
        //self.editor.register_commands(&mut self.command_handler, &mut self.config_handler);

       self.command_handler.register(String::from("q"), "@", |mos, _args| {
           mos.state_handler.should_quit = true;
           Ok(String::from("Quit command executed"))
       });

        //self.command_handler.register(String::from("len"), "@", |mos, _args| {
        //    let mut editor = &mut mos.panel_handler.get_current_editor_panel().unwrap().editor;
        //    editor.input_str(format!("{}", editor.cursors.len()));
        //    Ok(String::from("Len command executed"))
        //});
    }

    fn register_shortcuts(&mut self) {
        //editor_shortcuts::register_shortcuts(&mut self.shortcut_handler, &mut self.config_handler);

        let mos = &self.config_handler.config.mos;

        // Register mos shortcuts with mos_key_as_mod as a prefix
        // Note: mos_key and mos_key_as_mod themselves are handled directly in input_handler
        // These shortcuts are for other mos-specific operations

        //let new_editor_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.new_editor);
        //self.shortcut_handler.register(String::from("mos.new_editor"), new_editor_shortcut, |mos| {
        //    let id = format!("editor_{}", mos.panel_handler.children.len() + 1);
        //    mos.panel_handler.add_panel(OldPanel::new(id.clone(), OldPanelChild::Editor(EditorPanel::new())));
        //    mos.panel_handler.set_current_panel(Some(id));
        //    Ok(String::from("New editor opened"))
        //});
//
        //let panel_quit_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_quit);
        //self.shortcut_handler.register(String::from("mos.panel_quit"), panel_quit_shortcut, |mos| {
        //    if let Some(current_id) = &mos.panel_handler.current_panel {
        //        let id = current_id.clone();
        //        mos.panel_handler.remove_panel(&id);
        //        mos.panel_handler.set_current_panel(mos.panel_handler.children.last().map(|p| p.id.clone()));
        //        Ok(String::from("Current panel closed"))
        //    } else {
        //        Err(String::from("No panel to close"))
        //    }
        //});
//
        //let panel_left_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_left);
        //self.shortcut_handler.register(String::from("mos.panel_left"), panel_left_shortcut, |mos| {
        //    mos.panel_handler.set_current_panel_relative(-1);
        //    Ok(String::from("Moved to left panel"))
        //});
//
        //let panel_right_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_right);
        //self.shortcut_handler.register(String::from("mos.panel_right"), panel_right_shortcut, |mos| {
        //    mos.panel_handler.set_current_panel_relative(1);
        //    Ok(String::from("Moved to right panel"))
        //});
//
        //let panel_up_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_up);
        //self.shortcut_handler.register(String::from("mos.panel_up"), panel_up_shortcut, |_mos| {
        //    Ok(String::from("Moved to upper panel"))
        //});
//
        //let panel_down_shortcut = format!("{} + {}", mos.shortcuts.mos_key_as_mod, mos.shortcuts.panel_down);
        //self.shortcut_handler.register(String::from("mos.panel_down"), panel_down_shortcut, |_mos| {
        //    Ok(String::from("Moved to lower panel"))
        //});
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
    mos.state_handler.mode = Mode::Normal;
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

            if mos.state_handler.mode == Mode::Command || mos.state_handler.mode == Mode::Normal {
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