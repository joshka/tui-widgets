use color_eyre::Result;
use ratatui::{
    layout::Offset,
    prelude::{Frame, Stylize},
    text::Line,
};
use tui_big_text::{BigText, PixelSize};

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    common::run(render)?;
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text alignment demo. Press 'q' to quit")
        .cyan()
        .centered();

    let left = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .left_aligned()
        .lines(vec!["Left".white().into()])
        .build();

    let right = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .right_aligned()
        .lines(vec!["Right".green().into()])
        .build();

    let centered = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .centered()
        .lines(vec!["Centered".red().into()])
        .build();

    let area = frame.size();
    frame.render_widget(title, area);

    let area = area.offset(Offset { x: 0, y: 2 }).intersection(area);
    frame.render_widget(left, area);
    frame.render_widget(right, area);
    frame.render_widget(centered, area);
}
