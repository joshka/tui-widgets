use ratatui::{layout::Size, prelude::*, widgets::*};

use crate::ScrollViewState;

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

impl StatefulWidget for ScrollView {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (mut x, mut y) = state.offset.into();
        // ensure that we don't scroll past the end of the buffer in either direction
        x = x.min(self.buf.area.width.saturating_sub(1));
        y = y.min(self.buf.area.height.saturating_sub(1));
        state.offset = (x, y).into();
        let visible_area = Rect::new(x, y, area.width, area.height).intersection(self.buf.area);
        self.render_visible_area(area, buf, visible_area);
        // TODO work out whether to render the scrollbars or not
        self.render_vertical_scrollbar(area, buf, state);
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
            ScrollbarState::new(self.size.height as usize).position(state.offset.y as usize);
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

        let mut scroll_view_state = ScrollViewState::default();
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

        let mut scroll_view_state = ScrollViewState::default();
        scroll_view_state.scroll_right();
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

        let mut scroll_view_state = ScrollViewState {
            offset: (0, 1).into(),
        };
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
