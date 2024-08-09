//! An example of using multiple tabs each with a scrollable view, as well as how state can be
//! managed across multiple tabs using Stateful Widgets.
//!
//! This example uses the `unstable-widget-ref` feature in Ratatui to allow the tab widgets to
//! created once and then reused across multiple frames. Each tab has some static lorem ipsum text,
//! and we store the scroll state for each tab separately.

use std::{collections::HashMap, fmt::Debug, io};

use color_eyre::Result;
use lipsum::lipsum;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect, Size},
    style::{palette::tailwind, Stylize},
    text::Line,
    widgets::{Paragraph, StatefulWidget, StatefulWidgetRef, Tabs, Widget, Wrap},
};
use tui::Tui;
use tui_scrollview::{ScrollView, ScrollViewState};

fn main() -> Result<()> {
    tui::init_error_hooks()?;
    let terminal = tui::init_terminal()?;
    let app = &mut App::new();
    app.run(terminal)?;
    tui::restore_terminal()?;
    Ok(())
}

#[derive(Default)]
struct App {
    state: AppState,
    tabs: HashMap<
        VisibleTab,
        (
            Box<dyn StatefulWidgetRef<State = ScrollViewState>>,
            ScrollViewState,
        ),
    >,
    visible_tab: VisibleTab,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum VisibleTab {
    #[default]
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
struct RedTab {
    text: String,
}

impl RedTab {
    fn new() -> Self {
        Self { text: lipsum(500) }
    }
}

#[derive(Debug, Default, Clone)]
struct GreenTab {
    text: String,
}

impl GreenTab {
    fn new() -> Self {
        Self {
            text: lipsum(1_000),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct BlueTab {
    text: String,
}

impl BlueTab {
    fn new() -> Self {
        Self {
            text: lipsum(10_000),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    fn new() -> Self {
        let mut tabs: HashMap<
            VisibleTab,
            (
                Box<dyn StatefulWidgetRef<State = ScrollViewState>>,
                ScrollViewState,
            ),
        > = HashMap::new();
        tabs.insert(
            VisibleTab::Red,
            (Box::new(RedTab::new()), ScrollViewState::default()),
        );
        tabs.insert(
            VisibleTab::Green,
            (Box::new(GreenTab::new()), ScrollViewState::default()),
        );
        tabs.insert(
            VisibleTab::Blue,
            (Box::new(BlueTab::new()), ScrollViewState::default()),
        );
        Self {
            tabs,
            ..Default::default()
        }
    }

    fn run(&mut self, mut tui: Tui) -> Result<()> {
        while self.is_running() {
            self.draw(&mut tui)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(&mut self, tui: &mut Tui) -> io::Result<()> {
        tui.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        let (_widget, scroll_view_state) = self
            .tabs
            .get_mut(&self.visible_tab)
            .expect("visible tab should exist");
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Tab => {
                    self.visible_tab = match self.visible_tab {
                        VisibleTab::Red => VisibleTab::Green,
                        VisibleTab::Green => VisibleTab::Blue,
                        VisibleTab::Blue => VisibleTab::Red,
                    }
                }
                BackTab => {
                    self.visible_tab = match self.visible_tab {
                        VisibleTab::Red => VisibleTab::Blue,
                        VisibleTab::Green => VisibleTab::Red,
                        VisibleTab::Blue => VisibleTab::Green,
                    }
                }
                Char('j') | Down => scroll_view_state.scroll_down(),
                Char('k') | Up => scroll_view_state.scroll_up(),
                Char('f') | PageDown => scroll_view_state.scroll_page_down(),
                Char('b') | PageUp => scroll_view_state.scroll_page_up(),
                Char('g') | Home => scroll_view_state.scroll_to_top(),
                Char('G') | End => scroll_view_state.scroll_to_bottom(),
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
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]);
        let [title, tabs, body] = layout.areas(area);

        self.title().render(title, buf);
        self.tabs().render(tabs, buf);
        let (tab, mut state) = self.tabs.get_mut(&self.visible_tab).unwrap();
        tab.render_ref(body, buf, &mut state);
    }
}

impl App {
    fn title(&self) -> impl Widget {
        let palette = tailwind::SLATE;
        let fg = palette.c900;
        let bg = palette.c300;
        let keys_fg = palette.c50;
        let keys_bg = palette.c600;
        Line::from(vec![
            "Tui-scrollview  ".into(),
            "  ↓ | ↑ | PageDown | PageUp | Home | End | Tab  "
                .fg(keys_fg)
                .bg(keys_bg),
            "  Quit: ".into(),
            " Esc ".fg(keys_fg).bg(keys_bg),
        ])
        .style((fg, bg))
    }

    fn tabs(&self) -> impl Widget {
        let selected = self.visible_tab as usize;
        Tabs::new([
            " Red ".fg(tailwind::RED.c900),
            " Green ".fg(tailwind::GREEN.c900),
            " Blue ".fg(tailwind::BLUE.c900),
        ])
        .padding("", "")
        .divider("")
        .select(selected)
        .style(tailwind::SLATE.c900)
    }
}

impl StatefulWidgetRef for RedTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 50;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_red()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}

impl StatefulWidgetRef for GreenTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 100;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_green()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}

impl StatefulWidgetRef for BlueTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 200;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_blue()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}

mod tui {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{config::HookBuilder, eyre, Result};
    use ratatui::{
        backend::CrosstermBackend,
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };

    // a type alias to simplify the use of the crossterm backend
    pub type Tui = Terminal<CrosstermBackend<Stdout>>;

    pub fn init_error_hooks() -> Result<()> {
        let (panic, error) = HookBuilder::default().into_hooks();
        let panic = panic.into_panic_hook();
        let error = error.into_eyre_hook();
        eyre::set_hook(Box::new(move |e| {
            let _ = restore_terminal();
            error(e)
        }))?;
        panic::set_hook(Box::new(move |info| {
            let _ = restore_terminal();
            panic(info)
        }));
        Ok(())
    }

    pub fn init_terminal() -> io::Result<Tui> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
