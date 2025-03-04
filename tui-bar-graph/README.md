# Tui-bar-graph

<!-- cargo-rdme start -->

A [Ratatui] widget for displaying pretty bar graphs

Uses the [Colorgrad] crate for gradient coloring.

![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)

![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![License Badge]](./LICENSE-MIT)
[![Discord Badge]][Discord]

## Installation

```shell
cargo add ratatui tui-bar-graph
```

## Example

```rust
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
let bar_graph = BarGraph::new(data)
    .with_gradient(colorgrad::preset::turbo())
    .with_bar_style(BarStyle::Braille)
    .with_color_mode(ColorMode::VerticalGradient);
frame.render_widget(bar_graph, area);
```

[Colorgrad]: https://crates.io/crates/colorgrad
[Ratatui]: https://crates.io/crates/ratatui
[Crate]: https://crates.io/crates/tui-bar-graph
[Docs]: https://docs.rs/tui-bar-graph
[Discord]: https://discord.gg/pMCEU9hNEj
[Crate badge]: https://img.shields.io/crates/v/tui-bar-graph.svg?logo=rust&style=for-the-badge
[Docs Badge]: https://img.shields.io/docsrs/tui-bar-graph?logo=rust&style=for-the-badge
[License Badge]: https://img.shields.io/crates/l/tui-bar-graph.svg?style=for-the-badge
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

<!-- cargo-rdme end -->
