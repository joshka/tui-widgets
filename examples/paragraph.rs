/// Demonstrates how to use the `Popup` widget with a `Paragraph` as the body.
use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use lipsum::lipsum;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};
use tui_popup::{Popup, SizedWrapper};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = init_terminal()?;

    let mut scroll = 0;
    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let lorem_ipsum = lipsum(area.area() as usize / 5);
            let background = Paragraph::new(lorem_ipsum)
                .wrap(Wrap { trim: false })
                .dark_gray();
            frame.render_widget(background, area);
            let lines: Text = (0..10).map(|i| Span::raw(format!("Line {i}"))).collect();
            let paragraph = Paragraph::new(lines).scroll((scroll, 0));
            let sized_paragraph = SizedWrapper {
                inner: paragraph,
                width: 21,
                height: 5,
            };
            let popup = Popup::new("scroll: ↑/↓ quit: Esc", sized_paragraph)
                .style(Style::new().white().on_blue());
            frame.render_widget_ref(popup, area);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => scroll = scroll.saturating_add(1),
                KeyCode::Char('k') | KeyCode::Up => scroll = scroll.saturating_sub(1),
                _ => {}
            }
        }
    }
    restore_terminal()?;
    Ok(())
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
