mod ui;
mod input;
mod editor;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style, Stylize};
use ratatui::Terminal;
use std::fmt::Display;
use std::io::{BufRead, StdoutLock};
use std::ops::AddAssign;
use std::str::FromStr;
use std::{env, fmt, fs, io};
use crate::editor::Editor;

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

#[derive(Debug)]
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

#[derive(Debug)]
struct Mosaic<'a> {
    mode: Mode,
    should_quit: bool,
    command: Command,
    editors: Vec<Editor<'a>>,
    current_editor: usize,
}

impl<'a> Mosaic<'a> {
    fn new(mode: Mode, editor: Editor<'a>) -> Self {
        Self {
            mode,
            should_quit: false,
            command: Command::new(),
            editors: vec![editor],
            current_editor: 0,
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.command.clear();
    }

    fn quit(&mut self) {
        self.should_quit = true;
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

    //text_area.set_line_number_style(Style::default().fg(Color::DarkGray));
    //text_area.set_tab_length(4);

    let mosaic = Mosaic::new(Mode::Normal, Editor::new(initial_content.as_str(), file_path));

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
            ui::draw(frame, &mut mosaic); // pass immutable state
        })?;

        input::handle(&mut mosaic).expect("TODO: panic message");
        
        if mosaic.should_quit {
            break Ok(());
        }
    }
}