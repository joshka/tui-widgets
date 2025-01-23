# TUI QR Code

<!-- cargo-rdme start -->

TUI QR Code is a library for rendering QR codes in a terminal using the [ratatui] crate.

## Usage

Add qrcode and tui-qrcode to your Cargo.toml. You can disable the default features of qrcode as
we don't need the code which renders the QR code to an image.

```shell
cargo add qrcode tui-qrcode --no-default-features
```

## Example

This example can be found in the `examples` directory of the repository.

```rust
use qrcode::QrCode;
use ratatui::{crossterm::event, DefaultTerminal, Frame};
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
    let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
    frame.render_widget(widget, frame.area());
}
```

Renders the following QR code:

```text
█████████████████████████████████
█████████████████████████████████
████ ▄▄▄▄▄ █▄ ▄▄▄ ████ ▄▄▄▄▄ ████
████ █   █ █▄▄▄█▀▄██ █ █   █ ████
████ █▄▄▄█ █▀   ▄▀ ███ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄▀▄█ ▀▄▀ █▄▄▄▄▄▄▄████
████ █▄▀▀▀▄▄▀▄▄  ▄█▀▄█▀ █▀▄▀ ████
██████▀█  ▄▀▄▄▀▀ ▄ ▄█ ▄▄█ ▄█▄████
████▄▀▀▀▄▄▄▄▀█▄▄█  ▀ ▀ ▀███▀ ████
████▄▄ ▀█▄▄▀▄▄ ▄█▀█▄▀█▄▀▀ ▄█▄████
████▄▄█▄██▄█ ▄▀▄ ▄█  ▄▄▄ ██▄▀████
████ ▄▄▄▄▄ █▄▄▄▀ ▄ ▀ █▄█ ███ ████
████ █   █ ██ ███  ▄▄ ▄▄ █▀ ▄████
████ █▄▄▄█ █▄▀ ▄█▀█▀ ▄█  ▄█▄▄████
████▄▄▄▄▄▄▄█▄▄█▄▄▄██▄█▄██▄██▄████
█████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀    
```

<!-- cargo-rdme end -->
