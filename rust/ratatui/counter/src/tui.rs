/// A type alias for the terminal type used in this application
pub type Tui = ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>;

/// Initialize the terminal
pub fn init() -> std::io::Result<Tui> {
    // separate app from the terminal
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    // disable terminal IO processing 
    // allow you to set up your own IO handling
    crossterm::terminal::enable_raw_mode()?;

    // create crossterm backend to handle IO
    ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(std::io::stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> std::io::Result<()> {
    crossterm::execute!(std::io::stdout(), erm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}