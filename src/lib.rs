use std::cmp::min;

use derive_builder::Builder;
use font8x8::UnicodeFonts;
use itertools::Itertools;
use ratatui::{prelude::*, text::StyledGrapheme, widgets::Widget};

/// Displays one or more lines of text using 8x8 pixel characters.
///
/// The text is rendered using the [font8x8](https://crates.io/crates/font8x8) crate.
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::*;
/// use tui_big_text::BigTextBuilder;
///
/// BigTextBuilder::default()
///     .style(Style::new().white())
///     .lines(vec![
///         "Hello".red().into(),
///         "World".blue().into(),
///         "=====".into(),
///     ])
///    .build();
/// ```
///
/// Renders:
///
/// ```plain
/// ██  ██           ███     ███
/// ██  ██            ██      ██
/// ██  ██   ████     ██      ██     ████
/// ██████  ██  ██    ██      ██    ██  ██
/// ██  ██  ██████    ██      ██    ██  ██
/// ██  ██  ██        ██      ██    ██  ██
/// ██  ██   ████    ████    ████    ████
///
/// ██   ██                  ███       ███
/// ██   ██                   ██        ██
/// ██   ██  ████   ██ ███    ██        ██
/// ██ █ ██ ██  ██   ███ ██   ██     █████
/// ███████ ██  ██   ██  ██   ██    ██  ██
/// ███ ███ ██  ██   ██       ██    ██  ██
/// ██   ██  ████   ████     ████    ███ ██
///
///  ███ ██  ███ ██  ███ ██  ███ ██  ███ ██
/// ██ ███  ██ ███  ██ ███  ██ ███  ██ ███
/// ```
#[derive(Debug, Builder, Clone, PartialEq, Eq, Hash)]
pub struct BigText<'a> {
    /// The text to display
    #[builder(setter(into))]
    lines: Vec<Line<'a>>,

    /// The style of the widget
    ///
    /// Defaults to `Style::default()`
    #[builder(default)]
    style: Style,
}

impl Widget for BigText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_lines(area, buf);
    }
}

impl BigText<'_> {
    fn render_lines(&self, area: Rect, buf: &mut Buffer) {
        let layout = Self::layout(area);
        for (line, line_layout) in self.lines.iter().zip(layout) {
            for (g, cell) in line.styled_graphemes(self.style).zip(line_layout) {
                Self::render_symbol(g, cell, buf);
            }
        }
    }

    /// Chunk the area into as many 8x8 cells as possible returned as a 2D iterator of `Rect`s
    /// representing the rows of cells.
    fn layout(area: Rect) -> impl IntoIterator<Item = impl IntoIterator<Item = Rect>> {
        (0..area.height)
            .step_by(8)
            .map(|y| {
                (0..area.width)
                    .step_by(8)
                    .map(|x| {
                        let xx = x + area.x;
                        let yy = y + area.y;
                        let width = min(area.width - x, 8);
                        let height = min(area.height - y, 8);
                        Rect::new(xx, yy, width, height)
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    /// Render a single grapheme into a cell by looking up the corresponding 8x8 bitmap in the
    /// `BITMAPS` array and setting the corresponding cells in the buffer.
    fn render_symbol(grapheme: StyledGrapheme, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, grapheme.style);
        let c = grapheme.symbol.chars().next().unwrap(); // TODO: handle multi-char graphemes
        if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
            for (dy, glyph_row) in glyph.iter().enumerate() {
                let y = area.y + dy as u16;
                if y >= area.bottom() {
                    break;
                }
                for dx in 0..8 {
                    let x = area.x + dx;
                    if x >= area.right() {
                        break;
                    }
                    let cell = buf.get_mut(area.x + dx, area.y + dy as u16);
                    match glyph_row & (1 << dx) {
                        0 => cell.set_symbol(" "),
                        _ => cell.set_symbol("█"),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::assert_buffer_eq;

    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn build() -> Result<()> {
        let lines = vec![Line::from(vec!["Hello".red(), "World".blue()])];
        let style = Style::new().green();
        assert_eq!(
            BigTextBuilder::default()
                .lines(lines.clone())
                .style(style)
                .build()?,
            BigText { lines, style }
        );
        Ok(())
    }

    #[test]
    fn render_single_line() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 8));
        big_text.render(buf.area, &mut buf);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                " ████     ██                     ███            ████      ██                    ",
                "██  ██                            ██             ██                             ",
                "███      ███    █████    ███ ██   ██     ████    ██      ███    █████    ████   ",
                " ███      ██    ██  ██  ██  ██    ██    ██  ██   ██       ██    ██  ██  ██  ██  ",
                "   ███    ██    ██  ██  ██  ██    ██    ██████   ██   █   ██    ██  ██  ██████  ",
                "██  ██    ██    ██  ██   █████    ██    ██       ██  ██   ██    ██  ██  ██      ",
                " ████    ████   ██  ██      ██   ████    ████   ███████  ████   ██  ██   ████   ",
                "                        █████                                                   ",
            ])
        );
        Ok(())
    }

    #[test]
    fn render_truncated() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 6));
        big_text.render(buf.area, &mut buf);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "██████                                             █               ███",
                "█ ██ █                                            ██                ██",
                "  ██    ██ ███  ██  ██  █████    ████    ████    █████   ████       ██",
                "  ██     ███ ██ ██  ██  ██  ██  ██  ██      ██    ██    ██  ██   █████",
                "  ██     ██  ██ ██  ██  ██  ██  ██       █████    ██    ██████  ██  ██",
                "  ██     ██     ██  ██  ██  ██  ██  ██  ██  ██    ██ █  ██      ██  ██",
            ])
        );
        Ok(())
    }

    #[test]
    fn render_multiple_lines() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 16));
        big_text.render(buf.area, &mut buf);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "██   ██          ███       █      ██    ",
                "███ ███           ██      ██            ",
                "███████ ██  ██    ██     █████   ███    ",
                "███████ ██  ██    ██      ██      ██    ",
                "██ █ ██ ██  ██    ██      ██      ██    ",
                "██   ██ ██  ██    ██      ██ █    ██    ",
                "██   ██  ███ ██  ████      ██    ████   ",
                "                                        ",
                "████      ██                            ",
                " ██                                     ",
                " ██      ███    █████    ████    █████  ",
                " ██       ██    ██  ██  ██  ██  ██      ",
                " ██   █   ██    ██  ██  ██████   ████   ",
                " ██  ██   ██    ██  ██  ██          ██  ",
                "███████  ████   ██  ██   ████   █████   ",
                "                                        ",
            ])
        );
        Ok(())
    }

    #[test]
    fn render_widget_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 48, 8));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            " ████      █             ███               ███  ",
            "██  ██    ██              ██                ██  ",
            "███      █████  ██  ██    ██     ████       ██  ",
            " ███      ██    ██  ██    ██    ██  ██   █████  ",
            "   ███    ██    ██  ██    ██    ██████  ██  ██  ",
            "██  ██    ██ █   █████    ██    ██      ██  ██  ",
            " ████      ██       ██   ████    ████    ███ ██ ",
            "                █████                           ",
        ]);
        expected.set_style(Rect::new(0, 0, 48, 8), Style::new().bold());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_line_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 24));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "██████             ███                  ",
            " ██  ██             ██                  ",
            " ██  ██  ████       ██                  ",
            " █████  ██  ██   █████                  ",
            " ██ ██  ██████  ██  ██                  ",
            " ██  ██ ██      ██  ██                  ",
            "███  ██  ████    ███ ██                 ",
            "                                        ",
            "  ████                                  ",
            " ██  ██                                 ",
            "██      ██ ███   ████    ████   █████   ",
            "██       ███ ██ ██  ██  ██  ██  ██  ██  ",
            "██  ███  ██  ██ ██████  ██████  ██  ██  ",
            " ██  ██  ██     ██      ██      ██  ██  ",
            "  █████ ████     ████    ████   ██  ██  ",
            "                                        ",
            "██████   ███                            ",
            " ██  ██   ██                            ",
            " ██  ██   ██    ██  ██   ████           ",
            " █████    ██    ██  ██  ██  ██          ",
            " ██  ██   ██    ██  ██  ██████          ",
            " ██  ██   ██    ██  ██  ██              ",
            "██████   ████    ███ ██  ████           ",
            "                                        ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 8), Style::new().red());
        expected.set_style(Rect::new(0, 8, 40, 8), Style::new().green());
        expected.set_style(Rect::new(0, 16, 32, 8), Style::new().blue());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }
}
