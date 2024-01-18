use std::io::{self, stdout};

use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Size,
    prelude::*,
    style::palette::tailwind,
    widgets::{Gauge, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Debug, Default, Clone)]
struct App {
    text: String,
    scroll: ScrollViewState,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::new().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn new() -> Self {
        Self {
            text: lipsum::lipsum(10_000),
            ..Default::default()
        }
    }

    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        self.draw(&mut terminal)?;
        while self.is_running() {
            self.handle_events()?;
            self.draw(&mut terminal)?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('j') | Down => self.scroll.down(),
                Char('k') | Up => self.scroll.up(),
                Char('g') | Home => self.scroll.top(),
                Char('G') | End => self.scroll.bottom(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let [title, body] = area.split(&layout);

        self.render_title(title, buf);
        self.render_scrollview(area, body, buf);
    }
}

impl App {
    fn render_title(&self, title: Rect, buf: &mut Buffer) {
        Paragraph::new("Tui-scrollview example. Esc: quit, ↓: down, ↑: up, Home: top, End: bottom")
            .style((tailwind::SLATE.c900, tailwind::SLATE.c300))
            .render(title, buf);
    }

    fn render_scrollview(&mut self, area: Rect, body: Rect, buf: &mut Buffer) {
        let size = Size::new(area.width, 100);
        let mut scroll_view = ScrollView::new(size);
        self.render_into_scrollview(&mut scroll_view, size);
        scroll_view.render(body, buf, &mut self.scroll);
    }

    fn render_into_scrollview(&self, scroll_view: &mut ScrollView, size: Size) {
        let scroll_area = Rect::new(0, 0, size.width, size.height);
        let [numbers, text] = scroll_area.split(&Layout::horizontal([
            Constraint::Length(5),
            Constraint::Min(0),
        ]));
        let line_numbers = (1..=size.height)
            .map(|n| format!("{n:>4} \n"))
            .collect::<String>();
        scroll_view.render_widget(Paragraph::new(line_numbers).dim(), numbers);
        scroll_view.render_widget(
            Paragraph::new(self.text.clone()).wrap(Wrap { trim: false }),
            text,
        );
        let percent = (self.scroll.offset().1.saturating_mul(2)).min(100);
        let gauge = Gauge::default()
            .gauge_style(Style::new().blue().on_light_blue())
            .percent(percent);
        scroll_view.render_widget(gauge, Rect::new(text.x + 20, 45, text.width - 40, 10));
    }
}

fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
