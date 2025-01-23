use qrcode::QrCode;
use ratatui::{crossterm::event, style::Stylize, DefaultTerminal, Frame};
use tui_qrcode::{Colors, QrCodeWidget};

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
        if matches!(event::read()?, event::Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted).blue();
    frame.render_widget(widget, frame.area());
}
