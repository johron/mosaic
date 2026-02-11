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
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::Mos;

fn main() -> Result<(), String> {
    if crossterm::terminal::enable_raw_mode().is_err() {
        return Err("Failed to enable raw mode".to_string());
    }
    
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = if let Ok(term) = Terminal::new(backend) {;
        term
    } else {
        crossterm::terminal::disable_raw_mode().ok();
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

    crossterm::terminal::disable_raw_mode().ok();
    Ok(())
}
