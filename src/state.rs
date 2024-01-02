use derive_getters::Getters;
use ratatui::prelude::*;

#[derive(Clone, Debug, Default, Getters)]
pub struct PopupState {
    pub area: Option<Rect>,
    pub mouse_state: MouseState,
}

#[derive(Clone, Debug, Default)]
pub enum MouseState {
    #[default]
    None,
    Dragging {
        col_offset: u16,
        row_offset: u16,
    },
}

impl PopupState {
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

    /// Set the state to dragging if the mouse click is in the popup title
    pub fn mouse_down(&mut self, col: u16, row: u16) {
        if let Some(area) = self.area {
            if area.y == row && area.x <= col && col <= area.right() {
                self.mouse_state = MouseState::Dragging {
                    col_offset: col.saturating_sub(area.x),
                    row_offset: row.saturating_sub(area.y),
                };
            }
        }
    }

    /// Set the state to not dragging
    pub fn mouse_up(&mut self, _col: u16, _row: u16) {
        self.mouse_state = MouseState::None;
    }

    /// Move the popup if the state is dragging
    pub fn mouse_drag(&mut self, col: u16, row: u16) {
        if let MouseState::Dragging {
            col_offset,
            row_offset,
        } = self.mouse_state
        {
            if let Some(area) = self.area {
                let x = col.saturating_sub(col_offset);
                let y = row.saturating_sub(row_offset);
                self.area.replace(Rect { x, y, ..area });
            }
        }
    }
}
