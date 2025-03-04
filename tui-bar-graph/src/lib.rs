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
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
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
///    .with_gradient(colorgrad::preset::turbo())
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

    /// Gets the color of a bar based on its value.
    fn color(&self, value: f64) -> Color {
        if let Some(gradient) = &self.gradient {
            let color = gradient.at(value as f32);
            to_ratatui_color(&color)
        } else {
            Color::Reset
        }
    }

    /// Renders the graph using solid blocks (█).
    fn render_solid(&self, area: Rect, buf: &mut Buffer, min: f64, max: f64) {
        let range = max - min;

        for (i, &value) in self.data.iter().enumerate() {
            if i == area.width as usize {
                break;
            }
            let normalized = (value - min) / range;
            let height = (normalized * area.height as f64).round() as u16;
            for y in 0..area.height {
                if y < height {
                    let cell = &mut buf[(area.left() + i as u16, area.bottom() - y - 1)];
                    cell.set_symbol("█");

                    let color_value = match self.color_mode {
                        ColorMode::Solid => value,
                        ColorMode::VerticalGradient => {
                            // For gradient coloring, we calculate the color based on the y position
                            // within the bar (bottom to top gradient)
                            let y_normalized = y as f64 / area.height as f64;
                            min + y_normalized * range
                        }
                    };
                    cell.set_fg(self.color(color_value));
                }
            }
        }
    }

    /// Renders the graph using braille characters.
    fn render_braille(&self, area: Rect, buf: &mut Buffer, min: f64, max: f64) {
        // Each braille character represents 4 vertical dots
        const DOTS_PER_ROW: usize = 4;

        let range = max - min;
        let row_count = area.height;
        let total_dots = row_count as usize * DOTS_PER_ROW;

        for (i, chunk) in self.data.chunks(2).enumerate() {
            if i >= area.width as usize {
                break;
            }
            let left_value = chunk.get(0).cloned().unwrap_or(min);
            let right_value = chunk.get(1).cloned().unwrap_or(min);

            let left_normalized = ((left_value - min) / range).clamp(0.0, 1.0);
            let right_normalized = ((right_value - min) / range).clamp(0.0, 1.0);

            // Calculate total height in dots (scaled to fill the entire height)
            let left_total_dots = (left_normalized * total_dots as f64).round() as usize;
            let right_total_dots = (right_normalized * total_dots as f64).round() as usize;

            // Render each row of braille characters
            for row in 0..row_count {
                // Calculate row position (from bottom to top)
                let y_pos = area.bottom() - 1 - row;

                // Calculate the base dot index for this row (counting from bottom)
                let row_base = row * DOTS_PER_ROW as u16;

                // Calculate dots for this braille character
                let left_height = (left_total_dots as u16).saturating_sub(row_base).min(4) as usize;
                let right_height =
                    (right_total_dots as u16).saturating_sub(row_base).min(4) as usize;

                // Place the braille character
                let symbol = BRAILLE_PATTERNS[left_height][right_height];
                let cell = &mut buf[(area.left() + i as u16, y_pos)];
                cell.set_symbol(symbol);

                let color_value = match self.color_mode {
                    ColorMode::Solid => {
                        // Use the average of the two values for coloring
                        (left_value + right_value) / 2.0
                    }
                    ColorMode::VerticalGradient => {
                        // For gradient coloring, calculate position in the overall graph
                        let row_normalized = row as f64 / row_count as f64;
                        min + row_normalized * range
                    }
                };
                cell.set_fg(self.color(color_value));
            }
        }
    }
}

impl Widget for BarGraph {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // f64 doesn't impl Ord because NaN != NaN, so we can't use iter::max/min
        let max = self
            .max
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        let min = self
            .min
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::INFINITY, f64::min));

        match self.bar_style {
            BarStyle::Solid => self.render_solid(area, buf, min, max),
            BarStyle::Braille => self.render_braille(area, buf, min, max),
        }
    }
}

/// Converts a colorgrad color to a ratatui color.
fn to_ratatui_color(color: &colorgrad::Color) -> Color {
    let rgba = color.to_rgba8();
    Color::Rgb(rgba[0], rgba[1], rgba[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let bar_graph = BarGraph::new(data);

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        bar_graph.render(Rect::new(0, 0, 10, 10), &mut buf);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "⠀⠀⡇       ",
                "⠀⠀⡇       ",
                "⠀⢠⡇       ",
                "⠀⢸⡇       ",
                "⠀⢸⡇       ",
                "⠀⣿⡇       ",
                "⠀⣿⡇       ",
                "⢠⣿⡇       ",
                "⢸⣿⡇       ",
                "⢸⣿⡇       ",
            ])
        );
    }
}
