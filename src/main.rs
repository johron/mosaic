mod ui;
mod input;
mod command;

use std::{env, fmt, fs, io};
use std::fmt::{format, Display};
use std::io::{BufRead, StdoutLock};
use std::ops::AddAssign;
use std::str::FromStr;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{DefaultTerminal, Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Padding};
use tui_textarea::{Input, TextArea};

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
    text_area: TextArea<'a>,
    command: Command,
}

impl<'a> Mosaic<'a> {
    fn new(mode: Mode, text_area: TextArea<'a>) -> Self {
        Self {
            mode,
            should_quit: false,
            text_area,
            command: Command::new(),
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

    let mut text_area = if let Some(path_str) = env::args().nth(1) {
        let path = std::path::Path::new(&path_str);
        if path.exists() {
            let file = fs::File::open(path)?;
            io::BufReader::new(file)
                .lines()
                .collect::<io::Result<_>>()?
        } else {
            TextArea::default()
        }
    } else {
        TextArea::default()
    };

    text_area.set_line_number_style(Style::default().fg(Color::DarkGray));

    let mosaic = Mosaic::new(Mode::Normal, text_area);

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