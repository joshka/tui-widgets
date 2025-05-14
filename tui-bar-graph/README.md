# Tui-bar-graph

<!-- cargo-rdme start -->

A [Ratatui] widget for displaying pretty bar graphs

Uses the [Colorgrad] crate for gradient coloring.

![Braille Rainbow](https://vhs.charm.sh/vhs-1sx9Ht6NzU6e28Cl51jJVv.gif)
![Solid Plasma](https://vhs.charm.sh/vhs-7pWuLtZpzrz1OVD04cMt1a.gif)

<details><summary>More examples</summary>

![Braille Magma](https://vhs.charm.sh/vhs-4RDwcz9DApA90iJYMQXHXd.gif)
![Braille Viridis](https://vhs.charm.sh/vhs-5ylsZAdKGPiHUYboOpZFZL.gif)
![Solid Inferno](https://vhs.charm.sh/vhs-4z1gbmJ50KGz2TPej3mnVf.gif)
![Solid Sinebow](https://vhs.charm.sh/vhs-63aAmMhcfMT8CnWCV20dsn.gif)

</details>

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![License Badge]](./LICENSE-MIT)
[![Discord Badge]][Discord]

## Installation

```shell
cargo add colorgrad ratatui tui-bar-graph
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
