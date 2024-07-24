use ratatui::{
    layout::{Position, Size},
    prelude::*,
    widgets::*,
};

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
        let visible_area = self
            .render_scrollbars(area, buf, state)
            .intersection(self.buf.area);
        self.render_visible_area(area, buf, visible_area);
    }
}

impl ScrollView {
    /// Render the horizontal and vertical scrollbars if exists, and return the size taken by the
    /// scrollbars
    fn render_scrollbars(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) -> Rect {
        let size = self.size;
        let width = size.width.saturating_sub(area.width);
        let height = size.height.saturating_sub(area.height);
        match (width, height) {
            (0, 0) => {
                // area is taller and wider than the scroll_view
                state.offset = Position::default();
                Rect::new(state.offset.x, state.offset.y, area.width, area.height)
            }
            (_, 0) if area.height > size.height => {
                // area is taller and narrower than the scroll_view
                state.offset.y = 0;
                self.render_horizontal_scrollbar(area, buf, state);
                Rect::new(state.offset.x, 0, area.width, area.height - 1)
            }
            (0, _) if area.width > size.width => {
                // area is wider and shorter than the scroll_view
                state.offset.x = 0;
                self.render_vertical_scrollbar(area, buf, state);
                Rect::new(0, state.offset.y, area.width - 1, area.height)
            }
            (_, _) => {
                // scroll_view is both wider and taller than the area
                let vertical_area = Rect {
                    height: area.height - 1,
                    ..area
                };
                let horizontal_area = Rect {
                    width: area.width - 1,
                    ..area
                };
                self.render_vertical_scrollbar(vertical_area, buf, state);
                self.render_horizontal_scrollbar(horizontal_area, buf, state);
                Rect::new(
                    state.offset.x,
                    state.offset.y,
                    area.width - 1,
                    area.height - 1,
                )
            }
        }
    }

    fn render_vertical_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let mut scrollbar_state =
            ScrollbarState::new(self.size.height as usize).position(state.offset.y as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }

    fn render_horizontal_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let mut scrollbar_state =
            ScrollbarState::new(self.size.width as usize).position(state.offset.x as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }

    fn render_visible_area(&self, area: Rect, buf: &mut Buffer, visible_area: Rect) {
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
    use rstest::{fixture, rstest};

    /// Initialize a buffer and a scroll view with a buffer size of 10x10
    ///
    /// The buffer will be filled with characters from A to Z in a 10x10 grid
    ///
    /// ```plain
    /// ABCDEFGHIJ
    /// KLMNOPQRST
    /// UVWXYZABCD
    /// EFGHIJKLMN
    /// OPQRSTUVWX
    /// YZABCDEFGH
    /// IJKLMNOPQR
    /// STUVWXYZAB
    /// CDEFGHIJKL
    /// MNOPQRSTUV
    /// ```
    #[fixture]
    fn scroll_view() -> ScrollView {
        let mut scroll_view = ScrollView::new(Size::new(10, 10));
        for y in 0..10 {
            for x in 0..10 {
                let c = char::from_u32((x + y * 10) % 26 + 65).unwrap();
                let widget = Span::raw(format!("{c}"));
                let area = Rect::new(x as u16, y as u16, 1, 1);
                scroll_view.render_widget(widget, area);
            }
        }
        scroll_view
    }

    #[rstest]
    fn zero_offset(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::default();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDE▲",
                "KLMNO█",
                "UVWXY║",
                "EFGHI║",
                "OPQRS▼",
                "◄█══► ",
            ])
        )
    }

    #[rstest]
    fn move_right(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((3, 0).into());
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "DEFGH▲",
                "NOPQR█",
                "XYZAB║",
                "HIJKL║",
                "RSTUV▼",
                "◄═█═► ",
            ])
        )
    }

    #[rstest]
    fn move_down(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((0, 3).into());
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "EFGHI▲",
                "OPQRS║",
                "YZABC█",
                "IJKLM║",
                "STUVW▼",
                "◄█══► ",
            ])
        )
    }

    #[rstest]
    fn hides_both_scrollbars(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ",
                "KLMNOPQRST",
                "UVWXYZABCD",
                "EFGHIJKLMN",
                "OPQRSTUVWX",
                "YZABCDEFGH",
                "IJKLMNOPQR",
                "STUVWXYZAB",
                "CDEFGHIJKL",
                "MNOPQRSTUV",
            ])
        )
    }

    #[rstest]
    fn hides_horizontal_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ▲",
                "KLMNOPQRST█",
                "UVWXYZABCD█",
                "EFGHIJKLMN█",
                "OPQRSTUVWX█",
                "YZABCDEFGH║",
                "IJKLMNOPQR║",
                "STUVWXYZAB║",
                "CDEFGHIJKL▼",
            ])
        )
    }

    #[rstest]
    fn hides_vertical_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 11));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
                "◄████═══►",
            ])
        )
    }

    /// Tests the scenario where the vertical scollbar steals a column from the right side of the
    /// buffer which causes the horizontal scrollbar to be shown.
    #[rstest]
    fn does_not_hide_horizontal_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI▲",
                "KLMNOPQRS█",
                "UVWXYZABC█",
                "EFGHIJKLM█",
                "OPQRSTUVW║",
                "YZABCDEFG║",
                "IJKLMNOPQ║",
                "STUVWXYZA▼",
                "◄████═══► ",
            ])
        )
    }

    /// Tests the scenario where the horizontal scollbar steals a row from the bottom side of the
    /// buffer which causes the vertical scrollbar to be shown.
    #[rstest]
    fn does_not_hide_vertical_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGH▲",
                "KLMNOPQR█",
                "UVWXYZAB█",
                "EFGHIJKL█",
                "OPQRSTUV█",
                "YZABCDEF║",
                "IJKLMNOP║",
                "STUVWXYZ║",
                "CDEFGHIJ▼",
                "◄███═══► ",
            ])
        )
    }

    /// The purpose of this test is to ensure that the buffer offset is correctly calculated when
    /// rendering a scroll view into a buffer (i.e. the buffer offset is not always (0, 0))
    #[rstest]
    fn ensure_buffer_offset_is_correct(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 20));
        let mut state = ScrollViewState::with_offset((3, 4).into());
        scroll_view.render(Rect::new(5, 6, 7, 8), &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "     RSTUVW▲        ",
                "     BCDEFG║        ",
                "     LMNOPQ█        ",
                "     VWXYZA█        ",
                "     FGHIJK║        ",
                "     PQRSTU║        ",
                "           ▼        ",
                "     ◄═█══►         ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
            ])
        )
    }
}
