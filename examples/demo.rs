use std::io::stdout;

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use lipsum::lipsum;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};
use tui_popup::Popup;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = init_terminal()?;

    terminal.draw(|frame| {
        let area = frame.size();

        let lorem_ipsum = lipsum(area.area() as usize / 5);
        let background = Paragraph::new(lorem_ipsum)
            .wrap(Wrap { trim: false })
            .dark_gray();
        frame.render_widget(background, area);

        let popup = Popup::new("tui-popup demo", "Press any key to exit")
            .style(Style::new().white().on_blue());
        frame.render_widget_ref(popup, area);
    })?;
    while !matches!(event::read()?, Event::Key(_)) {}
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
