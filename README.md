# tui-big-text

[![Crate badge]][tui-big-text]
[![Docs.rs Badge]][API Docs]
[![Deps.rs Badge]][Dependency Status]<br>
[![License Badge]](./LICENSE-MIT)
[![Codecov.io Badge]][Code Coverage]
[![Discord Badge]][Ratatui Discord]

<!-- cargo-rdme start -->

[tui-big-text] is a rust crate that renders large pixel text as a [Ratatui] widget using the
glyphs from the [font8x8] crate.

![Hello World example](https://vhs.charm.sh/vhs-2UxNc2SJgiNqHoowbsXAMW.gif)

## Installation

```shell
cargo add ratatui tui-big-text
```

## Usage

Create a [`BigText`] widget using `BigTextBuilder` and pass it to [`Frame::render_widget`] to
render be rendered. The builder allows you to customize the [`Style`] of the widget and the
[`PixelSize`] of the glyphs. The [`PixelSize`] can be used to control how many character cells
are used to represent a single pixel of the 8x8 font.

## Example

```rust
use anyhow::Result;
use ratatui::prelude::*;
use tui_big_text::{BigTextBuilder, PixelSize};

fn render(frame: &mut Frame) -> Result<()> {
    let big_text = BigTextBuilder::default()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec![
            "Hello".red().into(),
            "World".white().into(),
            "~~~~~".into(),
        ])
        .build()?;
    frame.render_widget(big_text, frame.size());
    Ok(())
}
```

[tui-big-text]: https://crates.io/crates/tui-big-text
[Ratatui]: https://crates.io/crates/ratatui
[font8x8]: https://crates.io/crates/font8x8
[`BigText`]: https://docs.rs/tui-big-text/latest/tui_big_text/struct.BigText.html
[`PixelSize`]: https://docs.rs/tui-big-text/latest/tui_big_text/enum.PixelSize.html
[`Frame::render_widget`]: ratatui::Frame::render_widget
[`Style`]: ratatui::style::Style

<!-- cargo-rdme end -->

## License

Copyright (c) 2023 Josh McKinney

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

[Crate badge]: https://img.shields.io/crates/v/tui-big-text?logo=rust&style=for-the-badge
[Docs.rs Badge]: https://img.shields.io/docsrs/tui-big-text?logo=rust&style=for-the-badge
[Deps.rs Badge]: https://deps.rs/repo/github/joshka/tui-big-text/status.svg?style=for-the-badge
[License Badge]: https://img.shields.io/crates/l/tui-big-text?style=for-the-badge
[Codecov.io Badge]: https://img.shields.io/codecov/c/github/joshka/tui-big-text?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

[API Docs]: https://docs.rs/crate/tui-big-text/
[Dependency Status]: https://deps.rs/repo/github/joshka/tui-big-text
[Code Coverage]: https://app.codecov.io/gh/joshka/tui-big-text
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
