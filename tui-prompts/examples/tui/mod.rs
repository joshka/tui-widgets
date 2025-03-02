use std::{
    io::Stderr,
    ops::{Deref, DerefMut},
};

use color_eyre::Result;
use ratatui::{backend::CrosstermBackend, crossterm, Terminal};

/// A wrapper around the terminal that enables raw mode on creation and disables it on drop.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = Self::init()?;
        Ok(Self { terminal })
    }

    fn init() -> Result<Terminal<CrosstermBackend<Stderr>>> {
        let buffer = std::io::stderr();
        let mut backend = CrosstermBackend::new(buffer);
        crossterm::execute!(backend, crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(terminal)
    }

    fn cleanup(&mut self) -> Result<()> {
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Deref for Tui {
    type Target = Terminal<CrosstermBackend<Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.cleanup().unwrap();
    }
}
