use color_eyre::Result;
use lipsum::lipsum;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::{Constraint, Frame, Layout, Rect, Style, Stylize, Text},
    widgets::{Paragraph, Wrap},
};
use tui_popup::{Popup, PopupState};

mod terminal;

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::default();
    while !app.should_exit {
        terminal.draw(|frame| app.render(frame))?;
        app.handle_events()?;
    }
    terminal::restore()?;
    Ok(())
}

#[derive(Default)]
struct App {
    popup: PopupState,
    should_exit: bool,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let [background_area, status_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.size());

        let background = Self::background(background_area);
        frame.render_widget(background, background_area);

        let popup = Self::popup_widget();
        frame.render_stateful_widget_ref(popup, background_area, &mut self.popup);

        // must be called after rendering the popup widget as it relies on the popup area being set
        let status_bar = self.status_bar();
        frame.render_widget(status_bar, status_area);
    }

    fn background(area: Rect) -> Paragraph<'static> {
        let lorem_ipsum = lipsum(area.area() as usize / 5);
        Paragraph::new(lorem_ipsum)
            .wrap(Wrap { trim: false })
            .dark_gray()
    }

    fn popup_widget() -> Popup<'static, Text<'static>> {
        Popup::new(Text::from_iter([
            "q: exit",
            "r: reset",
            "j: move down",
            "k: move up",
            "h: move left",
            "l: move right",
        ]))
        .title("Popup")
        .style(Style::new().white().on_blue())
    }

    /// Status bar at the bottom of the screen
    ///
    /// Must be called after rendering the popup widget as it relies on the popup area being set
    fn status_bar(&self) -> Paragraph<'static> {
        let popup_area = self.popup.area().unwrap_or_default();
        let text = format!("Popup area: {popup_area:?}");
        Paragraph::new(text).style(Style::new().white().on_black())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) => self.handle_key_event(event),
            Event::Mouse(event) => self.popup.handle_mouse_event(event),
            _ => (),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('r') => self.popup = PopupState::default(),
            // TODO: move handling to PopupState (e.g. move_up, move_down, etc. or move(Move:Up))
            KeyCode::Char('j') | KeyCode::Down => self.popup.move_by(0, 1),
            KeyCode::Char('k') | KeyCode::Up => self.popup.move_by(0, -1),
            KeyCode::Char('h') | KeyCode::Left => self.popup.move_by(-1, 0),
            KeyCode::Char('l') | KeyCode::Right => self.popup.move_by(1, 0),
            _ => {}
        }
    }
}
