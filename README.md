# tui-big-text

[![Crates.io](https://img.shields.io/crates/v/tui-big-text?logo=rust&style=for-the-badge)](https://crates.io/crates/tui-big-text)
[![License](https://img.shields.io/crates/l/tui-big-text?style=for-the-badge)](./LICENSE-MIT)
[![Docs.rs](https://img.shields.io/docsrs/tui-big-text?logo=rust&style=for-the-badge)](https://docs.rs/crate/tui-big-text/)
[![Dependency Status](https://deps.rs/repo/github/joshka/tui-big-text/status.svg?style=for-the-badge)](https://deps.rs/repo/github/joshka/tui-big-text)
[![Codecov](https://img.shields.io/codecov/c/github/joshka/tui-big-text?logo=codecov&style=for-the-badge&token=BAQ8SOKEST)](https://app.codecov.io/gh/joshka/tui-big-text)
[![Discord](https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge)](https://discord.gg/pMCEU9hNEj)

<!-- cargo-rdme start -->

[tui-big-text](https://crates.io/crates/tui-big-text) is a rust crate that renders large pixel
text as a [ratatui](https://crates.io/crates/ratatui) widget using the glyphs from the
[font8x8](https://crates.io/crates/font8x8) crate.

## Installation

```shell
cargo add ratatui tui-big-text
```

## Example

```rust
use anyhow::Result;
use ratatui::prelude::*;
use tui_big_text::BigTextBuilder;

fn render(frame: &mut Frame) -> Result<()> {
    let big_text = BigTextBuilder::default()
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

![Example Made with VHS](https://vhs.charm.sh/vhs-1dIs1zoxqGwkP60aMcfpR8.gif)

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
