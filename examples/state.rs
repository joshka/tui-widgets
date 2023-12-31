use std::{io::stdout, ops::ControlFlow};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use lipsum::lipsum;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};
use tui_popup::{Popup, PopupState};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = init_terminal()?;
    let mut popup_state = PopupState::default();
    loop {
        terminal.draw(|frame| render(frame, &mut popup_state))?;
        if handle_events(&mut popup_state)?.is_break() {
            break;
        }
    }
    restore_terminal()?;
    Ok(())
}

fn render(frame: &mut Frame<'_>, popup_state: &mut PopupState) {
    let area = frame.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);
    let (background_area, status_area) = (layout[0], layout[1]);
    let lorem_ipsum = lipsum(area.area() as usize / 5);
    let background = Paragraph::new(lorem_ipsum)
        .wrap(Wrap { trim: false })
        .dark_gray();
    frame.render_widget(background, background_area);
    let body = Text::from(vec![
        "q: exit".into(),
        "r: reset".into(),
        "j: move down".into(),
        "k: move up".into(),
        "h: move left".into(),
        "l: move right".into(),
    ]);
    let popup = Popup::new("Popup", body).style(Style::new().white().on_blue());
    frame.render_stateful_widget(popup.to_widget(), background_area, popup_state);
    let text = format!("{area:?}", area = popup_state.area());
    let status = Paragraph::new(text).style(Style::new().white().on_black());
    frame.render_widget(status, status_area);
}

fn handle_events(popup_state: &mut PopupState) -> color_eyre::Result<ControlFlow<()>> {
    if let Event::Key(KeyEvent {
        kind: KeyEventKind::Press,
        code,
        ..
    }) = event::read()?
    {
        use KeyCode as key;
        use KeyCode::Char as char;
        match code {
            char('q') | key::Esc => return Ok(ControlFlow::Break(())),
            char('r') => *popup_state = PopupState::default(),
            char('j') | key::Down => popup_state.move_by(0, 1),
            char('k') | key::Up => popup_state.move_by(0, -1),
            char('h') | key::Left => popup_state.move_by(-1, 0),
            char('l') | key::Right => popup_state.move_by(1, 0),
            _ => {}
        }
    };
    Ok(ControlFlow::Continue(()))
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, color_eyre::eyre::Error> {
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    enable_raw_mode()?;
    Ok(terminal)
}

fn restore_terminal() -> Result<(), color_eyre::eyre::Error> {
    disable_raw_mode()?;
    stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
