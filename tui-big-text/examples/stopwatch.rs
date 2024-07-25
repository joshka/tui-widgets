use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
// use futures::{select, FutureExt, StreamExt};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::Paragraph};
use strum::EnumIs;
use tokio::select;
use tui_big_text::BigText;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = StopwatchApp::default();
    app.run().await
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIs)]
enum AppState {
    #[default]
    Stopped,
    Running,
    Quitting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    StartOrSplit,
    Stop,
    Tick,
    Quit,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct StopwatchApp {
    state: AppState,
    splits: Vec<Instant>,
    fps_counter: FpsCounter,
}

impl StopwatchApp {
    async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::init()?;
        let mut events = EventHandler::new(60.0);
        while !self.state.is_quitting() {
            self.draw(&mut tui)?;
            let message = events.next().await?;
            self.handle_message(message)?;
        }
        Ok(())
    }

    fn handle_message(&mut self, message: Message) -> Result<()> {
        match message {
            Message::StartOrSplit => self.start_or_split(),
            Message::Stop => self.stop(),
            Message::Tick => self.tick(),
            Message::Quit => self.quit(),
        }
        Ok(())
    }

    fn start_or_split(&mut self) {
        if self.state.is_stopped() {
            self.start();
        } else {
            self.record_split();
        }
    }

    fn stop(&mut self) {
        self.record_split();
        self.state = AppState::Stopped;
    }

    fn tick(&mut self) {
        self.fps_counter.tick()
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting
    }

    fn start(&mut self) {
        self.splits.clear();
        self.state = AppState::Running;
        self.record_split();
    }

    fn record_split(&mut self) {
        if !self.state.is_running() {
            return;
        }
        self.splits.push(Instant::now());
    }

    fn elapsed(&mut self) -> Duration {
        if self.state.is_running() {
            self.splits.first().map_or(Duration::ZERO, Instant::elapsed)
        } else {
            // last - first or 0 if there are no splits
            let now = Instant::now();
            let first = *self.splits.first().unwrap_or(&now);
            let last = *self.splits.last().unwrap_or(&now);
            last - first
        }
    }

    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let layout = layout(frame.size());
            frame.render_widget(Paragraph::new("Stopwatch Example"), layout[0]);
            frame.render_widget(self.fps_paragraph(), layout[1]);
            frame.render_widget(self.timer_paragraph(), layout[2]);
            frame.render_widget(Paragraph::new("Splits:"), layout[3]);
            frame.render_widget(self.splits_paragraph(), layout[4]);
            frame.render_widget(self.help_paragraph(), layout[5]);
        })
    }

    fn fps_paragraph(&mut self) -> Paragraph<'_> {
        let fps = format!("{:.2} fps", self.fps_counter.fps);
        Paragraph::new(fps).dim().right_aligned()
    }

    fn timer_paragraph(&mut self) -> BigText<'_> {
        let style = if self.state.is_running() {
            Style::new().green()
        } else {
            Style::new().red()
        };
        let duration = format_duration(self.elapsed());
        let lines = vec![duration.into()];
        BigText::builder().lines(lines).style(style).build()
    }

    /// Renders the splits as a list of lines.
    ///
    /// ```text
    /// #01 -- 00:00.693 -- 00:00.693
    /// #02 -- 00:00.719 -- 00:01.413
    /// ```
    fn splits_paragraph(&mut self) -> Paragraph<'_> {
        let start = *self.splits.first().unwrap_or(&Instant::now());
        let mut splits = self
            .splits
            .iter()
            .copied()
            .tuple_windows()
            .enumerate()
            .map(|(index, (prev, current))| format_split(index, start, prev, current))
            .collect::<Vec<_>>();
        splits.reverse();
        Paragraph::new(splits)
    }

    fn help_paragraph(&mut self) -> Paragraph<'_> {
        let space_action = if self.state.is_stopped() {
            "start"
        } else {
            "split"
        };
        let help_text = Line::from(vec![
            "space ".into(),
            space_action.dim(),
            " enter ".into(),
            "stop".dim(),
            " q ".into(),
            "quit".dim(),
        ]);
        Paragraph::new(help_text).gray()
    }
}

fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::vertical(vec![
        Constraint::Length(2), // top bar
        Constraint::Length(8), // timer
        Constraint::Length(1), // splits header
        Constraint::Min(0),    // splits
        Constraint::Length(1), // help
    ])
    .split(area);
    let top_layout = Layout::horizontal(vec![
        Constraint::Length(20), // title
        Constraint::Min(0),     // fps counter
    ])
    .split(layout[0]);

    // return a new vec with the top_layout rects and then rest of layout
    top_layout[..]
        .iter()
        .chain(layout[1..].iter())
        .copied()
        .collect()
}

fn format_split<'a>(index: usize, start: Instant, previous: Instant, current: Instant) -> Line<'a> {
    let split = format_duration(current - previous);
    let elapsed = format_duration(current - start);
    Line::from(vec![
        format!("#{:02} -- ", index + 1).into(),
        Span::styled(split, Style::new().yellow()),
        " -- ".into(),
        Span::styled(elapsed, Style::new()),
    ])
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{:02}:{:02}.{:03}",
        duration.as_secs() / 60,
        duration.as_secs() % 60,
        duration.subsec_millis()
    )
}

#[derive(Debug, Clone, PartialEq)]
struct FpsCounter {
    start_time: Instant,
    frames: u32,
    pub fps: f64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            frames: 0,
            fps: 0.0,
        }
    }

    fn tick(&mut self) {
        self.frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frames as f64 / elapsed;
            self.start_time = now;
            self.frames = 0;
        }
    }
}

/// Handles events from crossterm and emits `Message`s.
struct EventHandler {
    crossterm_events: event::EventStream,
    interval: tokio::time::Interval,
}

impl EventHandler {
    /// Creates a new event handler that emits a `Message::Tick` every `1.0 / max_fps` seconds.
    fn new(max_fps: f32) -> Self {
        let period = Duration::from_secs_f32(1.0 / max_fps);
        Self {
            crossterm_events: event::EventStream::new(),
            interval: tokio::time::interval(period),
        }
    }

    async fn next(&mut self) -> Result<Message> {
        select! {
            event = self.crossterm_events.next().fuse() => Self::handle_crossterm_event(event),
            _ = self.interval.tick().fuse() => Ok(Message::Tick),
        }
    }

    fn handle_crossterm_event(
        event: Option<core::result::Result<event::Event, std::io::Error>>,
    ) -> Result<Message> {
        match event {
            Some(Ok(event::Event::Key(key))) => Ok(match key.code {
                KeyCode::Char('q') => Message::Quit,
                KeyCode::Char(' ') => Message::StartOrSplit,
                KeyCode::Char('s') | KeyCode::Enter => Message::Stop,
                _ => Message::Tick,
            }),
            Some(Err(err)) => bail!(err),
            None => bail!("event stream ended unexpectedly"),
            _ => Ok(Message::Tick),
        }
    }
}

struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    fn init() -> Result<Tui> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).wrap_err("failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).wrap_err("failed to create terminal")?;
        enable_raw_mode().wrap_err("failed to enable raw mode")?;
        terminal.hide_cursor().wrap_err("failed to hide cursor")?;
        terminal.clear().wrap_err("failed to clear console")?;
        Ok(Self { terminal })
    }

    fn draw(&mut self, frame: impl FnOnce(&mut Frame)) -> Result<()> {
        self.terminal.draw(frame).wrap_err("failed to draw frame")?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().expect("failed to disable raw mode");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("failed to switch to main screen");
        self.terminal.show_cursor().expect("failed to show cursor");
        self.terminal.clear().expect("failed to clear console");
    }
}
