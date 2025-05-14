//! A [Ratatui] widget for displaying pretty bar graphs
//!
//! Uses the [Colorgrad] crate for gradient coloring.
//!
//! ![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)
//!
//! ![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)
//!
//! [![Crate badge]][Crate]
//! [![Docs Badge]][Docs]
//! [![License Badge]](./LICENSE-MIT)
//! [![Discord Badge]][Discord]
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui tui-bar-graph
//! ```
//!
//! # Example
//!
//! ```rust
//! use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
//!
//! # fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
//! let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
//! let bar_graph = BarGraph::new(data)
//!     .with_gradient(colorgrad::preset::turbo())
//!     .with_bar_style(BarStyle::Braille)
//!     .with_color_mode(ColorMode::VerticalGradient);
//! frame.render_widget(bar_graph, area);
//! # }
//! ```
//!
//! [Colorgrad]: https://crates.io/crates/colorgrad
//! [Ratatui]: https://crates.io/crates/ratatui
//! [Crate]: https://crates.io/crates/tui-bar-graph
//! [Docs]: https://docs.rs/tui-bar-graph
//! [Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-bar-graph.svg?logo=rust&style=for-the-badge
//! [Docs Badge]: https://img.shields.io/docsrs/tui-bar-graph?logo=rust&style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/tui-bar-graph.svg?style=for-the-badge
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

use colorgrad::Gradient;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;
use strum::{Display, EnumString};

// Each side (left/right) has 5 possible heights (0-4)
const BRAILLE_PATTERNS: [[&str; 5]; 5] = [
    // Right height 0-4 (columns) for left height 0 (row)
    ["⠀", "⢀", "⢠", "⢰", "⢸"],
    // Right height 0-4 (columns) for left height 1 (row)
    ["⡀", "⣀", "⣠", "⣰", "⣸"],
    // Right height 0-4 (columns) for left height 2 (row)
    ["⡄", "⣄", "⣤", "⣴", "⣼"],
    // Right height 0-4 (columns) for left height 3 (row)
    ["⡆", "⣆", "⣦", "⣶", "⣾"],
    // Right height 0-4 (columns) for left height 4 (row)
    ["⡇", "⣇", "⣧", "⣷", "⣿"],
];

/// A widget for displaying a bar graph.
///
/// The bars can be colored using a gradient, and can be rendered using either solid blocks or
/// braille characters for a more granular appearance.
///
/// # Example
///
/// ```rust
/// use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
///
/// # fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
/// let bar_graph = BarGraph::new(data)
///     .with_gradient(colorgrad::preset::turbo())
///     .with_bar_style(BarStyle::Braille)
///     .with_color_mode(ColorMode::VerticalGradient);
/// frame.render_widget(bar_graph, area);
/// # }
/// ```
pub struct BarGraph {
    /// The data to display as bars.
    data: Vec<f64>,

    /// The maximum value to display.
    max: Option<f64>,

    /// The minimum value to display.
    min: Option<f64>,

    /// A gradient to use for coloring the bars.
    gradient: Option<Box<dyn Gradient>>,

    /// The direction of the gradient coloring.
    color_mode: ColorMode,

    /// The style of bar to render.
    bar_style: BarStyle,
}

/// The direction of the gradient coloring.
///
/// - `Solid`: Each bar has a single color based on its value.
/// - `Gradient`: Each bar is gradient-colored from bottom to top.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorMode {
    /// Each bar has a single color based on its value.
    Solid,
    /// Each bar is gradient-colored from bottom to top.
    #[default]
    VerticalGradient,
}

/// The style of bar to render.
///
/// - `Solid`: Render bars using the full block character '█'.
/// - `Braille`: Render bars using braille characters for a more granular appearance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum BarStyle {
    /// Render bars using the full block character '█'.
    Solid,
    /// Render bars using braille characters for more granular representation.
    #[default]
    Braille,
}

impl BarGraph {
    /// Creates a new bar graph with the given data.
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            max: None,
            min: None,
            gradient: None,
            color_mode: ColorMode::default(),
            bar_style: BarStyle::default(),
        }
    }

    /// Sets the gradient to use for coloring the bars.
    ///
    /// If `None`, the bars are colored with the default foreground color.
    ///
    /// See the [colorgrad] crate for information on creating gradients. Note that the default
    /// domain (range) of the gradient is [0, 1], so you may need to scale your data to fit this
    /// range, or modify the gradient's domain to fit your data.
    pub fn with_gradient<T: Gradient + 'static>(mut self, gradient: T) -> Self {
        self.gradient = Some(Box::new(gradient));
        self
    }

    /// Sets the maximum value to display.
    ///
    /// Values greater than this will be clamped to this value. If `None`, the maximum value is
    /// calculated from the data.
    pub fn with_max(mut self, max: impl Into<Option<f64>>) -> Self {
        self.max = max.into();
        self
    }

    /// Sets the minimum value to display.
    ///
    /// Values less than this will be clamped to this value. If `None`, the minimum value is
    /// calculated from the data.
    pub fn with_min(mut self, min: impl Into<Option<f64>>) -> Self {
        self.min = min.into();
        self
    }

    /// Sets the color mode for the bars.
    ///
    /// The default is `ColorMode::VerticalGradient`.
    ///
    /// - `Solid`: Each bar has a single color based on its value.
    /// - `Gradient`: Each bar is gradient-colored from bottom to top.
    pub fn with_color_mode(mut self, color: ColorMode) -> Self {
        self.color_mode = color;
        self
    }

    /// Sets the style of the bars.
    ///
    /// The default is `BarStyle::Braille`.
    ///
    /// - `Solid`: Render bars using the full block character '█'.
    /// - `Braille`: Render bars using braille characters for more granular representation.
    pub fn with_bar_style(mut self, style: BarStyle) -> Self {
        self.bar_style = style;
        self
    }

    /// Renders the graph using solid blocks (█).
    fn render_solid(&self, area: Rect, buf: &mut Buffer, min: f64, max: f64) {
        let range = max - min;
        for (&value, column) in self.data.iter().zip(area.columns()) {
            let normalized = (value - min) / range;
            let column_height = (normalized * area.height as f64).ceil() as usize;
            for (i, row) in column.rows().rev().enumerate().take(column_height) {
                let color = self.color_for(area, min, range, value, i);
                buf[row].set_symbol("█").set_fg(color);
            }
        }
    }

    /// Renders the graph using braille characters.
    fn render_braille(&self, area: Rect, buf: &mut Buffer, min: f64, max: f64) {
        let range = max - min;
        const DOTS_PER_ROW: usize = 4;
        let row_count = area.height;
        let total_dots = row_count as usize * DOTS_PER_ROW;

        for (chunk, column) in self
            .data
            .chunks(2)
            .zip(area.columns())
            .take(area.width as usize)
        {
            let left_value = chunk.get(0).cloned().unwrap_or(min);
            let right_value = chunk.get(1).cloned().unwrap_or(min);

            let left_normalized = (left_value - min) / range;
            let right_normalized = (right_value - min) / range;

            let left_total_dots = (left_normalized * total_dots as f64).round() as usize;
            let right_total_dots = (right_normalized * total_dots as f64).round() as usize;

            let column_height = (left_total_dots.max(right_total_dots) as f64 / DOTS_PER_ROW as f64)
                .ceil() as usize;

            for (row_index, row) in column.rows().rev().enumerate().take(column_height) {
                // TODO midpoint is stablized in 1.87 https://github.com/rust-lang/rust/pull/134340
                // let value = f64::midpoint(left_value, right_value);
                let value = (left_value + right_value) / 2.0;
                let color = self.color_for(area, min, max, value, row_index);

                let dots_below = row_index * DOTS_PER_ROW;
                let left_dots = left_total_dots.saturating_sub(dots_below).min(4);
                let right_dots = right_total_dots.saturating_sub(dots_below).min(4);

                let symbol = BRAILLE_PATTERNS[left_dots][right_dots];
                buf[row].set_symbol(symbol).set_fg(color);
            }
        }
    }

    fn color_for(&self, area: Rect, min: f64, max: f64, value: f64, row: usize) -> Color {
        let color_value = match self.color_mode {
            ColorMode::Solid => value,
            ColorMode::VerticalGradient => min + row as f64 / area.height as f64 * (max - min),
        };
        self.gradient
            .as_ref()
            .map(|gradient| {
                let color = gradient.at(color_value as f32);
                let rgba = color.to_rgba8();
                // TODO this can be changed to .into() in ratatui 0.30
                Color::Rgb(rgba[0], rgba[1], rgba[2])
            })
            .unwrap_or(Color::Reset)
    }
}

impl Widget for BarGraph {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // f64 doesn't impl Ord because NaN != NaN, so we use fold instead of iter::max/min
        let min = self
            .min
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::INFINITY, f64::min));
        let max = self
            .max
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        let max = max.max(min + f64::EPSILON); // avoid division by zero if min == max
        match self.bar_style {
            BarStyle::Solid => self.render_solid(area, buf, min, max),
            BarStyle::Braille => self.render_braille(area, buf, min, max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let bar_graph = BarGraph::new(data);

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        bar_graph.render(Rect::new(0, 0, 10, 10), &mut buf);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "  ⢸       ",
                "  ⢸       ",
                "  ⣿       ",
                "  ⣿       ",
                " ⢸⣿       ",
                " ⢸⣿       ",
                " ⣿⣿       ",
                " ⣿⣿       ",
                "⢸⣿⣿       ",
                "⢸⣿⣿       ",
            ])
        );
    }
}
