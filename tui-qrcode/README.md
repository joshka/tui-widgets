# TUI QR Code

<!-- cargo-rdme start -->

TUI QR Code is a library for rendering QR codes in a terminal using the [Ratatui] crate.

[![Crate badge]][tui-qrcode]
[![Docs.rs Badge]][API Docs]
[![Deps.rs Badge]][Dependency Status]
[![License Badge]](./LICENSE-MIT)
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

![Demo](https://vhs.charm.sh/vhs-nUpcmCP1igCcGoJ5iio07.gif)

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

[Ratatui]: https://crates.io/crates/ratatui
[Crate badge]: https://img.shields.io/crates/v/tui-qrcode.svg?style=for-the-badge
[tui-qrcode]: https://crates.io/crates/tui-qrcode
[Docs.rs Badge]: https://img.shields.io/badge/docs.rs-tui--qrcode-blue?style=for-the-badge
[API Docs]: https://docs.rs/tui-qrcode
[Deps.rs Badge]: https://deps.rs/repo/github/joshka/tui-qrcode/status.svg?style=for-the-badge
[Dependency Status]: https://deps.rs/repo/github/joshka/tui-qrcode
[License Badge]: https://img.shields.io/crates/l/tui-qrcode?style=for-the-badge
[Discord Badge]:
    https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[GitHub Repository]: https://github.com/joshka/tui-widgets
[Examples]: https://github.com/joshka/tui-widgets/tree/main/tui-qrcode/examples
[Changelog]: https://github.com/joshka/tui-widgets/blob/main/tui-qrcode/CHANGELOG.md
[Contributing]: https://github.com/joshka/tui-widgets/blob/main/CONTRIBUTING.md

<!-- cargo-rdme end -->
