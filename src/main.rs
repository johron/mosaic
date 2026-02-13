mod app;
mod panel;
mod workspace;
mod floating_panel;
mod plugin_builtin;
mod event;
mod plugin;
mod system;

use std::io::stdout;
use std::time::Duration;
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::Mos;

fn main() -> Result<(), String> {
    if crossterm::terminal::enable_raw_mode().is_err() {
        return Err("Failed to enable raw mode".to_string());
    }
    
    // Enter the alternate screen and enable mouse capture so only our UI is visible.
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).map_err(|e| {
        crossterm::terminal::disable_raw_mode().ok();
        format!("Failed to enter alternate screen: {}", e)
    })?;

    // RAII guard to restore terminal state on exit (also runs on panic)
    struct TerminalRestore;
    impl Drop for TerminalRestore {
        fn drop(&mut self) {
            crossterm::terminal::disable_raw_mode().ok();
            // Best-effort restore: disable mouse capture and leave alternate screen
            execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen).ok();
        }
    }
    let _terminal_restore = TerminalRestore;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = if let Ok(term) = Terminal::new(backend) {
        term
    } else {
        // If terminal initialization failed, try to restore terminal state and return error.
        crossterm::terminal::disable_raw_mode().ok();
        execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen).ok();
        return Err("Failed to initialize terminal".to_string());
    };

    let mut mos = Mos::new();

    loop {
        if mos.should_quit {
            break;
        }
        
        while crossterm::event::poll(Duration::from_millis(0)).map_err(|e| format!("Failed to poll events: {}", e))? {
            let ev = crossterm::event::read().map_err(|e| format!("Failed to read event: {}", e))?;
            mos.handle_terminal_event(ev);
        }

        mos.update();

        terminal.draw(|frame| {
            mos.render(frame);
        }).map_err(|e| format!("Failed to draw terminal: {}", e))?;

        //  std::thread::sleep(Duration::from_millis(16));
    }

    // Normal cleanup will also happen in TerminalRestore::drop, but do an explicit best-effort here.
    crossterm::terminal::disable_raw_mode().ok();
    execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen).ok();
    Ok(())
}
