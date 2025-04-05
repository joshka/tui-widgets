use clap::Parser;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use rand::Rng;
use ratatui::{DefaultTerminal, Frame};
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

#[derive(Debug, Parser)]
struct Args {
    /// The style of bar to render (solid or braille)
    #[arg(default_value_t = BarStyle::Braille)]
    bar_style: BarStyle,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &args);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, args: &Args) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, args))?;
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

fn render(frame: &mut Frame, args: &Args) {
    let width = match args.bar_style {
        BarStyle::Solid => frame.area().width as usize,
        BarStyle::Braille => frame.area().width as usize * 2,
    };
    let mut data = vec![0.0; width];
    rand::rng().fill(&mut data[..]);

    let bar_graph = BarGraph::new(data)
        .with_gradient(colorgrad::preset::turbo())
        .with_color_mode(ColorMode::VerticalGradient)
        .with_bar_style(args.bar_style);
    frame.render_widget(bar_graph, frame.area());
}
