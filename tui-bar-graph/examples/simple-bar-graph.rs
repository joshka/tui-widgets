use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use rand::Rng;
use ratatui::{DefaultTerminal, Frame};
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            })
        ) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let data_count = frame.area().width as usize * 2;
    let mut data = vec![0.0; data_count];
    rand::rng().fill(&mut data[..]);
    let gradient = colorgrad::preset::rainbow();
    let bar_graph = BarGraph::new(data)
        .with_gradient(gradient)
        .with_color_mode(ColorMode::VerticalGradient)
        .with_bar_style(BarStyle::Braille);
    frame.render_widget(bar_graph, frame.area());
}
