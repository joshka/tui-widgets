use std::io::{self, stdout};

use color_eyre::Result;
use ratatui::{
    crossterm::{
        self,
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::{Constraint, CrosstermBackend, Frame, Layout, Rect, Style, Stylize},
    symbols::border,
    widgets::{Paragraph, Wrap},
    DefaultTerminal, Terminal,
};
use tui_popup::{KnownSizeWrapper, Popup, PopupState};

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}

pub fn try_init() -> io::Result<DefaultTerminal> {
    set_panic_hook();
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::new(backend)
}

pub fn init() -> DefaultTerminal {
    try_init().expect("failed to initialize terminal")
}

pub fn try_restore() -> io::Result<()> {
    // disabling raw mode first is important as it has more side effects than leaving the alternate
    // screen buffer
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub fn restore() {
    if let Err(err) = try_restore() {
        // There's not much we can do if restoring the terminal fails, so we just print the error
        eprintln!("Failed to restore terminal: {err}");
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = init();
    let result = run(terminal);
    restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let (width, height) = crossterm::terminal::size()?;
    let mut state = PopupState::new(Rect::new(0, 0, width, height));
    let mut exit = false;
    while !exit {
        terminal.draw(|frame| draw(frame, &mut state))?;
        handle_events(&mut state, &mut exit)?;
    }
    Ok(())
}

fn draw(frame: &mut Frame, state: &mut PopupState) {
    let vertical = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]);
    let [background_area, status_area] = vertical.areas(frame.area());

    render_background(frame, background_area);
    render_popup(frame, background_area, state);
    render_status_bar(frame, status_area, state);
}

fn render_background(frame: &mut Frame, area: Rect) {
    let background = Paragraph::new("Background content - click and drag the popup to move it!")
        .style(Style::default().dark_gray());
    frame.render_widget(background, area);
}

fn render_popup(frame: &mut Frame, area: Rect, state: &mut PopupState) {
    let content = "This is an interactive popup example!\n\
                    \n\
                    • Drag the popup by clicking and dragging anywhere inside it\n\
                    • Resize by dragging the ⟋ handle in the bottom-right corner\n\
                    • Press 'q' to exit";

    let wrapped_content =
        KnownSizeWrapper::new(Paragraph::new(content).wrap(Wrap { trim: true }), 40, 6);

    let popup = Popup::new(wrapped_content)
        .title("Interactive Popup")
        .border_set(border::ROUNDED)
        .border_style(Style::default().blue().bold());

    frame.render_stateful_widget(popup, area, state);
}

/// Status bar at the bottom of the screen
///
/// Must be called after rendering the popup widget as it relies on the popup area being set
fn render_status_bar(frame: &mut Frame, area: Rect, state: &mut PopupState) {
    let popup_area = state.area().unwrap_or_default();
    let text = format!("Popup area: {popup_area:?}");
    let paragraph = Paragraph::new(text).style(Style::new().white().on_black());
    frame.render_widget(paragraph, area);
}

fn handle_events(popup: &mut PopupState, exit: &mut bool) -> Result<()> {
    if let Ok(event) = crossterm::event::read() {
        match event {
            crossterm::event::Event::Key(key) => {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    *exit = true;
                }
            }
            crossterm::event::Event::Mouse(event) => {
                popup.handle_mouse_event(event);
            }
            _ => {}
        }
    }
    Ok(())
}
