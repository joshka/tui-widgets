use std::fmt::Debug;

use derive_setters::Setters;
use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use crate::KnownSize;

/// The `KnownSizeWrapper` struct wraps a widget and provides a fixed size for it.
///
/// This struct is used to wrap a widget and provide a fixed size for it. This is useful when you
/// want to use a widget that does not implement `SizedWidgetRef` as the body of a popup.
#[derive(Debug, Setters)]
pub struct KnownSizeWrapper<W> {
    #[setters(skip)]
    pub inner: W,
    pub width: usize,
    pub height: usize,
}

impl<W: WidgetRef> WidgetRef for KnownSizeWrapper<W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.inner.render_ref(area, buf);
    }
}

impl<W: WidgetRef> KnownSize for KnownSizeWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W> KnownSizeWrapper<W> {
    /// Create a new `KnownSizeWrapper` with the given widget and size.
    pub fn new(inner: W, width: usize, height: usize) -> Self {
        Self {
            inner,
            width,
            height,
        }
    }
}
#[cfg(test)]
mod tests {
    use ratatui::{buffer::Buffer, layout::Rect};

    use super::*;

    struct TestWidget;

    impl WidgetRef for TestWidget {
        fn render_ref(&self, _area: Rect, _buf: &mut Buffer) {
            "Hello".render_ref(_area, _buf);
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
        wrapper.render_ref(buffer.area, &mut buffer);
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
