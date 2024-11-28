use std::fmt::Debug;

use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use crate::SizedWidgetRef;

#[derive(Debug)]
pub struct SizedWrapper<W: Debug> {
    pub inner: W,
    pub width: usize,
    pub height: usize,
}

impl<W: WidgetRef + Debug> WidgetRef for SizedWrapper<W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.inner.render_ref(area, buf);
    }
}

impl<W: WidgetRef + Debug> SizedWidgetRef for SizedWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}
