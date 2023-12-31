use std::io::stdout;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    enable_raw_mode()?;
    let popup = tui_popup::Popup::new("Greeting", "Hello, world!");
    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(popup.to_widget(), size);
    })?;
    disable_raw_mode()?;
    println!();
    Ok(())
}
