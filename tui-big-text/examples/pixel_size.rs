use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    Frame,
};
use tui_big_text::{BigText, PixelSize};

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    common::run(render)?;
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text pixel size demo. <q> quit")
        .centered()
        .cyan();

    let full_size_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec!["FullSize".white().into()])
        .build();

    let half_height_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines(vec!["1/2 high".green().into()])
        .build();

    let half_wide_text = BigText::builder()
        .pixel_size(PixelSize::HalfWidth)
        .lines(vec!["1/2 wide".red().into()])
        .build();

    let quadrant_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec!["Quadrant".blue().into(), " 1/2*1/2".blue().into()])
        .build();

    let third_text = BigText::builder()
        .pixel_size(PixelSize::ThirdHeight)
        .lines(vec!["1/3".yellow().into(), "high".yellow().into()])
        .build();

    let sextant_text = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .lines(vec!["Sextant".cyan().into(), " 1/2*1/3".cyan().into()])
        .build();

    // Setup layout for the title and 6 blocks
    use Constraint::*;
    let [top, full, half_height, middle, bottom] =
        Layout::vertical([Length(2), Length(8), Length(4), Length(8), Length(6)])
            .areas(frame.area());
    let [half_wide, quadrant] = Layout::horizontal([Length(32), Length(32)]).areas(middle);
    let [third_height, sextant] = Layout::horizontal([Length(32), Length(32)]).areas(bottom);

    frame.render_widget(title, top);
    frame.render_widget(full_size_text, full);
    frame.render_widget(half_height_text, half_height);
    frame.render_widget(half_wide_text, half_wide);
    frame.render_widget(quadrant_text, quadrant);
    frame.render_widget(third_text, third_height);
    frame.render_widget(sextant_text, sextant);
}
