use ratatui::{layout::Size, prelude::*, widgets::*};

use crate::ScrollViewState;

/// A widget that can scroll its contents
///
/// Allows you to render a widget into a buffer larger than the area it is rendered into, and then
/// scroll the contents of that buffer around.
///
/// Note that the origin of the buffer is always at (0, 0), and the buffer is always the size of the
/// size passed to `new`. The `ScrollView` widget itself is responsible for rendering the visible
/// area of the buffer into the main buffer.
///
/// # Examples
///
/// ```rust
/// use ratatui::{prelude::*, layout::Size, widgets::*};
/// use tui_scrollview::{ScrollView, ScrollViewState};
///
/// # fn render(buf: &mut Buffer) {
/// let mut scroll_view = ScrollView::new(Size::new(20, 20));
///
/// // render a few widgets into the buffer at various positions
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(0, 0, 20, 1));
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(10, 10, 20, 1));
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(15, 15, 20, 1));
///
/// // You can also render widgets into the buffer programmatically
/// Line::raw("Hello, world!").render(Rect::new(0, 0, 20, 1), scroll_view.buf_mut());
///
/// // usually you would store the state of the scroll view in a struct that implements
/// // StatefulWidget (or in your app state if you're using an `App` struct)
/// let mut state = ScrollViewState::default();
///
/// // you can also scroll the view programmatically
/// state.scroll_down();
///
/// // render the scroll view into the main buffer at the given position within a widget
/// let scroll_view_area = Rect::new(0, 0, 10, 10);
/// scroll_view.render(scroll_view_area, buf, &mut state);
/// # }
/// // or if you're rendering in a terminal draw closure instead of from within another widget:
/// # fn terminal_draw(frame: &mut Frame, scroll_view: ScrollView, state: &mut ScrollViewState) {
/// frame.render_stateful_widget(scroll_view, frame.size(), state);
/// # }
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ScrollView {
    buf: Buffer,
    size: Size,
}

impl ScrollView {
    /// Create a new scroll view with a buffer of the given size
    ///
    /// The buffer will be empty, with coordinates ranging from (0, 0) to (size.width, size.height).
    pub fn new(size: Size) -> Self {
        // TODO: this is replaced with Rect::from(size) in the next version of ratatui
        let area = Rect::new(0, 0, size.width, size.height);
        Self {
            buf: Buffer::empty(area),
            size,
        }
    }

    /// The content size of the scroll view
    pub fn size(&self) -> Size {
        self.size
    }

    /// The area of the buffer that is available to be scrolled
    pub fn area(&self) -> Rect {
        self.buf.area
    }

    /// The buffer containing the contents of the scroll view
    pub fn buf(&self) -> &Buffer {
        &self.buf
    }

    /// The mutable buffer containing the contents of the scroll view
    ///
    /// This can be used to render widgets into the buffer programmatically
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20));
    /// Line::raw("Hello, world!").render(Rect::new(0, 0, 20, 1), scroll_view.buf_mut());
    /// ```
    pub fn buf_mut(&mut self) -> &mut Buffer {
        &mut self.buf
    }

    /// Render a widget into the scroll buffer
    ///
    /// This is the equivalent of `Frame::render_widget`, but renders the widget into the scroll
    /// buffer rather than the main buffer. The widget will be rendered into the area of the buffer
    /// specified by the `area` parameter.
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
        state.size = Some(self.size);
        state.page_size = Some(area.into());
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

        let mut scroll_view_state = ScrollViewState::with_offset((0, 1).into());
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
