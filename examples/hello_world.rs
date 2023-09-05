use std::{io::stdout, thread::sleep, time::Duration};

use anyhow::Result;
use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use tui_big_text::BigTextBuilder;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|frame| render(frame).expect("failed to render"))?;
    sleep(Duration::from_secs(5));
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn render<B: Backend>(frame: &mut Frame<B>) -> Result<()> {
    let big_text = BigTextBuilder::default()
        .style(Style::new().blue())
        .lines(vec![
            "Hello".red().into(),
            "World".white().into(),
            "~~~~~".into(),
        ])
        .build()?;
    frame.render_widget(big_text, frame.size());
    Ok(())
}
