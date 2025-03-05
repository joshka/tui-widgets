use color_eyre::Result;
use ratatui::{
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
            KeyModifiers,
        },
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::Size,
    prelude::{Constraint, CrosstermBackend, Frame, Layout, Rect, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Wrap},
    DefaultTerminal, Terminal,
};
use std::io::{self, stdout};
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
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub fn restore() {
    if let Err(err) = try_restore() {
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
    let Size { width, height } = terminal.size()?;
    let mut state = PopupState::new(Rect::new(0, 0, width, height));
    let mut show_help = false;
    let mut exit = false;

    while !exit {
        terminal.draw(|frame| draw(frame, &mut state, show_help))?;
        handle_events(&mut state, &mut show_help, &mut exit)?;
    }
    Ok(())
}

fn draw(frame: &mut Frame, state: &mut PopupState, show_help: bool) {
    let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]);
    let [main_area, status_area] = layout.areas(frame.area());

    render_background(frame, main_area);
    render_main_popup(frame, main_area, state);
    if show_help {
        render_help_popup(frame, main_area);
    }
    render_enhanced_status_bar(frame, status_area, state, show_help);
}

fn render_background(frame: &mut Frame, area: Rect) {
    let stars: String = (0..area.area())
        .map(|i| if i % 7 == 0 { '✧' } else { ' ' })
        .collect();

    let styled_background = Paragraph::new(stars)
        .style(Style::default().dark_gray())
        .wrap(Wrap { trim: false });

    frame.render_widget(styled_background, area);
}

fn render_main_popup(frame: &mut Frame, area: Rect, state: &mut PopupState) {
    let content = Text::from(vec![
        Line::from(vec![
            Span::styled("Welcome to ", Style::default().blue()),
            Span::styled("Interactive Popup!", Style::default().blue().bold()),
        ]),
        Line::raw(""),
        Line::styled("Mouse Controls:", Style::default().yellow()),
        Line::raw("• Click and drag title bar to move"),
        Line::raw("• Drag ⟋ handle to resize"),
        Line::raw("• Click [✕] to close"),
        Line::raw(""),
        Line::styled("Press '?' for keyboard shortcuts", Style::default().dim()),
    ]);

    let wrapped_content =
        KnownSizeWrapper::new(Paragraph::new(content).wrap(Wrap { trim: true }), 40, 10);

    let popup = Popup::new(wrapped_content)
        .title("Interactive Demo")
        .border_set(border::ROUNDED)
        .border_style(Style::default().blue().bold())
        .style(Style::default().white());

    frame.render_stateful_widget(popup, area, state);
}

fn render_help_popup(frame: &mut Frame, area: Rect) {
    let mut help_state = PopupState::new(area);
    help_state.open(area.x + 5, area.y + 2, 35, 12);

    let shortcuts = Text::from(vec![
        Line::styled("Keyboard Shortcuts", Style::default().bold()),
        Line::raw(""),
        Line::raw("Movement:"),
        Line::raw("h/←, j/↓, k/↑, l/→: Move popup"),
        Line::raw("Ctrl + Arrow: Jump to edge"),
        Line::raw(""),
        Line::raw("Other:"),
        Line::raw("?: Toggle this help"),
        Line::raw("r: Reset position"),
        Line::raw("q: Quit"),
    ]);

    let help_popup = Popup::new(shortcuts)
        .title("Help")
        .border_set(border::DOUBLE)
        .border_style(Style::default().yellow())
        .style(Style::default().white());

    frame.render_stateful_widget(help_popup, area, &mut help_state);
}

fn render_enhanced_status_bar(frame: &mut Frame, area: Rect, state: &PopupState, show_help: bool) {
    let pos = state.area().map_or("Hidden".to_string(), |area| {
        format!("x:{} y:{} {}x{}", area.x, area.y, area.width, area.height)
    });

    let mode = if show_help { "Help" } else { "Normal" };
    let status = format!("Position: {} | Mode: {} | Press '?' for help", pos, mode);

    let status_bar = Paragraph::new(status)
        .style(Style::default().black().on_blue())
        .block(Block::default());

    frame.render_widget(status_bar, area);
}

fn handle_events(state: &mut PopupState, show_help: &mut bool, exit: &mut bool) -> Result<()> {
    if let Ok(event) = event::read() {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                handle_key_event(key, state, show_help, exit);
            }
            Event::Mouse(event) => {
                state.handle_mouse_event(event);
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_key_event(
    event: KeyEvent,
    state: &mut PopupState,
    show_help: &mut bool,
    exit: &mut bool,
) {
    let is_ctrl = event.modifiers.contains(KeyModifiers::CONTROL);

    match event.code {
        KeyCode::Char('q') | KeyCode::Esc => *exit = true,
        KeyCode::Char('?') => *show_help = !*show_help,
        KeyCode::Char('r') => state.reset_position(),
        KeyCode::Char('j') | KeyCode::Down => {
            if is_ctrl {
                state.move_to_bottom();
            } else {
                state.move_down(1);
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if is_ctrl {
                state.move_to_top();
            } else {
                state.move_up(1);
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if is_ctrl {
                state.move_to_leftmost();
            } else {
                state.move_left(1);
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if is_ctrl {
                state.move_to_rightmost();
            } else {
                state.move_right(1);
            }
        }
        _ => {}
    }
}
