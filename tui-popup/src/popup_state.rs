use derive_getters::Getters;
#[cfg(feature = "crossterm")]
use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::Rect;

/// Minimum dimensions for a popup
const MIN_WIDTH: u16 = 3;
const MIN_HEIGHT: u16 = 3;

#[derive(Clone, Debug, Default, Getters)]
pub struct PopupState {
    /// The last rendered area of the popup
    pub(crate) area: Option<Rect>,
    /// A state indicating whether the popup is being dragged or resized
    pub(crate) interaction_state: InteractionState,
    /// The current terminal size
    terminal_size: Option<Rect>,
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
        }
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
        let area = self.area.unwrap();

        // Convert to signed integers for safe arithmetic
        let start_x = i32::from(start_x);
        let start_y = i32::from(start_y);
        let col = i32::from(col);
        let row = i32::from(row);

        // Calculate size changes, handling both positive and negative deltas
        let width_delta = col - start_x;
        let height_delta = row - start_y;

        println!(
            "col: {}, start_x: {}, width_delta: {}",
            col, start_x, width_delta
        );
        println!(
            "start_width: {}, new calculated width: {}",
            start_width,
            i32::from(start_width) + width_delta
        );

        // Calculate new dimensions, ensuring we don't go below minimum size
        let new_width = u16::try_from(
            (i32::from(start_width) + width_delta)
                .max(i32::from(MIN_WIDTH))
                .min(i32::from(self.terminal_size.unwrap_or_default().width)),
        )
        .unwrap_or(MIN_WIDTH);

        let new_height = u16::try_from(
            (i32::from(start_height) + height_delta)
                .max(i32::from(MIN_HEIGHT))
                .min(i32::from(self.terminal_size.unwrap_or_default().height)),
        )
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

    #[cfg(feature = "crossterm")]
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => self.mouse_down(event.column, event.row),
            MouseEventKind::Up(MouseButton::Left) => self.mouse_up(event.column, event.row),
            MouseEventKind::Drag(MouseButton::Left) => self.mouse_drag(event.column, event.row),
            _ => {}
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

        // Test resize with valid dimensions
        state.interaction_state = InteractionState::Resizing {
            start_width: 20,
            start_height: 10,
            start_x: 29,
            start_y: 20,
        };

        // Move mouse to x=25, y=15
        // Width delta = 25 - 29 = -4, so new width = 20 + (-4) = 16
        state.mouse_drag(25, 15);
        let area = state.area.unwrap();
        assert_eq!(
            area.width, 16,
            "Width should be start_width + delta = 20 + (25-29) = 16"
        );
        assert_eq!(
            area.height, 5,
            "Height should be start_height + (row - start_y) = 10 + (15-20) = 5"
        );

        // Test significant resize reduction
        // Moving to x=20 means delta = 20 - 29 = -9, so width = 20 + (-9) = 11
        state.mouse_drag(20, 10);
        let area = state.area.unwrap();
        assert_eq!(
            area.width, 11,
            "Width should be start_width + delta = 20 + (20-29) = 11"
        );
        assert_eq!(
            area.height, MIN_HEIGHT,
            "Height should be clamped to MIN_HEIGHT"
        );

        // Move even further to test terminal boundary constraint
        state.mouse_drag(90, 30);
        let area = state.area.unwrap();
        assert!(area.right() <= 80);
        assert!(area.bottom() <= 24);
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
        let area = state.area.unwrap();
        assert!(area.x <= 80 - area.width);
        assert!(area.y <= 24 - area.height);

        // Test dragging against terminal edges
        state.mouse_drag(90, 30);
        let area = state.area.unwrap();
        assert!(area.right() <= 80);
        assert!(area.bottom() <= 24);
    }

    #[test]
    fn test_terminal_resize() {
        let mut state = PopupState::new(Rect::new(0, 0, 80, 24));
        state.area = Some(Rect::new(70, 20, 20, 10));

        // Simulate terminal getting smaller
        state.set_terminal_size(Rect::new(0, 0, 60, 15));
        let area = state.area.unwrap();
        assert!(area.right() <= 60);
        assert!(area.bottom() <= 15);
    }
}
