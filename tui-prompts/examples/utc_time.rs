mod tui;

use std::time::Duration;

use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui::Tui;
use tui_prompts::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = App::new(cli);
    app.run()?;
    Ok(())
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct App<'a> {
    today_state: DateTimeState<'a>,
    app_state: AppState,
}

impl<'a> App<'a> {
    pub fn new(_cli: Cli) -> Self {
        Self {
            today_state: DateTimeState::new(),
            app_state: AppState::Running,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        *self.today_state.focus_state_mut() = FocusState::Focused;
        while self.is_running() {
            tui.draw(|frame| self.draw_ui(frame))?;
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key_event) = event::read()? {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        let [_, intro_area, _, today_area, _, text_area, _, converted_area, _, info_area] =
            Layout::new(
                Direction::Vertical,
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ],
            )
            .areas(frame.area());

        match self.today_state.is_finished() {
            false => {
                frame.render_widget(
                    Line::from("Please enter a local time below for conversion."),
                    intro_area,
                );

                DateTimePrompt::new(
                    std::borrow::Cow::Borrowed("today"),
                    vec![
                        NumericChildPrompt::new(std::borrow::Cow::Borrowed("year")),
                        NumericChildPrompt::new(std::borrow::Cow::Borrowed("month")),
                        NumericChildPrompt::new(std::borrow::Cow::Borrowed("day")),
                        NumericChildPrompt::new(std::borrow::Cow::Borrowed("hour")),
                        NumericChildPrompt::new(std::borrow::Cow::Borrowed("minute")),
                    ],
                    Span::raw("  "),
                )
                .with_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Local Time ")
                        .padding(Padding::horizontal(1)),
                )
                .draw(frame, today_area, &mut self.today_state);
                frame.render_widget(
                    Line::from(" Q: Exit | N: Current Time | Enter: Convert ").centered(),
                    info_area,
                );
            }
            true => {
                frame.render_widget(
                    Line::from("At that local time, the time in UTC is:").centered(),
                    text_area,
                );
                let naive = self.today_state.output_value().unwrap();
                let local_datetime: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
                frame.render_widget(
                    Line::from(format!(
                        "Time entered: {x}",
                        x = local_datetime.format("%a, %d %b %Y %H:%M").to_string()
                    ))
                    .centered(),
                    today_area,
                );
                let utc_datetime = local_datetime.to_utc();
                frame.render_widget(
                    Line::from(utc_datetime.format("%a, %d %b %Y %H:%M").to_string()).centered(),
                    converted_area,
                );
                frame.render_widget(Line::from(" Q: Exit ").centered(), info_area);
            }
        }
    }

    fn quit(&mut self) {
        self.app_state = AppState::Quit;
    }

    fn is_running(&self) -> bool {
        self.app_state == AppState::Running
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Enter, _) => self.submit(),
            (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('Q'), KeyModifiers::SHIFT) => self.quit(),
            (KeyCode::Char('n'), KeyModifiers::NONE)
            | (KeyCode::Char('N'), KeyModifiers::SHIFT) => self.today_state.time_now(),
            _ => self.focus_handle_event(key_event),
        }
    }

    fn focus_handle_event(&mut self, key_event: KeyEvent) {
        self.today_state.handle_key_event(key_event);
    }

    fn submit(&mut self) {
        self.today_state.complete();
        if self.today_state.is_finished() {
            self.today_state.blur();
        }
    }
}
