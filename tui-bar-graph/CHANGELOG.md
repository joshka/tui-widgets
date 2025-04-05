# Changelog

All notable changes to this project will be documented in this file.

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
