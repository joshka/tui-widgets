use color_eyre::Result;
use lipsum::lipsum;
use ratatui::{
    crossterm::event::{self, Event},
    prelude::{Rect, Style, Stylize},
    widgets::{Paragraph, Wrap},
    Frame,
};
use tui_popup::Popup;

mod terminal;

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    loop {
        terminal.draw(render)?;
        if read_any_key()? {
            break;
        }
    }
    terminal::restore()?;
    Ok(())
}

fn render(frame: &mut Frame) {
    let area = frame.size();
    let background = background(area);
    let popup = Popup::new("Press any key to exit")
        .title("tui-popup demo")
        .style(Style::new().white().on_blue());
    frame.render_widget(background, area);
    frame.render_widget(&popup, area);
}

fn read_any_key() -> Result<bool> {
    let event = event::read()?;
    Ok(matches!(event, Event::Key(_)))
}

fn background(area: Rect) -> Paragraph<'static> {
    let lorem_ipsum = lipsum(area.area() as usize / 5);
    Paragraph::new(lorem_ipsum)
        .wrap(Wrap { trim: false })
        .dark_gray()
}
