use std::cmp::min;

use derive_builder::Builder;
use font8x8::UnicodeFonts;
use ratatui::{prelude::*, text::StyledGrapheme, widgets::Widget};

use crate::PixelSize;

/// Displays one or more lines of text using 8x8 pixel characters.
///
/// The text is rendered using the [font8x8](https://crates.io/crates/font8x8) crate.
///
/// Using the `pixel_size` method, you can also chose, how 'big' a pixel should be. Currently a
/// pixel of the 8x8 font can be represented by one full or half (horizontal/vertical/both)
/// character cell of the terminal.
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::*;
/// use tui_big_text::{BigText, PixelSize};
///
/// BigText::builder()
///     .pixel_size(PixelSize::Full)
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
    #[builder(default, setter(into))]
    style: Style,

    /// The size of single glyphs
    ///
    /// Defaults to `BigTextSize::default()` (=> BigTextSize::Full)
    #[builder(default)]
    pixel_size: PixelSize,

    /// The horizontal alignmnet of the text
    ///
    /// Defaults to `Alignment::default()` (=> Alignment::Left)
    #[builder(default)]
    alignment: Alignment,
}

impl BigText<'static> {
    /// Create a new [`BigTextBuilder`] to configure a [`BigText`] widget.
    pub fn builder() -> BigTextBuilder<'static> {
        BigTextBuilder::default()
    }
}

impl BigTextBuilder<'_> {
    /// Set the alignment of the text.
    pub fn left_aligned(&mut self) -> &mut Self {
        self.alignment(Alignment::Left)
    }

    /// Set the alignment of the text.
    pub fn right_aligned(&mut self) -> &mut Self {
        self.alignment(Alignment::Right)
    }

    /// Set the alignment of the text.
    pub fn centered(&mut self) -> &mut Self {
        self.alignment(Alignment::Center)
    }
}

impl Widget for BigText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = layout(area, &self.pixel_size, self.alignment, &self.lines);
        for (line, line_layout) in self.lines.iter().zip(layout) {
            for (g, cell) in line.styled_graphemes(self.style).zip(line_layout) {
                render_symbol(g, cell, buf, &self.pixel_size);
            }
        }
    }
}

/// Chunk the area into as many x*y cells as possible returned as a 2D iterator of `Rect`s
/// representing the rows of cells. The size of each cell depends on given font size
fn layout<'a>(
    area: Rect,
    pixel_size: &PixelSize,
    alignment: Alignment,
    lines: &'a [Line<'a>],
) -> impl IntoIterator<Item = impl IntoIterator<Item = Rect>> + 'a {
    let (step_x, step_y) = pixel_size.pixels_per_cell();
    let width = 8_u16.div_ceil(step_x);
    let height = 8_u16.div_ceil(step_y);

    (area.top()..area.bottom())
        .step_by(height as usize)
        .zip(lines.iter())
        .map(move |(y, line)| {
            let offset = get_alignment_offset(area.width, width, alignment, line);
            (area.left() + offset..area.right())
                .step_by(width as usize)
                .map(move |x| {
                    let width = min(area.right() - x, width);
                    let height = min(area.bottom() - y, height);
                    Rect::new(x, y, width, height)
                })
        })
}

fn get_alignment_offset<'a>(
    area_width: u16,
    letter_width: u16,
    alignment: Alignment,
    line: &'a Line<'a>,
) -> u16 {
    let big_line_width = line.width() as u16 * letter_width;
    match alignment {
        Alignment::Center => (area_width / 2).saturating_sub(big_line_width / 2),
        Alignment::Right => area_width.saturating_sub(big_line_width),
        Alignment::Left => 0,
    }
}

/// Render a single grapheme into a cell by looking up the corresponding 8x8 bitmap in the
/// `BITMAPS` array and setting the corresponding cells in the buffer.
fn render_symbol(grapheme: StyledGrapheme, area: Rect, buf: &mut Buffer, pixel_size: &PixelSize) {
    buf.set_style(area, grapheme.style);
    let c = grapheme.symbol.chars().next().unwrap(); // TODO: handle multi-char graphemes
    if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
        render_glyph(glyph, area, buf, pixel_size);
    }
}

/// Render a single 8x8 glyph into a cell by setting the corresponding cells in the buffer.
fn render_glyph(glyph: [u8; 8], area: Rect, buf: &mut Buffer, pixel_size: &PixelSize) {
    let (step_x, step_y) = pixel_size.pixels_per_cell();

    let glyph_vertical_index = (0..glyph.len()).step_by(step_y as usize);
    let glyph_horizontal_bit_selector = (0..8).step_by(step_x as usize);

    for (row, y) in glyph_vertical_index.zip(area.top()..area.bottom()) {
        for (col, x) in glyph_horizontal_bit_selector
            .clone()
            .zip(area.left()..area.right())
        {
            let cell = buf.get_mut(x, y);
            let symbol_character = pixel_size.symbol_for_position(&glyph, row, col);
            cell.set_char(symbol_character);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn build() -> Result<()> {
        let lines = vec![Line::from(vec!["Hello".red(), "World".blue()])];
        let style = Style::new().green();
        let pixel_size = PixelSize::default();
        let alignment = Alignment::Center;
        assert_eq!(
            BigText::builder()
                .lines(lines.clone())
                .style(style)
                .alignment(Alignment::Center)
                .build()?,
            BigText {
                lines,
                style,
                pixel_size,
                alignment,
            }
        );
        Ok(())
    }

    #[test]
    fn render_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            " ████     ██                     ███            ████      ██                    ",
            "██  ██                            ██             ██                             ",
            "███      ███    █████    ███ ██   ██     ████    ██      ███    █████    ████   ",
            " ███      ██    ██  ██  ██  ██    ██    ██  ██   ██       ██    ██  ██  ██  ██  ",
            "   ███    ██    ██  ██  ██  ██    ██    ██████   ██   █   ██    ██  ██  ██████  ",
            "██  ██    ██    ██  ██   █████    ██    ██       ██  ██   ██    ██  ██  ██      ",
            " ████    ████   ██  ██      ██   ████    ████   ███████  ████   ██  ██   ████   ",
            "                        █████                                                   ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██████                                             █               ███",
            "█ ██ █                                            ██                ██",
            "  ██    ██ ███  ██  ██  █████    ████    ████    █████   ████       ██",
            "  ██     ███ ██ ██  ██  ██  ██  ██  ██      ██    ██    ██  ██   █████",
            "  ██     ██  ██ ██  ██  ██  ██  ██       █████    ██    ██████  ██  ██",
            "  ██     ██     ██  ██  ██  ██  ██  ██  ██  ██    ██ █  ██      ██  ██",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 16));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
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
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_widget_style() -> Result<()> {
        let big_text = BigText::builder()
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
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_line_style() -> Result<()> {
        let big_text = BigText::builder()
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
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▄█▀▀█▄    ▀▀                     ▀██            ▀██▀      ▀▀                    ",
            "▀██▄     ▀██    ██▀▀█▄  ▄█▀▀▄█▀   ██    ▄█▀▀█▄   ██      ▀██    ██▀▀█▄  ▄█▀▀█▄  ",
            "▄▄ ▀██    ██    ██  ██  ▀█▄▄██    ██    ██▀▀▀▀   ██  ▄█   ██    ██  ██  ██▀▀▀▀  ",
            " ▀▀▀▀    ▀▀▀▀   ▀▀  ▀▀  ▄▄▄▄█▀   ▀▀▀▀    ▀▀▀▀   ▀▀▀▀▀▀▀  ▀▀▀▀   ▀▀  ▀▀   ▀▀▀▀   ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█▀██▀█                                            ▄█               ▀██",
            "  ██    ▀█▄█▀█▄ ██  ██  ██▀▀█▄  ▄█▀▀█▄   ▀▀▀█▄   ▀██▀▀  ▄█▀▀█▄   ▄▄▄██",
            "  ██     ██  ▀▀ ██  ██  ██  ██  ██  ▄▄  ▄█▀▀██    ██ ▄  ██▀▀▀▀  ██  ██",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██▄ ▄██          ▀██      ▄█      ▀▀    ",
            "███████ ██  ██    ██     ▀██▀▀   ▀██    ",
            "██ ▀ ██ ██  ██    ██      ██ ▄    ██    ",
            "▀▀   ▀▀  ▀▀▀ ▀▀  ▀▀▀▀      ▀▀    ▀▀▀▀   ",
            "▀██▀      ▀▀                            ",
            " ██      ▀██    ██▀▀█▄  ▄█▀▀█▄  ▄█▀▀▀▀  ",
            " ██  ▄█   ██    ██  ██  ██▀▀▀▀   ▀▀▀█▄  ",
            "▀▀▀▀▀▀▀  ▀▀▀▀   ▀▀  ▀▀   ▀▀▀▀   ▀▀▀▀▀   ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_widget_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 48, 4));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▄█▀▀█▄    ▄█             ▀██               ▀██  ",
            "▀██▄     ▀██▀▀  ██  ██    ██    ▄█▀▀█▄   ▄▄▄██  ",
            "▄▄ ▀██    ██ ▄  ▀█▄▄██    ██    ██▀▀▀▀  ██  ██  ",
            " ▀▀▀▀      ▀▀   ▄▄▄▄█▀   ▀▀▀▀    ▀▀▀▀    ▀▀▀ ▀▀ ",
        ]);
        expected.set_style(Rect::new(0, 0, 48, 4), Style::new().bold());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_line_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 12));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▀██▀▀█▄            ▀██                  ",
            " ██▄▄█▀ ▄█▀▀█▄   ▄▄▄██                  ",
            " ██ ▀█▄ ██▀▀▀▀  ██  ██                  ",
            "▀▀▀  ▀▀  ▀▀▀▀    ▀▀▀ ▀▀                 ",
            " ▄█▀▀█▄                                 ",
            "██      ▀█▄█▀█▄ ▄█▀▀█▄  ▄█▀▀█▄  ██▀▀█▄  ",
            "▀█▄ ▀██  ██  ▀▀ ██▀▀▀▀  ██▀▀▀▀  ██  ██  ",
            "  ▀▀▀▀▀ ▀▀▀▀     ▀▀▀▀    ▀▀▀▀   ▀▀  ▀▀  ",
            "▀██▀▀█▄  ▀██                            ",
            " ██▄▄█▀   ██    ██  ██  ▄█▀▀█▄          ",
            " ██  ██   ██    ██  ██  ██▀▀▀▀          ",
            "▀▀▀▀▀▀   ▀▀▀▀    ▀▀▀ ▀▀  ▀▀▀▀           ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 4), Style::new().red());
        expected.set_style(Rect::new(0, 4, 40, 4), Style::new().green());
        expected.set_style(Rect::new(0, 8, 32, 4), Style::new().blue());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▐█▌  █          ▐█      ██   █          ",
            "█ █              █      ▐▌              ",
            "█▌  ▐█  ██▌ ▐█▐▌ █  ▐█▌ ▐▌  ▐█  ██▌ ▐█▌ ",
            "▐█   █  █ █ █ █  █  █ █ ▐▌   █  █ █ █ █ ",
            " ▐█  █  █ █ █ █  █  ███ ▐▌ ▌ █  █ █ ███ ",
            "█ █  █  █ █ ▐██  █  █   ▐▌▐▌ █  █ █ █   ",
            "▐█▌ ▐█▌ █ █   █ ▐█▌ ▐█▌ ███▌▐█▌ █ █ ▐█▌ ",
            "            ██▌                         ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 35, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "███                      ▐       ▐█",
            "▌█▐                      █        █",
            " █  █▐█ █ █ ██▌ ▐█▌ ▐█▌ ▐██ ▐█▌   █",
            " █  ▐█▐▌█ █ █ █ █ █   █  █  █ █ ▐██",
            " █  ▐▌▐▌█ █ █ █ █   ▐██  █  ███ █ █",
            " █  ▐▌  █ █ █ █ █ █ █ █  █▐ █   █ █",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 16));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█ ▐▌    ▐█   ▐   █  ",
            "█▌█▌     █   █      ",
            "███▌█ █  █  ▐██ ▐█  ",
            "███▌█ █  █   █   █  ",
            "█▐▐▌█ █  █   █   █  ",
            "█ ▐▌█ █  █   █▐  █  ",
            "█ ▐▌▐█▐▌▐█▌  ▐▌ ▐█▌ ",
            "                    ",
            "██   █              ",
            "▐▌                  ",
            "▐▌  ▐█  ██▌ ▐█▌ ▐██ ",
            "▐▌   █  █ █ █ █ █   ",
            "▐▌ ▌ █  █ █ ███ ▐█▌ ",
            "▐▌▐▌ █  █ █ █     █ ",
            "███▌▐█▌ █ █ ▐█▌ ██▌ ",
            "                    ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_widget_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 24, 8));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▐█▌  ▐      ▐█       ▐█ ",
            "█ █  █       █        █ ",
            "█▌  ▐██ █ █  █  ▐█▌   █ ",
            "▐█   █  █ █  █  █ █ ▐██ ",
            " ▐█  █  █ █  █  ███ █ █ ",
            "█ █  █▐ ▐██  █  █   █ █ ",
            "▐█▌  ▐▌   █ ▐█▌ ▐█▌ ▐█▐▌",
            "        ██▌             ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 8), Style::new().bold());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_line_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 24));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "███      ▐█         ",
            "▐▌▐▌      █         ",
            "▐▌▐▌▐█▌   █         ",
            "▐██ █ █ ▐██         ",
            "▐▌█ ███ █ █         ",
            "▐▌▐▌█   █ █         ",
            "█▌▐▌▐█▌ ▐█▐▌        ",
            "                    ",
            " ██                 ",
            "▐▌▐▌                ",
            "█   █▐█ ▐█▌ ▐█▌ ██▌ ",
            "█   ▐█▐▌█ █ █ █ █ █ ",
            "█ █▌▐▌▐▌███ ███ █ █ ",
            "▐▌▐▌▐▌  █   █   █ █ ",
            " ██▌██  ▐█▌ ▐█▌ █ █ ",
            "                    ",
            "███ ▐█              ",
            "▐▌▐▌ █              ",
            "▐▌▐▌ █  █ █ ▐█▌     ",
            "▐██  █  █ █ █ █     ",
            "▐▌▐▌ █  █ █ ███     ",
            "▐▌▐▌ █  █ █ █       ",
            "███ ▐█▌ ▐█▐▌▐█▌     ",
            "                    ",
        ]);
        expected.set_style(Rect::new(0, 0, 12, 8), Style::new().red());
        expected.set_style(Rect::new(0, 8, 20, 8), Style::new().green());
        expected.set_style(Rect::new(0, 16, 16, 8), Style::new().blue());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_quadrant_size_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▟▀▙  ▀          ▝█      ▜▛   ▀          ",
            "▜▙  ▝█  █▀▙ ▟▀▟▘ █  ▟▀▙ ▐▌  ▝█  █▀▙ ▟▀▙ ",
            "▄▝█  █  █ █ ▜▄█  █  █▀▀ ▐▌▗▌ █  █ █ █▀▀ ",
            "▝▀▘ ▝▀▘ ▀ ▀ ▄▄▛ ▝▀▘ ▝▀▘ ▀▀▀▘▝▀▘ ▀ ▀ ▝▀▘ ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_quadrant_size_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 35, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▛█▜                      ▟       ▝█",
            " █  ▜▟▜▖█ █ █▀▙ ▟▀▙ ▝▀▙ ▝█▀ ▟▀▙ ▗▄█",
            " █  ▐▌▝▘█ █ █ █ █ ▄ ▟▀█  █▗ █▀▀ █ █",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_quadrant_size_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█▖▟▌    ▝█   ▟   ▀  ",
            "███▌█ █  █  ▝█▀ ▝█  ",
            "█▝▐▌█ █  █   █▗  █  ",
            "▀ ▝▘▝▀▝▘▝▀▘  ▝▘ ▝▀▘ ",
            "▜▛   ▀              ",
            "▐▌  ▝█  █▀▙ ▟▀▙ ▟▀▀ ",
            "▐▌▗▌ █  █ █ █▀▀ ▝▀▙ ",
            "▀▀▀▘▝▀▘ ▀ ▀ ▝▀▘ ▀▀▘ ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_quadrant_size_widget_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 24, 4));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▟▀▙  ▟      ▝█       ▝█ ",
            "▜▙  ▝█▀ █ █  █  ▟▀▙ ▗▄█ ",
            "▄▝█  █▗ ▜▄█  █  █▀▀ █ █ ",
            "▝▀▘  ▝▘ ▄▄▛ ▝▀▘ ▝▀▘ ▝▀▝▘",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 4), Style::new().bold());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_quadrant_size_line_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 12));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▜▛▜▖     ▝█         ",
            "▐▙▟▘▟▀▙ ▗▄█         ",
            "▐▌▜▖█▀▀ █ █         ",
            "▀▘▝▘▝▀▘ ▝▀▝▘        ",
            "▗▛▜▖                ",
            "█   ▜▟▜▖▟▀▙ ▟▀▙ █▀▙ ",
            "▜▖▜▌▐▌▝▘█▀▀ █▀▀ █ █ ",
            " ▀▀▘▀▀  ▝▀▘ ▝▀▘ ▀ ▀ ",
            "▜▛▜▖▝█              ",
            "▐▙▟▘ █  █ █ ▟▀▙     ",
            "▐▌▐▌ █  █ █ █▀▀     ",
            "▀▀▀ ▝▀▘ ▝▀▝▘▝▀▘     ",
        ]);
        expected.set_style(Rect::new(0, 0, 12, 4), Style::new().red());
        expected.set_style(Rect::new(0, 4, 20, 4), Style::new().green());
        expected.set_style(Rect::new(0, 8, 16, 4), Style::new().blue());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_third_height_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "🬹█🬰🬂🬎🬋   🬭🬰🬰    🬭🬭🬭🬭🬭    🬭🬭🬭 🬭🬭  🬂██     🬭🬭🬭🬭   🬂██🬂     🬭🬰🬰    🬭🬭🬭🬭🬭    🬭🬭🬭🬭   ",
            "🬭🬰🬂🬎🬹🬹    ██    ██  ██  🬎█🬭🬭██    ██    ██🬋🬋🬎🬎   ██  🬭🬹   ██    ██  ██  ██🬋🬋🬎🬎  ",
            " 🬂🬂🬂🬂    🬂🬂🬂🬂   🬂🬂  🬂🬂  🬋🬋🬋🬋🬎🬂   🬂🬂🬂🬂    🬂🬂🬂🬂   🬂🬂🬂🬂🬂🬂🬂  🬂🬂🬂🬂   🬂🬂  🬂🬂   🬂🬂🬂🬂   ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_third_height_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 2));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "🬎🬂██🬂🬎  🬭🬭 🬭🬭🬭  🬭🬭  🬭🬭  🬭🬭🬭🬭🬭    🬭🬭🬭🬭    🬭🬭🬭🬭    🬭🬹█🬭🬭   🬭🬭🬭🬭      🬂██",
            "  ██     ██🬂 🬎🬎 ██  ██  ██  ██  ██  🬰🬰  🬭🬹🬋🬋██    ██ 🬭  ██🬋🬋🬎🬎  🬹█🬂🬂██",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_third_height_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██🬹🬭🬹██ 🬭🬭  🬭🬭   🬂██     🬭🬹█🬭🬭   🬭🬰🬰    ",
            "██🬂🬎🬂██ ██  ██    ██      ██ 🬭    ██    ",
            "🬂🬂   🬂🬂  🬂🬂🬂 🬂🬂  🬂🬂🬂🬂      🬂🬂    🬂🬂🬂🬂   ",
            "🬂██🬂     🬭🬰🬰    🬭🬭🬭🬭🬭    🬭🬭🬭🬭    🬭🬭🬭🬭🬭  ",
            " ██  🬭🬹   ██    ██  ██  ██🬋🬋🬎🬎  🬂🬎🬋🬋🬹🬭  ",
            "🬂🬂🬂🬂🬂🬂🬂  🬂🬂🬂🬂   🬂🬂  🬂🬂   🬂🬂🬂🬂   🬂🬂🬂🬂🬂   ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_third_height_widget_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 48, 3));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "🬹█🬰🬂🬎🬋   🬭🬹█🬭🬭  🬭🬭  🬭🬭   🬂██     🬭🬭🬭🬭      🬂██  ",
            "🬭🬰🬂🬎🬹🬹    ██ 🬭  🬎█🬭🬭██    ██    ██🬋🬋🬎🬎  🬹█🬂🬂██  ",
            " 🬂🬂🬂🬂      🬂🬂   🬋🬋🬋🬋🬎🬂   🬂🬂🬂🬂    🬂🬂🬂🬂    🬂🬂🬂 🬂🬂 ",
        ]);
        expected.set_style(Rect::new(0, 0, 48, 3), Style::new().bold());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_third_height_line_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 9));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "🬂██🬂🬂█🬹  🬭🬭🬭🬭      🬂██                  ",
            " ██🬂🬎█🬭 ██🬋🬋🬎🬎  🬹█🬂🬂██                  ",
            "🬂🬂🬂  🬂🬂  🬂🬂🬂🬂    🬂🬂🬂 🬂🬂                 ",
            "🬭🬹🬎🬂🬂🬎🬋 🬭🬭 🬭🬭🬭   🬭🬭🬭🬭    🬭🬭🬭🬭   🬭🬭🬭🬭🬭   ",
            "🬎█🬭 🬋🬹🬹  ██🬂 🬎🬎 ██🬋🬋🬎🬎  ██🬋🬋🬎🬎  ██  ██  ",
            "  🬂🬂🬂🬂🬂 🬂🬂🬂🬂     🬂🬂🬂🬂    🬂🬂🬂🬂   🬂🬂  🬂🬂  ",
            "🬂██🬂🬂█🬹  🬂██    🬭🬭  🬭🬭   🬭🬭🬭🬭           ",
            " ██🬂🬂█🬹   ██    ██  ██  ██🬋🬋🬎🬎          ",
            "🬂🬂🬂🬂🬂🬂   🬂🬂🬂🬂    🬂🬂🬂 🬂🬂  🬂🬂🬂🬂           ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 3), Style::new().red());
        expected.set_style(Rect::new(0, 3, 40, 3), Style::new().green());
        expected.set_style(Rect::new(0, 6, 32, 3), Style::new().blue());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_sextant_size_single_line() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "🬻🬒🬌 🬞🬰  🬭🬭🬏 🬞🬭🬞🬏🬁█  🬞🬭🬏 🬨🬕  🬞🬰  🬭🬭🬏 🬞🬭🬏 ",
            "🬯🬊🬹  █  █ █ 🬬🬭█  █  █🬋🬎 ▐▌🬞🬓 █  █ █ █🬋🬎 ",
            "🬁🬂🬀 🬁🬂🬀 🬂 🬂 🬋🬋🬆 🬁🬂🬀 🬁🬂🬀 🬂🬂🬂🬀🬁🬂🬀 🬂 🬂 🬁🬂🬀 ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_sextant_size_truncated() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 35, 2));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "🬆█🬊 🬭🬞🬭 🬭 🬭 🬭🬭🬏 🬞🬭🬏 🬞🬭🬏 🬞🬻🬭 🬞🬭🬏  🬁█",
            " █  ▐🬕🬉🬄█ █ █ █ █ 🬰 🬵🬋█  █🬞 █🬋🬎 🬻🬂█",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_sextant_size_multiple_lines() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█🬱🬻▌🬭 🬭 🬁█  🬞🬻🬭 🬞🬰  ",
            "█🬊🬨▌█ █  █   █🬞  █  ",
            "🬂 🬁🬀🬁🬂🬁🬀🬁🬂🬀  🬁🬀 🬁🬂🬀 ",
            "🬨🬕  🬞🬰  🬭🬭🬏 🬞🬭🬏 🬞🬭🬭 ",
            "▐▌🬞🬓 █  █ █ █🬋🬎 🬊🬋🬱 ",
            "🬂🬂🬂🬀🬁🬂🬀 🬂 🬂 🬁🬂🬀 🬂🬂🬀 ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_sextant_size_widget_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 24, 3));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "🬻🬒🬌 🬞🬻🬭 🬭 🬭 🬁█  🬞🬭🬏  🬁█ ",
            "🬯🬊🬹  █🬞 🬬🬭█  █  █🬋🬎 🬻🬂█ ",
            "🬁🬂🬀  🬁🬀 🬋🬋🬆 🬁🬂🬀 🬁🬂🬀 🬁🬂🬁🬀",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 3), Style::new().bold());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_sextant_size_line_style() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 9));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "🬨🬕🬨🬓🬞🬭🬏  🬁█         ",
            "▐🬕🬬🬏█🬋🬎 🬻🬂█         ",
            "🬂🬀🬁🬀🬁🬂🬀 🬁🬂🬁🬀        ",
            "🬵🬆🬊🬃🬭🬞🬭 🬞🬭🬏 🬞🬭🬏 🬭🬭🬏 ",
            "🬬🬏🬩🬓▐🬕🬉🬄█🬋🬎 █🬋🬎 █ █ ",
            " 🬂🬂🬀🬂🬂  🬁🬂🬀 🬁🬂🬀 🬂 🬂 ",
            "🬨🬕🬨🬓🬁█  🬭 🬭 🬞🬭🬏     ",
            "▐🬕🬨🬓 █  █ █ █🬋🬎     ",
            "🬂🬂🬂 🬁🬂🬀 🬁🬂🬁🬀🬁🬂🬀     ",
        ]);
        expected.set_style(Rect::new(0, 0, 12, 3), Style::new().red());
        expected.set_style(Rect::new(0, 3, 20, 3), Style::new().green());
        expected.set_style(Rect::new(0, 6, 16, 3), Style::new().blue());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_alignment_left() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Left")])
            .alignment(Alignment::Left)
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▜▛      ▗▛▙  ▟                          ",
            "▐▌  ▟▀▙ ▟▙  ▝█▀                         ",
            "▐▌▗▌█▀▀ ▐▌   █▗                         ",
            "▀▀▀▘▝▀▘ ▀▀   ▝▘                         ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_alignment_right() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Right")])
            .alignment(Alignment::Right)
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "                    ▜▛▜▖ ▀      ▜▌   ▟  ",
            "                    ▐▙▟▘▝█  ▟▀▟▘▐▙▜▖▝█▀ ",
            "                    ▐▌▜▖ █  ▜▄█ ▐▌▐▌ █▗ ",
            "                    ▀▘▝▘▝▀▘ ▄▄▛ ▀▘▝▘ ▝▘ ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_alignment_center() -> Result<()> {
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Centered"), Line::from("Lines")])
            .alignment(Alignment::Center)
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "    ▗▛▜▖         ▟               ▝█     ",
            "    █   ▟▀▙ █▀▙ ▝█▀ ▟▀▙ ▜▟▜▖▟▀▙ ▗▄█     ",
            "    ▜▖▗▖█▀▀ █ █  █▗ █▀▀ ▐▌▝▘█▀▀ █ █     ",
            "     ▀▀ ▝▀▘ ▀ ▀  ▝▘ ▝▀▘ ▀▀  ▝▀▘ ▝▀▝▘    ",
            "          ▜▛   ▀                        ",
            "          ▐▌  ▝█  █▀▙ ▟▀▙ ▟▀▀           ",
            "          ▐▌▗▌ █  █ █ █▀▀ ▝▀▙           ",
            "          ▀▀▀▘▝▀▘ ▀ ▀ ▝▀▘ ▀▀▘           ",
        ]);
        assert_eq!(buf, expected);
        Ok(())
    }
}
