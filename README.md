# tui-big-text

[![Crates.io](https://img.shields.io/crates/v/tui-big-text?logo=rust&style=for-the-badge)](https://crates.io/crates/tui-big-text)
[![License](https://img.shields.io/crates/l/tui-big-text?style=for-the-badge)](./LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/tui-big-text?logo=rust&style=for-the-badge)](https://docs.rs/crate/tui-big-text/)
[![Dependency Status](https://deps.rs/repo/github/joshka/tui-big-text/status.svg?style=for-the-badge)](https://deps.rs/repo/github/joshka/tui-big-text)
[![Codecov](https://img.shields.io/codecov/c/github/joshka/tui-big-text?logo=codecov&style=for-the-badge&token=BAQ8SOKEST)](https://app.codecov.io/gh/joshka/tui-big-text)
[![Discord](https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge)](https://discord.gg/pMCEU9hNEj)

[tui-big-text](https://crates.io/crates/tui-big-text) is a rust crate that renders large pixel
text as a [ratatui](https://crates.io/crates/ratatui) widget using the glyphs from the [font8x8](https://crates.io/crates/font8x8)
crate.

## Installation

```shell
cargo add ratatui tui-big-text
```

## Example

```rust
fn render<B: Backend>(frame: &mut Frame<B>) -> Result<()> {
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
