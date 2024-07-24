use ratatui::layout::{Position, Size};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ScrollViewState {
    /// The offset is the number of rows and columns to shift the scroll view by.
    pub(crate) offset: Position,
    /// The size of the scroll view. Not set until the first render call.
    pub(crate) size: Option<Size>,
    /// The size of a page of the scroll view. Not set until the first render call.
    pub(crate) page_size: Option<Size>,
}

impl ScrollViewState {
    /// Create a new scroll view state with an offset of (0, 0)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new scroll view state with the given offset
    pub fn with_offset(offset: Position) -> Self {
        Self {
            offset,
            ..Default::default()
        }
    }

    /// Set the offset of the scroll view state
    pub fn set_offset(&mut self, offset: Position) {
        self.offset = offset;
    }

    /// Get the offset of the scroll view state
    pub fn offset(&self) -> Position {
        self.offset
    }

    /// Move the scroll view state up by one row
    pub fn scroll_up(&mut self) {
        self.offset.y = self.offset.y.saturating_sub(1);
    }

    /// Move the scroll view state down by one row
    pub fn scroll_down(&mut self) {
        self.offset.y = self.offset.y.saturating_add(1);
    }

    /// Move the scroll view state down by one page
    pub fn scroll_page_down(&mut self) {
        let page_size = self.page_size.map_or(1, |size| size.height);
        // we subtract 1 to ensure that there is a one row overlap between pages
        self.offset.y = self.offset.y.saturating_add(page_size).saturating_sub(1);
    }

    /// Move the scroll view state up by one page
    pub fn scroll_page_up(&mut self) {
        let page_size = self.page_size.map_or(1, |size| size.height);
        // we add 1 to ensure that there is a one row overlap between pages
        self.offset.y = self.offset.y.saturating_add(1).saturating_sub(page_size);
    }

    /// Move the scroll view state left by one column
    pub fn scroll_left(&mut self) {
        self.offset.x = self.offset.x.saturating_sub(1);
    }

    /// Move the scroll view state right by one column
    pub fn scroll_right(&mut self) {
        self.offset.x = self.offset.x.saturating_add(1);
    }

    /// Move the scroll view state to the top of the buffer
    pub fn scroll_to_top(&mut self) {
        self.offset = (0, 0).into();
    }

    /// Move the scroll view state to the bottom of the buffer
    pub fn scroll_to_bottom(&mut self) {
        // the render call will adjust the offset to ensure that we don't scroll past the end of
        // the buffer, so we can set the offset to the maximum value here
        let bottom = self
            .size
            .map_or(u16::MAX, |size| size.height.saturating_sub(1));
        self.offset.y = bottom;
    }
}
