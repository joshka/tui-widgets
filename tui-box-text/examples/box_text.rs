use std::iter::zip;

use color_eyre::eyre::Ok;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    text::Line,
    DefaultTerminal, Frame,
};
use tui_box_text::BoxChar;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(draw)?;
        if matches!(event::read()?, Event::Key(key) if key.code == KeyCode::Esc) {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [header, body] = layout.areas(frame.area());
    frame.render_widget(
        Line::from("Tui-box-text. Press Esc to exit").centered(),
        header,
    );
    let mut areas = Vec::new();
    for y in (body.top() + 3..body.bottom()).step_by(3) {
        for x in (body.left() + 4..body.right()).step_by(4) {
            areas.push(Rect::new(x - 4, y - 3, 4, 3));
        }
    }
    for (c, area) in zip(' '..='~', areas) {
        let box_char = BoxChar::new(c);
        frame.render_widget(&box_char, area);
    }
}
