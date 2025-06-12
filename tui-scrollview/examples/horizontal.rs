use std::io;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Size;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::DefaultTerminal;
use tui_scrollview::{ScrollView, ScrollViewState};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default, Clone)]
struct App {
    text: String,
    scroll_view_state: ScrollViewState,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    fn new() -> Self {
        Self {
            text: lipsum::lipsum(10_000),
            ..Default::default()
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    const fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();
            let size = Size::new(area.width * 2, area.width);
            let mut scroll_view = ScrollView::new(size);
            let paragraph = Paragraph::new(self.text.clone()).wrap(Wrap::default());
            scroll_view.render_widget(paragraph, scroll_view.area());
            frame.render_stateful_widget(scroll_view, area, &mut self.scroll_view_state);
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }
}
