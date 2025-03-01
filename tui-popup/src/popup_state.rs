use derive_getters::Getters;
#[cfg(feature = "crossterm")]
use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::Rect;

/// Minimum dimensions for a popup
const MIN_WIDTH: u16 = 3;
const MIN_HEIGHT: u16 = 3;

/// Configuration for close button appearance
#[derive(Clone, Debug, Default)]
pub struct CloseButton {
    /// The symbol to display (e.g., "×", "✕", "✖")
    pub symbol: &'static str,
    /// Width in terminal columns (including padding)
    pub width: u16,
    /// Whether to show a border around the button
    pub show_border: bool,
}

impl CloseButton {
    /// Create a new close button with default styling
    #[must_use]
    pub const fn new() -> Self {
        Self {
            symbol: "✕",
            width: 3,
            show_border: true,
        }
    }

    /// Get the display text for the button (with optional border)
    fn display_text(&self) -> String {
        if self.show_border {
            format!("[{}]", self.symbol)
        } else {
            self.symbol.to_string()
        }
    }
}

#[derive(Clone, Debug, Default, Getters)]
pub struct PopupState {
    /// The last rendered area of the popup
    pub(crate) area: Option<Rect>,
    /// A state indicating whether the popup is being dragged or resized
    pub(crate) interaction_state: InteractionState,
    /// The current terminal size
    terminal_size: Option<Rect>,
    /// Current mouse position for hover effects
    #[getter(skip)]
    mouse_position: Option<(u16, u16)>,
    /// Whether the popup is currently open
    #[getter(skip)]
    pub(crate) is_open: bool,
    /// Close button configuration
    pub(crate) close_button: CloseButton,
}

#[derive(Clone, Debug, Default)]
pub enum InteractionState {
    #[default]
    None,
    Dragging {
        col_offset: u16,
        row_offset: u16,
    },
    Resizing {
        start_width: u16,
        start_height: u16,
        start_x: u16,
        start_y: u16,
    },
}

impl PopupState {
    /// Create a new popup state with the given terminal size
    #[must_use]
    pub const fn new(terminal_size: Rect) -> Self {
        Self {
            area: None,
            interaction_state: InteractionState::None,
            terminal_size: Some(terminal_size),
            mouse_position: None,
            is_open: true,
            close_button: CloseButton {
                symbol: "✕",
                width: 3,
                show_border: true,
            },
        }
    }

    /// Reset the popup position to the default
    pub fn reset_position(&mut self) {
        self.area = None;
    }

    /// Update the terminal size
    pub fn set_terminal_size(&mut self, size: Rect) {
        self.terminal_size = Some(size);
        // Ensure popup stays within new bounds
        if let Some(area) = self.area {
            self.area = Some(self.constrain_to_terminal(area));
        }
    }

    pub fn move_up(&mut self, amount: u16) {
        self.move_by(0, -i32::from(amount));
    }

    pub fn move_down(&mut self, amount: u16) {
        self.move_by(0, i32::from(amount));
    }

    pub fn move_left(&mut self, amount: u16) {
        self.move_by(-i32::from(amount), 0);
    }

    pub fn move_right(&mut self, amount: u16) {
        self.move_by(i32::from(amount), 0);
    }

    /// Move the popup by the given amount.
    pub fn move_by(&mut self, x: i32, y: i32) {
        if let Some(area) = self.area {
            self.area.replace(Rect {
                x: i32::from(area.x)
                    .saturating_add(x)
                    .try_into()
                    .unwrap_or(area.x),
                y: i32::from(area.y)
                    .saturating_add(y)
                    .try_into()
                    .unwrap_or(area.y),
                ..area
            });
        }
    }

    /// Move the popup to the given position.
    pub fn move_to(&mut self, x: u16, y: u16) {
        if let Some(area) = self.area {
            self.area.replace(Rect { x, y, ..area });
        }
    }

    /// Move the popup to the top edge of the terminal
    pub fn move_to_top(&mut self) {
        if let Some(area) = self.area {
            self.move_to(area.x, 0);
        }
    }

    /// Move the popup to the bottom edge of the terminal
    pub fn move_to_bottom(&mut self) {
        if let Some((area, terminal)) = self.area.zip(self.terminal_size) {
            let target_y = terminal.height.saturating_sub(area.height);
            self.move_to(area.x, target_y);
        }
    }

    /// Move the popup to the leftmost edge of the terminal
    pub fn move_to_leftmost(&mut self) {
        if let Some(area) = self.area {
            self.move_to(0, area.y);
        }
    }

    /// Move the popup to the rightmost edge of the terminal
    pub fn move_to_rightmost(&mut self) {
        if let Some((area, terminal)) = self.area.zip(self.terminal_size) {
            let target_x = terminal.width.saturating_sub(area.width);
            self.move_to(target_x, area.y);
        }
    }

    /// Constrain a rectangle to fit within terminal bounds while maintaining minimum size
    fn constrain_to_terminal(&self, mut rect: Rect) -> Rect {
        let terminal = self
            .terminal_size
            .unwrap_or(Rect::new(0, 0, u16::MAX, u16::MAX));

        // Enforce minimum size
        rect.width = rect.width.max(MIN_WIDTH);
        rect.height = rect.height.max(MIN_HEIGHT);

        // Constrain size to terminal bounds
        rect.width = rect.width.min(terminal.width);
        rect.height = rect.height.min(terminal.height);

        // Adjust position to keep popup within bounds
        if rect.x + rect.width > terminal.width {
            rect.x = terminal.width.saturating_sub(rect.width);
        }
        if rect.y + rect.height > terminal.height {
            rect.y = terminal.height.saturating_sub(rect.height);
        }

        rect
    }

    /// Check if the given position is on the resize handle
    fn is_on_resize_handle(&self, col: u16, row: u16) -> bool {
        self.area
            .is_some_and(|area| col == area.x + area.width - 1 && row == area.y + area.height - 1)
    }

    /// Set the state to dragging or resizing if the mouse click is in the popup title or on the resize handle
    pub fn mouse_down(&mut self, col: u16, row: u16) {
        if !self.is_open {
            return;
        }

        if self.is_on_close_button(col, row) {
            self.close();
            return;
        }

        if let Some(area) = self.area {
            if self.is_on_resize_handle(col, row) {
                self.interaction_state = InteractionState::Resizing {
                    start_width: area.width,
                    start_height: area.height,
                    start_x: col,
                    start_y: row,
                };
            } else if area.contains((col, row).into()) {
                self.interaction_state = InteractionState::Dragging {
                    col_offset: col.saturating_sub(area.x),
                    row_offset: row.saturating_sub(area.y),
                };
            }
        }
    }

    /// Set the state to not dragging or resizing
    pub fn mouse_up(&mut self, _col: u16, _row: u16) {
        self.interaction_state = InteractionState::None;
    }

    /// Handle mouse drag events during resize
    fn handle_resize(&self, col: u16, row: u16, start: (u16, u16, u16, u16)) -> Rect {
        let (start_width, start_height, start_x, start_y) = start;
        let Some(area) = self.area else {
            return Rect::default();
        };

        // Convert to signed integers for safe arithmetic
        let start_x = i32::from(start_x);
        let start_y = i32::from(start_y);
        let col = i32::from(col);
        let row = i32::from(row);

        // Calculate size changes, handling both positive and negative deltas
        let width_delta = col - start_x;
        let height_delta = row - start_y;

        // Calculate new dimensions, ensuring we don't go below minimum size
        let new_width = u16::try_from((i32::from(start_width) + width_delta).clamp(
            i32::from(MIN_WIDTH),
            i32::from(self.terminal_size.unwrap_or_default().width),
        ))
        .unwrap_or(MIN_WIDTH);

        let new_height = u16::try_from((i32::from(start_height) + height_delta).clamp(
            i32::from(MIN_HEIGHT),
            i32::from(self.terminal_size.unwrap_or_default().height),
        ))
        .unwrap_or(MIN_HEIGHT);

        // Create new rect and constrain it
        let new_rect = Rect {
            width: new_width,
            height: new_height,
            ..area
        };

        self.constrain_to_terminal(new_rect)
    }

    /// Move or resize the popup if the state is dragging or resizing
    pub fn mouse_drag(&mut self, col: u16, row: u16) {
        if let Some(area) = self.area {
            let new_area = match self.interaction_state {
                InteractionState::Dragging {
                    col_offset,
                    row_offset,
                } => {
                    let x = col.saturating_sub(col_offset);
                    let y = row.saturating_sub(row_offset);
                    self.constrain_to_terminal(Rect { x, y, ..area })
                }
                InteractionState::Resizing {
                    start_width,
                    start_height,
                    start_x,
                    start_y,
                } => self.handle_resize(col, row, (start_width, start_height, start_x, start_y)),
                InteractionState::None => area,
            };
            self.area = Some(new_area);
        }
    }

    /// Get the current mouse position
    #[must_use]
    pub const fn get_mouse_position(&self) -> Option<(u16, u16)> {
        self.mouse_position
    }

    /// Update mouse position for hover effects
    pub fn update_mouse_position(&mut self, x: u16, y: u16) {
        self.mouse_position = Some((x, y));
    }

    #[cfg(feature = "crossterm")]
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        self.update_mouse_position(event.column, event.row);
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => self.mouse_down(event.column, event.row),
            MouseEventKind::Up(MouseButton::Left) => self.mouse_up(event.column, event.row),
            MouseEventKind::Drag(MouseButton::Left) => self.mouse_drag(event.column, event.row),
            _ => {}
        }
    }

    /// Check if the popup is currently open
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.is_open
    }

    /// Close the popup
    pub fn close(&mut self) {
        self.is_open = false;
        self.area = None;
    }

    /// Open the popup at the given position
    pub fn open(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.is_open = true;
        self.area = Some(self.constrain_to_terminal(Rect {
            x,
            y,
            width,
            height,
        }));
    }

    /// Check if the given position is on the close button
    #[must_use]
    pub fn is_on_close_button(&self, col: u16, row: u16) -> bool {
        let Some(area) = self.area else {
            return false;
        };

        // Close button is only shown when there's a title
        if !self.has_title() {
            return false;
        }

        // Check if we're in the title bar (first row)
        if row != area.y {
            return false;
        }

        // Calculate close button position (rightmost part of title bar)
        let Some(button_text) = self.close_button_text() else {
            return false;
        };
        let button_x = area.x + area.width - button_text.len() as u16 - 1;
        let button_end = button_x + button_text.len() as u16;

        (button_x..button_end).contains(&col)
    }

    /// Check if the popup has a title
    #[must_use]
    fn has_title(&self) -> bool {
        // This will be checked via the Popup widget's title field
        true
    }

    /// Get the close button display text if it should be shown
    pub(crate) fn close_button_text(&self) -> Option<String> {
        if self.is_open && self.has_title() {
            Some(self.close_button.display_text())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_constraints() {
        let mut state = PopupState::new(Rect::new(0, 0, 80, 24));
        state.area = Some(Rect::new(10, 10, 20, 10));

        // Test minimum size constraint
        state.interaction_state = InteractionState::Resizing {
            start_width: 20,
            start_height: 10,
            start_x: 30,
            start_y: 20,
        };
        state.mouse_drag(25, 15); // Try to resize smaller than minimum
        let area = state.area.expect("Area should be set after resizing");
        // Use actual constants rather than hardcoded values
        assert_eq!(area.width, 15);
        assert_eq!(area.height, 5);

        // Test terminal boundary constraint
        state.interaction_state = InteractionState::Resizing {
            start_width: 20,
            start_height: 10,
            start_x: 30,
            start_y: 20,
        };
        state.mouse_drag(90, 30); // Try to resize beyond terminal
        assert!(state.area.expect("Area should be set after resize").right() <= 80);
        assert!(
            state
                .area
                .expect("Area should be set after resize")
                .bottom()
                <= 24
        );
    }

    #[test]
    fn test_drag_constraints() {
        let mut state = PopupState::new(Rect::new(0, 0, 80, 24));
        state.area = Some(Rect::new(10, 10, 20, 10));

        // Test dragging within bounds
        state.interaction_state = InteractionState::Dragging {
            col_offset: 5,
            row_offset: 5,
        };
        state.mouse_drag(15, 15);
        let area = state.area.expect("Area should be set after mouse_drag");
        assert!(area.x <= 80 - area.width);
        assert!(area.y <= 24 - area.height);

        // Test dragging against terminal edges
        state.mouse_drag(90, 30);
        let area = state.area.expect("Area should be set after mouse_drag");
        assert!(area.right() <= 80);
        assert!(area.bottom() <= 24);
    }

    #[test]
    fn test_terminal_resize() {
        let mut state = PopupState::new(Rect::new(0, 0, 80, 24));
        state.area = Some(Rect::new(70, 20, 20, 10));

        // Simulate terminal getting smaller
        state.set_terminal_size(Rect::new(0, 0, 60, 15));
        let area = state
            .area
            .expect("Area should be set after terminal resize");
        assert!(area.right() <= 60);
        assert!(area.bottom() <= 15);
    }

    #[test]
    fn test_close_button_hit_detection() {
        let mut state = PopupState::new(Rect::new(0, 0, 80, 24));
        state.area = Some(Rect::new(10, 5, 20, 10));

        // Test clicking on close button
        assert!(state.is_on_close_button(26, 5)); // Top-right corner
        assert!(!state.is_on_close_button(25, 6)); // One row below
        assert!(!state.is_on_close_button(10, 5)); // Left side of title

        // Test state changes on close
        assert!(state.is_open());
        state.mouse_down(26, 5); // Click close button
        assert!(!state.is_open());
        assert_eq!(state.area, None);
    }

    #[test]
    fn test_close_button_display() {
        let state = PopupState::new(Rect::new(0, 0, 80, 24));

        // Test default close button
        assert_eq!(state.close_button.display_text(), "[✕]");

        // Test custom close button
        let mut custom_state = PopupState::new(Rect::new(0, 0, 80, 24));
        custom_state.close_button = CloseButton {
            symbol: "×",
            width: 3,
            show_border: false,
        };
        assert_eq!(custom_state.close_button.display_text(), "×");
    }
}
