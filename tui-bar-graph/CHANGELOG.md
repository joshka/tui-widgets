# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-11-02

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Support boxed gradients ([#66](https://github.com/joshka/tui-widgets/issues/66))
  > This patch adds support for boxed gradients in the `BarGraph` widget.
  > This makes it possible to choose gradients of different types at runtime
  > without having to change the type of the `BarGraph` struct.

- *(tui-bar-graph)* Add Quadrant style ([#80](https://github.com/joshka/tui-widgets/issues/80))
  > This style uses the block drawing 2x2 quadrant characters.
  > In contrast to the braille style, it renders solid rather than dots.
  > In contrast to the solid style, it renders two columns and rows per bar.
  >
  > ![Quadrant Magma](https://vhs.charm.sh/vhs-1rx6XQ9mLiO8qybSBXRGwn.gif)

### 🐛 Bug Fixes

- Broken bar graph test

- Clippy lints ([#81](https://github.com/joshka/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- Use f64:midpoint ([#83](https://github.com/joshka/tui-widgets/issues/83))
  > MSRV is now 1.87

### 🚜 Refactor

- Simplify color / gradient handling logic

### 🎨 Styling

- Add rustfmt and reformat code

### Other

- Bump msrv to 1.82.0 ([#74](https://github.com/joshka/tui-widgets/issues/74))


## [tui-bar-graph-v0.1.1] - 2025-03-05

### 🚜 Refactor

- Simplify BarGraph rendering logic

## [tui-bar-graph-v0.1.0] - 2025-03-04

### 🚀 Features

- Add new tui-bar-graph crate ([#63](https://github.com/joshka/tui-widgets/issues/63))
  > ![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)
  >
  > ![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)
  >
  > ```rust
  > use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
  >
  > let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
  > let bar_graph = BarGraph::new(data)
  >     .with_gradient(colorgrad::preset::turbo())
  >     .with_bar_style(BarStyle::Braille)
  >     .with_color_mode(ColorMode::VerticalGradient);
  > frame.render_widget(bar_graph, area);
  > ```
