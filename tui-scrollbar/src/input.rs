//! Input types and interaction state for scrollbars.
//!
//! ## Design notes
//!
//! These types are intentionally backend-agnostic and small. The widget does not own scroll state;
//! it returns a [`ScrollCommand`] so the application can decide how to apply offsets. This keeps
//! the API compatible with any event loop and makes it easy to test.

/// Action requested by a pointer or wheel event.
///
/// Apply these to your stored offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollCommand {
    /// Update the content offset in the same logical units you supplied.
    SetOffset(usize),
}

/// Axis for scroll wheel events.
///
/// The scrollbar ignores wheel events that do not match its orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAxis {
    /// Wheel scroll in the vertical direction.
    Vertical,
    /// Wheel scroll in the horizontal direction.
    Horizontal,
}

/// Pointer button used for interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    /// Primary pointer button (usually left mouse button).
    Primary,
}

/// Kind of pointer interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerEventKind {
    /// Pointer pressed down.
    Down,
    /// Pointer moved while pressed down.
    Drag,
    /// Pointer released.
    Up,
}

/// Pointer input in terminal cell coordinates.
///
/// Use this to describe a pointer action relative to the scrollbar area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerEvent {
    /// Column of the event, in terminal cells.
    pub column: u16,
    /// Row of the event, in terminal cells.
    pub row: u16,
    /// Event kind.
    pub kind: PointerEventKind,
    /// Pointer button.
    pub button: PointerButton,
}

/// Scroll wheel input with an axis and a signed delta.
///
/// Positive deltas scroll down/right, negative deltas scroll up/left. The `column` and `row` are
/// used to ignore wheel events outside the scrollbar area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollWheel {
    /// Axis the wheel is scrolling.
    pub axis: ScrollAxis,
    /// Signed delta. Positive values scroll down/right.
    pub delta: isize,
    /// Column where the wheel event occurred.
    pub column: u16,
    /// Row where the wheel event occurred.
    pub row: u16,
}

/// Backend-agnostic input event for a scrollbar.
///
/// Use this in input handling when you want to stay backend-agnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollEvent {
    /// Pointer down/drag/up events.
    Pointer(PointerEvent),
    /// Scroll wheel input.
    ScrollWheel(ScrollWheel),
}

/// Drag state that should persist between frames.
///
/// Store this in your app state so drags remain active across draw calls.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScrollBarInteraction {
    pub(crate) drag_state: DragState,
}

/// Internal drag capture state stored by [`ScrollBarInteraction`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DragState {
    /// No active drag.
    #[default]
    Idle,
    /// A drag is active; `grab_offset` is in subcells.
    Dragging { grab_offset: usize },
}

impl ScrollBarInteraction {
    /// Creates a fresh interaction state with no active drag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts a drag by recording the grab offset in subcells.
    ///
    /// This keeps the pointer anchored to the same point within the thumb while dragging.
    pub(crate) fn start_drag(&mut self, grab_offset: usize) {
        self.drag_state = DragState::Dragging { grab_offset };
    }

    /// Stops any active drag and returns to the idle state.
    pub(crate) fn stop_drag(&mut self) {
        self.drag_state = DragState::Idle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_idle() {
        let interaction = ScrollBarInteraction::new();
        assert_eq!(interaction.drag_state, DragState::Idle);
    }

    #[test]
    fn records_grab_offset_on_start_drag() {
        let mut interaction = ScrollBarInteraction::new();
        interaction.start_drag(6);
        assert_eq!(
            interaction.drag_state,
            DragState::Dragging { grab_offset: 6 }
        );
    }

    #[test]
    fn resets_state_on_stop_drag() {
        let mut interaction = ScrollBarInteraction::new();
        interaction.start_drag(3);
        interaction.stop_drag();
        assert_eq!(interaction.drag_state, DragState::Idle);
    }

    // ScrollViewState is in tui-scrollview; only exercise scrollbar input here.
}
