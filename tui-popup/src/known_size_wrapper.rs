use std::fmt::Debug;

use derive_setters::Setters;
use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::KnownSize;

/// The `KnownSizeWrapper` struct wraps a widget and provides a fixed size for it.
///
/// This struct is used to wrap a widget and provide a fixed size for it. This is useful when you
/// want to use a widget that does not implement `SizedWidgetRef` as the body of a popup.
#[derive(Debug, Setters, Clone)]
pub struct KnownSizeWrapper<W> {
    #[setters(skip)]
    pub inner: W,
    pub width: usize,
    pub height: usize,
}

impl<W: Widget> Widget for KnownSizeWrapper<W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
    }
}

impl<W> KnownSize for KnownSizeWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W> KnownSize for &KnownSizeWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W> KnownSizeWrapper<W> {
    /// Create a new `KnownSizeWrapper` with the given widget and size.
    pub const fn new(inner: W, width: usize, height: usize) -> Self {
        Self {
            inner,
            width,
            height,
        }
    }
}
#[cfg(test)]
mod tests {
    use ratatui_core::{buffer::Buffer, layout::Rect};

    use super::*;

    struct TestWidget;

    impl Widget for TestWidget {
        fn render(self, area: Rect, buf: &mut Buffer) {
            "Hello".render(area, buf);
        }
    }

    #[test]
    fn test_sized_wrapper_new() {
        let widget = TestWidget;
        let wrapper = KnownSizeWrapper::new(widget, 10, 20);
        assert_eq!(wrapper.width, 10);
        assert_eq!(wrapper.height, 20);
    }

    #[test]
    fn test_sized_wrapper_render() {
        let widget = TestWidget;
        let wrapper = KnownSizeWrapper::new(widget, 20, 5);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
        wrapper.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines([
            "Hello               ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }
}
