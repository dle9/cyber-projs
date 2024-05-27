use std::io::{stdout, Result};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::{CrosstermBackend};

mod app; use app::App;
mod player;
mod screens;
mod controls;

fn main() -> Result<()> {
    // (1) set up the space for the application
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?; // allow custom terminal I/O handling
    let backend = CrosstermBackend::new(stdout()); // backend handles I/O 
    let mut tui = Terminal::new(backend)?;

    // (2) run the app
    let mut app = App::new(); // create the App instance
    let res = app.run(&mut tui); // run it
    
    // (3) restore the terminal
    execute!(stdout(), LeaveAlternateScreen)?; 
    disable_raw_mode()?;
    return res; // return App::run result
}