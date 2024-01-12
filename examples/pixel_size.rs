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
    let outer_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)],
    )
    .split(frame.size());

    let inner_layout_top = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)],
    )
    .split(outer_layout[0]);

    let inner_layout_bottom = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)],
    )
    .split(outer_layout[1]);

    // render one block for each font size
    {
        // Draw block showing Full size
        let big_text_full = BigTextBuilder::default()
            .pixel_size(tui_big_text::PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_full, inner_layout_top[0]);
    }

    {
        // Draw block showing HalfWidth size
        let big_text_half_width = BigTextBuilder::default()
            .pixel_size(tui_big_text::PixelSize::HalfWidth)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_width, inner_layout_top[1]);
    }

    {
        // Draw block showing HalfHeight size
        let big_text_half_height = BigTextBuilder::default()
            .pixel_size(tui_big_text::PixelSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_height, inner_layout_bottom[0]);
    }

    {
        // Draw block showing Half size
        let big_text_half_size = BigTextBuilder::default()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_size, inner_layout_bottom[1]);
    }

    Ok(())
}
