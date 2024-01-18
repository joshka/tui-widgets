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
//! use std::iter;
//! use tui_scrollview::{ScrollView, ScrollViewState};
//! use ratatui::{layout::Size, prelude::*, widgets::*};
//!
//! fn render(frame: &mut Frame) {
//!     let size = Size::new(10, 100);
//!     let mut scroll_view = ScrollView::new(size);
//!     let some_long_string =
//!         iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
//!            .take(100)
//!            .collect::<String>();
//!     let area = Rect::new(0, 0, 10, 100);
//!     scroll_view.render_widget(Paragraph::new(some_long_string), area);
//!     let mut state = ScrollViewState::default();
//!     frame.render_stateful_widget(scroll_view, area, &mut state);
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
    size: Size,
}

impl ScrollView {
    /// Create a new scroll view with a buffer of the given size
    pub fn new(size: Size) -> Self {
        // TODO: this is replaced with Rect::from(size) in the next version of ratatui
        let area = Rect::new(0, 0, size.width, size.height);
        Self {
            buf: Buffer::empty(area),
            size,
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

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
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

    /// Set the offset of the scroll view state
    pub fn set_offset(&mut self, offset: (u16, u16)) {
        self.offset = offset;
    }

    /// Get the offset of the scroll view state
    pub fn offset(&self) -> (u16, u16) {
        self.offset
    }

    /// Move the scroll view state up by one row
    pub fn up(&mut self) {
        self.offset.1 = self.offset.1.saturating_sub(1);
    }

    /// Move the scroll view state down by one row
    pub fn down(&mut self) {
        self.offset.1 = self.offset.1.saturating_add(1);
    }

    /// Move the scroll view state left by one column
    pub fn left(&mut self) {
        self.offset.0 = self.offset.0.saturating_sub(1);
    }

    /// Move the scroll view state right by one column
    pub fn right(&mut self) {
        self.offset.0 = self.offset.0.saturating_add(1);
    }

    /// Move the scroll view state to the top of the buffer
    pub fn top(&mut self) {
        self.offset = (0, 0);
    }

    /// Move the scroll view state to the bottom of the buffer
    pub fn bottom(&mut self) {
        // the render call will adjust the offset to ensure that we don't scroll past the end of
        // the buffer, so we can just set the offset to the maximum value here
        self.offset = (0, u16::MAX);
    }
}

impl StatefulWidget for ScrollView {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (mut x, mut y) = state.offset;
        // ensure that we don't scroll past the end of the buffer in either direction
        x = x.min(self.buf.area.width.saturating_sub(1));
        y = y.min(self.buf.area.height.saturating_sub(1));
        state.offset = (x, y);
        let visible_area = Rect::new(x, y, area.width, area.height).intersection(self.buf.area);
        self.render_visible_area(area, buf, visible_area);
        // TODO work out whether to render the scrollbars or not
        self.render_vertical_scrollbar(area, buf, &state);
    }
}

impl ScrollView {
    fn render_visible_area(&self, area: Rect, buf: &mut Buffer, visible_area: Rect) {
        // TODO: there's probably a more efficient way to do this
        for (src_row, dst_row) in visible_area.rows().zip(area.rows()) {
            for (src_col, dst_col) in src_row.columns().zip(dst_row.columns()) {
                *buf.get_mut(dst_col.x, dst_col.y) = self.buf.get(src_col.x, src_col.y).clone();
            }
        }
    }

    fn render_vertical_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let mut scrollbar_state =
            ScrollbarState::new(self.size.height as usize).position(state.offset.1 as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scrollbar.render(area, buf, &mut scrollbar_state);
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
                "  ABCD▲   ",
                "  UVWX█   ",
                "  OPQR║   ",
                "  IJKL║   ",
                "  CDEF▼   ",
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
                "  BCDE▲   ",
                "  VWXY█   ",
                "  PQRS║   ",
                "  JKLM║   ",
                "  DEFG▼   ",
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
                "  UVWX▲   ",
                "  OPQR█   ",
                "  IJKL║   ",
                "  CDEF║   ",
                "  WXYZ▼   ",
                "          ",
                "          ",
                "          ",
            ])
        )
    }
}
