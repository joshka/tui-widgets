use ratatui::layout::Position;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ScrollViewState {
    /// The offset is the number of rows and columns to shift the scroll view by.
    pub(crate) offset: Position,
}

impl Default for ScrollViewState {
    fn default() -> Self {
        Self {
            offset: (0, 0).into(),
        }
    }
}

impl ScrollViewState {
    /// Create a new scroll view state with an offset of (0, 0)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new scroll view state with the given offset
    pub fn with_offset(offset: Position) -> Self {
        Self { offset }
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
        // the buffer, so we can just set the offset to the maximum value here
        self.offset = (0, u16::MAX).into();
    }
}
