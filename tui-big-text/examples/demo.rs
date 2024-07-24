use std::{io::stdout, thread::sleep, time::Duration};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType},
};
use tui_big_text::BigText;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|frame| render(frame).expect("failed to render"))?;
    sleep(Duration::from_secs(5));
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render(frame: &mut Frame) -> Result<()> {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Tui-big-text Demo");
    frame.render_widget(&block, frame.size());
    let area = block.inner(frame.size());
    let big_text = BigText::builder()
        .style(Style::new().blue())
        .lines(vec![
            "Tui-".red().into(),
            "big-".white().into(),
            "text".into(),
        ])
        .build()?;
    frame.render_widget(big_text, area);
    Ok(())
}
