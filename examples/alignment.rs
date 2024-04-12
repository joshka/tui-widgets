use std::{io::stdout, thread::sleep, time::Duration};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use tui_big_text::{BigText, PixelSize};

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
    let left = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .alignment(Alignment::Left)
        .lines(vec!["Left".white().into()])
        .build()?;

    let right = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .alignment(Alignment::Right)
        .lines(vec!["Right".green().into()])
        .build()?;

    let centered = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .alignment(Alignment::Center)
        .lines(vec!["Centered".red().into()])
        .build()?;

    use Constraint::*;
    let [top, middle, bottom] = Layout::vertical([Length(4); 3]).areas(frame.size());

    frame.render_widget(left, top);
    frame.render_widget(right, middle);
    frame.render_widget(centered, bottom);

    Ok(())
}
