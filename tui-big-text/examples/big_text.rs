use color_eyre::Result;
use ratatui::{
    layout::Offset,
    prelude::{Frame, Style, Stylize},
    text::Line,
};
use tui_big_text::BigText;

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    common::run(render)?;
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text demo. <q> quit").cyan();

    let big_text = BigText::builder()
        .style(Style::new().blue())
        .lines(vec![
            "Tui-".red().into(),
            "big-".white().into(),
            "text".into(),
        ])
        .build();

    let area = frame.size();
    frame.render_widget(title, area);

    let area = area.offset(Offset { x: 0, y: 2 }).intersection(area);
    frame.render_widget(big_text, area);
}
