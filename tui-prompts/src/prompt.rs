use std::iter::once;

use itertools::chain;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::StatefulWidget;

use crate::Status;

/// A prompt that can be drawn to a terminal.
pub trait Prompt: StatefulWidget {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the [`StatefulWidget`] trait implementation as we need the [`Frame`]
    /// to set the cursor position.
    ///
    /// [`StatefulWidget`]: ratatui::widgets::StatefulWidget
    /// [`Frame`]: ratatui::Frame
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State);
}

/// The focus state of a prompt.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum FocusState {
    #[default]
    Unfocused,
    Focused,
}

impl FocusState {
    pub fn toggle(self) -> Self {
        match self {
            FocusState::Unfocused => FocusState::Focused,
            FocusState::Focused => FocusState::Unfocused,
        }
    }
}

pub trait StateCommon {
    /// The status of the prompt.
    fn status(&self) -> Status;

    /// A mutable reference to the status of the prompt.
    fn status_mut(&mut self) -> &mut Status;

    /// A mutable reference to the focus state of the prompt.
    fn focus_state_mut(&mut self) -> &mut FocusState;

    /// The focus state of the prompt.
    fn focus_state(&self) -> FocusState;

    /// The focus state of the prompt.
    fn is_valid_value(&self) -> bool;

    /// Sets the focus state of the prompt to [`Focus::Focused`].
    fn focus(&mut self) {
        *self.focus_state_mut() = FocusState::Focused;
    }

    /// Sets the focus state of the prompt to [`Focus::Unfocused`].
    fn blur(&mut self) {
        *self.focus_state_mut() = FocusState::Unfocused;
    }

    /// Whether the prompt is focused.
    fn is_focused(&self) -> bool {
        self.focus_state() == FocusState::Focused
    }

    /// If the value is valid, set the current state to done.
    fn complete(&mut self) {
        if self.is_valid_value() {
            *self.status_mut() = Status::Done;
        };
    }

    /// The active position within the prompt.
    /// This may indicate a cursor position or a subprompt.
    fn position(&self) -> usize;

    /// A mutable reference to the active position in the prompt.
    fn position_mut(&mut self) -> &mut usize;

    /// The index of the last character in the prompt.
    fn final_position(&self) -> usize;

    /// Set the position to the last index in the prompt.
    fn move_end(&mut self) {
        *self.position_mut() = self.final_position();
    }

    /// Set the position to the first index in the prompt.
    fn move_start(&mut self) {
        *self.position_mut() = 0;
    }

    /// Whether the current position is at the first position in the prompt.
    fn is_at_start(&self) -> bool {
        self.position() == 0
    }

    /// Whether the current position is at the last position in the prompt.
    fn is_at_end(&self) -> bool {
        self.position() == self.final_position()
    }
}

pub trait CursorControl {
    /// The cursor position of the prompt.
    fn cursor(&self) -> (u16, u16);

    /// A mutable reference to the cursor position of the prompt.
    fn cursor_mut(&mut self) -> &mut (u16, u16);
}

/// The state of a prompt.
///
/// Keybindings:
/// - Enter: Complete
/// - Esc | Ctrl+C: Abort
/// - Left | Ctrl+B: Move cursor left
/// - Right | Ctrl+F: Move cursor right
/// - Home | Ctrl+A: Move cursor to start of line
/// - End | Ctrl+E: Move cursor to end of line
/// - Backspace | Ctrl+H: Delete character before cursor
/// - Delete | Ctrl+D: Delete character after cursor
/// - Ctrl+K: Delete from cursor to end of line
/// - Ctrl+U: Delete from cursor to start of line
pub trait TextualState: StateCommon {
    /// The value of the prompt.
    fn value(&self) -> &str;

    /// A mutable reference to the value of the prompt.
    fn value_mut(&mut self) -> &mut String;

    /// The length of the textual value held in the state.
    fn len(&self) -> usize;

    /// Whether the textual value held by the state is empty.
    fn is_empty(&self) -> bool {
        self.value().len() == 0
    }

    /// Move the position to the right, up to and including the last possible position.
    fn move_right(&mut self) {
        *self.position_mut() = self.position().saturating_add(1).min(self.final_position());
    }

    /// Move the position to the left, up to and including the first position.
    fn move_left(&mut self) {
        *self.position_mut() = self.position().saturating_sub(1);
    }

    /// Move the position to the left, up to and including the first position.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        match (key_event.code, key_event.modifiers) {
            (KeyCode::Enter, _) => self.complete(),
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => self.move_left(),
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.move_right(),
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.move_start(),
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => self.move_end(),
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.backspace();
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.delete(),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.kill(),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.truncate(),
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => self.push(c),
            _ => {}
        }
    }

    fn delete(&mut self) {
        if self.is_at_end() {
            return;
        }
        let position = self.position();
        *self.value_mut() = chain!(
            self.value().chars().take(position),
            self.value().chars().skip(position + 1)
        )
        .collect();
    }

    fn backspace(&mut self) {
        if self.is_at_start() {
            return;
        }
        let position = self.position();
        *self.value_mut() = chain!(
            self.value().chars().take(position.saturating_sub(1)),
            self.value().chars().skip(position)
        )
        .collect();
        *self.position_mut() = position.saturating_sub(1);
    }

    fn kill(&mut self) {
        let position = self.position();
        self.value_mut().truncate(position);
    }

    fn truncate(&mut self) {
        self.value_mut().clear();
        *self.position_mut() = 0;
    }

    fn is_valid_char(&self, c: char) -> bool;

    fn push(&mut self, c: char) {
        if !self.is_valid_char(c) {
            return;
        }
        // We cannot use String::insert() as it operates on bytes, which can lead to incorrect
        // modifications with multibyte characters. Instead, we handle text
        // manipulation at the character level using Rust's char type for Unicode
        // correctness. Check docs of String::insert() and String::chars() for futher info.
        *self.value_mut() = chain![
            self.value().chars().take(self.position()),
            once(c),
            self.value().chars().skip(self.position())
        ]
        .collect();
        self.move_right();
    }
}

pub trait CompoundState<T, U>: StateCommon
where
    T: TextualState + Clone,
{
    fn move_right(&mut self) {
        let current = self.current_element_mut();
        if current.is_at_end() {
            self.jump_right();
        } else {
            current.move_right();
        }
    }

    fn jump_right(&mut self) {
        if !self.is_at_end() {
            *self.position_mut() = self.position().saturating_add(1);
            self.current_element_mut().focus();
        }
    }

    fn move_left(&mut self) {
        let current = self.current_element_mut();
        if current.is_at_start() {
            self.jump_left();
        } else {
            current.move_left();
        }
    }

    fn jump_left(&mut self) {
        *self.position_mut() = self.position().saturating_sub(1);
        self.current_element_mut().focus();
    }

    /// Update the value held by the CompoundState from its native type.
    fn with_value(&mut self, val: U);

    /// Return the value held by the CompoundState in its native type, if possible.
    fn output_value(&self) -> Result<U, ()> {
        self.output_value_from_elements(self.elements())
    }

    fn current_element(&self) -> &T;

    fn current_element_mut(&mut self) -> &mut T;

    fn elements(&self) -> Vec<T>;

    fn elements_mut(&mut self) -> &mut Vec<T>;

    /// The number of child elements in the CompoundState.
    fn element_count(&self) -> usize {
        self.elements().len()
    }

    /// Given a vector of child elements, return the value held by the CompoundState in its 
    /// native type, if possible.
    fn output_value_from_elements(&self, elements: Vec<T>) -> Result<U, ()>;

    fn push(&mut self, c: char) {
        if self.current_element().is_at_end() {
            return;
        }
        self.current_element_mut().push(c);
        self.move_right();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_symbols() {
        assert_eq!(Status::Pending.symbol(), "?".cyan());
        assert_eq!(Status::Aborted.symbol(), "✘".red());
        assert_eq!(Status::Done.symbol(), "✔".green());
    }
}
