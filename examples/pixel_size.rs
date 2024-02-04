use std::{io::stdout, thread::sleep, time::Duration};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use tui_big_text::BigTextBuilder;

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
    // Setup layout for 4 blocks
    use Constraint::*;
    let [top, middle, bottom] =
        Layout::new(Direction::Vertical, [Length(8), Length(4), Length(8)]).areas(frame.size());
    let [bottom_left, bottom_right] =
        Layout::new(Direction::Horizontal, [Length(32), Length(32)]).areas(bottom);

    // render one block for each font size
    // Draw block showing Full size
    let big_text_full = BigTextBuilder::default()
        .pixel_size(tui_big_text::PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec!["FullSize".white().into()])
        .build()?;
    frame.render_widget(big_text_full, top);

    // Draw block showing HalfHeight size
    let big_text_half_height = BigTextBuilder::default()
        .pixel_size(tui_big_text::PixelSize::HalfHeight)
        .style(Style::new().blue())
        .lines(vec!["1/2 high".green().into()])
        .build()?;
    frame.render_widget(big_text_half_height, middle);

    // Draw block showing HalfWidth size
    let big_text_half_width = BigTextBuilder::default()
        .pixel_size(tui_big_text::PixelSize::HalfWidth)
        .style(Style::new().blue())
        .lines(vec!["1/2 wide".red().into()])
        .build()?;
    frame.render_widget(big_text_half_width, bottom_left);

    // Draw block showing Half size
    let big_text_half_size = BigTextBuilder::default()
        .pixel_size(tui_big_text::PixelSize::Quadrant)
        .style(Style::new().blue())
        .lines(vec!["Quadrant".blue().into(), "1/2 both".blue().into()])
        .build()?;
    frame.render_widget(big_text_half_size, bottom_right);

    Ok(())
}
