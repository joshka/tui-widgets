use std::iter::zip;

use color_eyre::eyre::Ok;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
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
        terminal.draw(|frame| draw(frame))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => break Ok(()),
                _ => {}
            },
            _ => {}
        }
    }
}

fn draw(frame: &mut Frame) {
    let area = frame.area();
    let mut areas = Vec::new();
    for y in (area.top() + 3..area.bottom()).step_by(3) {
        for x in (area.left() + 4..area.right()).step_by(4) {
            areas.push(Rect::new(x - 4, y - 3, 4, 3));
        }
    }
    for (c, area) in zip(' '..='~', areas) {
        let box_char = BoxChar::new(c);
        frame.render_widget(&box_char, area);
    }
}
