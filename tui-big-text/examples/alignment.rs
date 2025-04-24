use color_eyre::Result;
use ratatui::layout::{Constraint, Layout, Offset};
use ratatui::prelude::{Frame, Stylize};
use ratatui::text::Line;
use tui_big_text::{BigText, PixelSize};

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    common::run(render)?;
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text alignment demo. <q> quit")
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

    let area = frame.area();
    frame.render_widget(title, area);

    let area = area.offset(Offset { x: 0, y: 2 }).intersection(area);
    let [top, middle, bottom] = Layout::vertical([Constraint::Length(4); 3]).areas(area);
    frame.render_widget(left, top);
    frame.render_widget(centered, middle);
    frame.render_widget(right, bottom);
}
