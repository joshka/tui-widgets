use derive_getters::Getters;
use ratatui::prelude::Rect;

#[cfg(feature = "crossterm")]
use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

#[derive(Clone, Debug, Default, Getters)]
pub struct PopupState {
    /// The last rendered area of the popup
    pub(crate) area: Option<Rect>,
    /// A state indicating whether the popup is being dragged or not
    pub(crate) drag_state: DragState,
}

#[derive(Clone, Debug, Default)]
pub enum DragState {
    #[default]
    NotDragging,
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
            if area.contains((col, row).into()) {
                self.drag_state = DragState::Dragging {
                    col_offset: col.saturating_sub(area.x),
                    row_offset: row.saturating_sub(area.y),
                };
            }
        }
    }

    /// Set the state to not dragging
    pub fn mouse_up(&mut self, _col: u16, _row: u16) {
        self.drag_state = DragState::NotDragging;
    }

    /// Move the popup if the state is dragging
    pub fn mouse_drag(&mut self, col: u16, row: u16) {
        if let DragState::Dragging {
            col_offset,
            row_offset,
        } = self.drag_state
        {
            if let Some(area) = self.area {
                let x = col.saturating_sub(col_offset);
                let y = row.saturating_sub(row_offset);
                self.area.replace(Rect { x, y, ..area });
            }
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
