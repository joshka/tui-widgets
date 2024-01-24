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
                Char('j') | Down => self.scroll.scroll_down(),
                Char('k') | Up => self.scroll.scroll_up(),
                Char('g') | Home => self.scroll.scroll_to_top(),
                Char('G') | End => self.scroll.scroll_to_bottom(),
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
        let size = Size::new(area.width, 100);
        self.render_title(title, buf);
        self.render_scrollview(body, buf, size);
    }
}

impl App {
    fn render_title(&self, title: Rect, buf: &mut Buffer) {
        Paragraph::new("Tui-scrollview example. Esc: quit, ↓: down, ↑: up, Home: top, End: bottom")
            .style((tailwind::SLATE.c900, tailwind::SLATE.c300))
            .render(title, buf);
    }

    fn render_scrollview(&mut self, body: Rect, buf: &mut Buffer, size: Size) {
        let mut scroll_view = ScrollView::new(size);
        self.render_into_scrollview(&mut scroll_view, size);
        scroll_view.render(body, buf, &mut self.scroll);
    }

    fn render_into_scrollview(&self, scroll_view: &mut ScrollView, size: Size) {
        let scroll_area = Rect::new(0, 0, size.width, size.height);
        let [numbers_area, text_area] = scroll_area.split(&Layout::horizontal([
            Constraint::Length(5),
            Constraint::Min(0),
        ]));
        let gauge_area = Rect::new(20, 10, size.width - 40, 10);

        self.render_line_numbers(numbers_area, scroll_view, size);
        self.render_text(text_area, scroll_view);

        self.render_gauge(gauge_area, scroll_view);
    }

    fn render_line_numbers(&self, area: Rect, scroll_view: &mut ScrollView, size: Size) {
        let line_numbers = (1..=size.height)
            .map(|n| format!("{n:>4} \n"))
            .collect::<String>();
        scroll_view.render_widget(Paragraph::new(line_numbers).dim(), area);
    }

    fn render_text(&self, area: Rect, scroll_view: &mut ScrollView) {
        scroll_view.render_widget(
            Paragraph::new(self.text.clone()).wrap(Wrap { trim: false }),
            area,
        );
    }

    fn render_gauge(&self, area: Rect, scroll_view: &mut ScrollView) {
        let percent = (self.scroll.offset().y.saturating_mul(10)).min(100);
        let gauge = Gauge::default()
            .gauge_style(Style::new().blue().on_light_blue())
            .percent(percent);
        scroll_view.render_widget(gauge, area);
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
