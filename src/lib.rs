//! # Tui-scrollview
//!
//! [![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
//! [![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
//! Discord]
//!
//! `tui-scrollview` is a library for creating scrollable views in [Ratatui].
//!
//! ## Installation
//!
//! ```shell
//! cargo add tui-scrollview
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use tui_scrollview::ScrollView;
//! use ratatui::{prelude::*, layout::Size};
//!
//! fn render(Frame: &mut Frame) {
//!     let size = Size::new(10, 100);
//!     let mut scroll_view = ScrollView::new(size);
//!     let some_long_string =
//!         iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
//!            .take(100)
//!            .collect::<String>();
//!     let area = size.into();
//!     scroll_view.render_widget(Text::from(some_long_string), area);
//!     let state = ScrollViewState::default();
//!     frame.render_stateful_widget(scroll_view, area);
//! }
//! ```
//!
//! [Crates.io Badge]: https://img.shields.io/crates/v/tui-scrollview?logo=rust&style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/tui-scrollview?style=for-the-badge
//! [Docs.rs Badge]: https://img.shields.io/docsrs/tui-scrollview?logo=rust&style=for-the-badge
//! [Deps.rs Badge]:
//!     https://deps.rs/repo/github/joshka/tui-scrollview/status.svg?style=for-the-badge
//! [Codecov.io Badge]:
//!     https://img.shields.io/codecov/c/github/joshka/tui-scrollview?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//!
//! [Crate]: https://crates.io/crates/tui-scrollview
//! [API Docs]: https://docs.rs/crate/tui-scrollview/
//! [Dependencies]: https://deps.rs/repo/github/joshka/tui-scrollview
//! [Coverage]: https://app.codecov.io/gh/joshka/tui-scrollview
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//!
//! [Ratatui]: https://crates.io/crates/ratatui

use ratatui::{buffer::Buffer, layout::Size, prelude::*, widgets::*};

/// A widget that can scroll its contents
///
/// Allows you to render a widget into a buffer larger than the area it is rendered into, and then
/// scroll the contents of that buffer around.
///
/// # Examples
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ScrollView {
    buf: Buffer,
}

impl ScrollView {
    /// Create a new scroll view with a buffer of the given size
    pub fn new(size: Size) -> Self {
        // TODO: this is replaced with Rect::from(size) in the next version of ratatui
        let area = Rect::new(0, 0, size.width, size.height);
        Self {
            buf: Buffer::empty(area),
        }
    }

    /// Render a widget into the scroll buffer
    ///
    /// This should not be confused with the `render` method, which renders the visible area of the
    /// ScrollView into the main buffer.
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, &mut self.buf);
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ScrollViewState {
    /// The offset is the number of rows and columns to shift the scroll view by.
    offset: (u16, u16),
}

impl ScrollViewState {
    /// Create a new scroll view state with an offset of (0, 0)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new scroll view state with the given offset
    pub fn with_offset(offset: (u16, u16)) -> Self {
        Self { offset }
    }
}

impl StatefulWidget for ScrollView {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_visible_area(state, area, buf);
    }
}

impl ScrollView {
    fn render_visible_area(self, state: &mut ScrollViewState, area: Rect, buf: &mut Buffer) {
        let (x, y) = state.offset;
        let visible_area = Rect::new(x, y, area.width, area.height);
        // TODO: there's probably a more efficient way to do this
        for (src_row, dst_row) in visible_area.rows().zip(area.rows()) {
            for (src_col, dst_col) in src_row.columns().zip(dst_row.columns()) {
                *buf.get_mut(dst_col.x, dst_col.y) = self.buf.get(src_col.x, src_col.y).clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::assert_buffer_eq;

    fn init() -> (Buffer, ScrollView) {
        let buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let mut scroll_buffer = ScrollView::new(Size::new(20, 20));
        for y in 0..20 {
            for x in 0..20 {
                let c = char::from_u32((x + y * 20) % 26 + 65).unwrap();
                let widget = Span::raw(format!("{c}"));
                let area = Rect::new(x as u16, y as u16, 1, 1);
                scroll_buffer.render_widget(widget, area);
            }
        }
        (buf, scroll_buffer)
    }

    #[test]
    fn zero_offset() {
        let (mut buf, scroll_buffer) = init();

        let mut scroll_view_state = ScrollViewState { offset: (0, 0) };
        scroll_buffer.render(Rect::new(2, 2, 5, 5), &mut buf, &mut scroll_view_state);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "          ",
                "          ",
                "  ABCDE   ",
                "  UVWXY   ",
                "  OPQRS   ",
                "  IJKLM   ",
                "  CDEFG   ",
                "          ",
                "          ",
                "          ",
            ])
        )
    }

    #[test]
    fn move_right() {
        let (mut buf, scroll_buffer) = init();

        let mut scroll_view_state = ScrollViewState { offset: (1, 0) };
        scroll_buffer.render(Rect::new(2, 2, 5, 5), &mut buf, &mut scroll_view_state);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "          ",
                "          ",
                "  BCDEF   ",
                "  VWXYZ   ",
                "  PQRST   ",
                "  JKLMN   ",
                "  DEFGH   ",
                "          ",
                "          ",
                "          ",
            ])
        )
    }

    #[test]
    fn move_down() {
        let (mut buf, scroll_buffer) = init();

        let mut scroll_view_state = ScrollViewState { offset: (0, 1) };
        scroll_buffer.render(Rect::new(2, 2, 5, 5), &mut buf, &mut scroll_view_state);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "          ",
                "          ",
                "  UVWXY   ",
                "  OPQRS   ",
                "  IJKLM   ",
                "  CDEFG   ",
                "  WXYZA   ",
                "          ",
                "          ",
                "          ",
            ])
        )
    }
}
