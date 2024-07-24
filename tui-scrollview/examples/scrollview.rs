use std::io::{self, stdout};

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Size,
    prelude::*,
    style::palette::tailwind,
    widgets::*,
};
use tui_scrollview::{ScrollView, ScrollViewState};

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::new().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

#[derive(Debug, Default, Clone)]
struct App {
    text: [String; 3],
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
            text: [
                lipsum::lipsum(10_000),
                lipsum::lipsum(10_000),
                lipsum::lipsum(10_000),
            ],
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
                Char('j') | Down => self.scroll_view_state.scroll_down(),
                Char('k') | Up => self.scroll_view_state.scroll_up(),
                Char('f') | PageDown => self.scroll_view_state.scroll_page_down(),
                Char('b') | PageUp => self.scroll_view_state.scroll_page_up(),
                Char('g') | Home => self.scroll_view_state.scroll_to_top(),
                Char('G') | End => self.scroll_view_state.scroll_to_bottom(),
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

const SCROLLVIEW_HEIGHT: u16 = 100;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title, body] = layout.areas(area);

        self.title().render(title, buf);
        let width = if buf.area.height < SCROLLVIEW_HEIGHT {
            buf.area.width - 1
        } else {
            buf.area.width
        };
        let mut scroll_view = ScrollView::new(Size::new(width, SCROLLVIEW_HEIGHT));
        self.render_widgets_into_scrollview(scroll_view.buf_mut());
        scroll_view.render(body, buf, &mut self.scroll_view_state)
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
            "  ↓ | ↑ | PageDown | PageUp | Home | End  "
                .fg(keys_fg)
                .bg(keys_bg),
            "  Quit: ".into(),
            " Esc ".fg(keys_fg).bg(keys_bg),
        ])
        .style((fg, bg))
    }

    fn render_widgets_into_scrollview(&self, buf: &mut Buffer) {
        use Constraint::*;
        let area = buf.area;
        let [numbers, widgets] = Layout::horizontal([Length(5), Fill(1)]).areas(area);
        let [bar_charts, text_0, text_1, text_2] =
            Layout::vertical([Length(7), Fill(1), Fill(2), Fill(4)]).areas(widgets);
        let [left_bar, right_bar] = Layout::horizontal([Length(20), Fill(1)]).areas(bar_charts);

        self.line_numbers(area.height).render(numbers, buf);
        self.vertical_bar_chart().render(left_bar, buf);
        self.horizontal_bar_chart().render(right_bar, buf);
        self.text(0).render(text_0, buf);
        self.text(1).render(text_1, buf);
        self.text(2).render(text_2, buf);
    }

    fn line_numbers(&self, height: u16) -> impl Widget {
        use std::fmt::Write;
        let line_numbers = (1..=height).fold(String::new(), |mut output, n| {
            let _ = writeln!(output, "{n:>4} ");
            output
        });
        Text::from(line_numbers).dim()
    }

    fn vertical_bar_chart(&self) -> impl Widget {
        let block = Block::bordered().title("Vertical Bar Chart");
        BarChart::default()
            .direction(Direction::Vertical)
            .block(block)
            .bar_width(5)
            .bar_gap(1)
            .data(bars())
    }

    fn horizontal_bar_chart(&self) -> impl Widget {
        let block = Block::bordered().title("Horizontal Bar Chart");
        BarChart::default()
            .direction(Direction::Horizontal)
            .block(block)
            .bar_width(1)
            .bar_gap(1)
            .data(bars())
    }

    fn text(&self, index: usize) -> impl Widget {
        let block = Block::bordered().title(format!("Text {}", index));
        Paragraph::new(self.text[index].clone())
            .wrap(Wrap { trim: false })
            .block(block)
    }
}

const CHART_DATA: [(&str, u64, Color); 3] = [
    ("Red", 2, Color::Red),
    ("Green", 7, Color::Green),
    ("Blue", 11, Color::Blue),
];

fn bars() -> BarGroup<'static> {
    let data = CHART_DATA
        .map(|(label, value, color)| Bar::default().label(label.into()).value(value).style(color));
    BarGroup::default().bars(&data)
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
