use color_eyre::Result;
use lipsum::lipsum;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::{Rect, Span, Style, Stylize, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};
use tui_popup::{Popup, SizedWrapper};

mod terminal;

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::default();
    while !app.should_exit {
        terminal.draw(|frame| app.render(frame))?;
        app.handle_events()?;
    }
    terminal::restore()?;
    Ok(())
}

#[derive(Default)]
struct App {
    should_exit: bool,
    scroll: u16,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let background = background(area);

        let paragraph = paragraph(self.scroll);
        let popup = Popup::new(paragraph)
            .title("scroll: ↑/↓ quit: Esc")
            .style(Style::new().white().on_blue());

        frame.render_widget(background, area);
        frame.render_widget(&popup, area);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                _ => {}
            }
        }
        Ok(())
    }

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}

fn paragraph(scroll: u16) -> SizedWrapper<Paragraph<'static>> {
    let lines: Text = (0..10).map(|i| Span::raw(format!("Line {i}"))).collect();
    let paragraph = Paragraph::new(lines).scroll((scroll, 0));
    SizedWrapper {
        inner: paragraph,
        width: 21,
        height: 5,
    }
}

fn background(area: Rect) -> Paragraph<'static> {
    let lorem_ipsum = lipsum(area.area() as usize / 5);
    Paragraph::new(lorem_ipsum)
        .wrap(Wrap { trim: false })
        .dark_gray()
}
