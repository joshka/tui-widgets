use std::{cmp::min, fmt};

use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Stylize,
    style::{Modifier, Style},
    symbols::border::Set,
    text::Line,
    widgets::{Block, Borders, Clear, StatefulWidget, Widget, WidgetRef},
};

use crate::{KnownSize, PopupState};

/// Configuration for a popup.
///
/// This struct is used to configure a [`Popup`]. It can be created using
/// [`Popup::new`](Popup::new).
///
/// # Example
///
/// ```rust
/// use ratatui::{prelude::*, symbols::border};
/// use tui_popup::Popup;
///
/// fn render_popup(frame: &mut Frame) {
///     let popup = Popup::new("Press any key to exit")
///         .title("tui-popup demo")
///         .style(Style::new().white().on_blue())
///         .border_set(border::ROUNDED)
///         .border_style(Style::new().bold());
///     frame.render_widget(&popup, frame.area());
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResizeHandle {
    /// The symbol to use for the resize handle (defaults to "⟋")
    pub symbol: &'static str,
    /// Custom style for the resize handle
    pub style: Style,
    /// Whether to show hover effects
    pub show_hover: bool,
    /// The active area around the handle (defaults to 1)
    pub active_margin: u16,
}

impl Default for ResizeHandle {
    fn default() -> Self {
        Self {
            symbol: "⟋",
            style: Style::default().bold(),
            show_hover: true,
            active_margin: 1,
        }
    }
}

#[derive(Setters)]
#[setters(into)]
#[non_exhaustive]
pub struct Popup<'content, W> {
    /// The body of the popup.
    #[setters(skip)]
    pub body: W,
    /// The title of the popup.
    pub title: Line<'content>,
    /// The style to apply to the entire popup.
    pub style: Style,
    /// The borders of the popup.
    pub borders: Borders,
    /// The symbols used to render the border.
    pub border_set: Set,
    /// Border style
    pub border_style: Style,
    /// Configuration for the resize handle
    pub resize_handle: ResizeHandle,
}

impl<W> fmt::Debug for Popup<'_, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // body does not implement Debug, so we can't use #[derive(Debug)]
        f.debug_struct("Popup")
            .field("body", &"...")
            .field("title", &self.title)
            .field("style", &self.style)
            .field("borders", &self.borders)
            .field("border_set", &self.border_set)
            .field("border_style", &self.border_style)
            .field("resize_handle", &self.resize_handle)
            .finish()
    }
}

impl<W: PartialEq> PartialEq for Popup<'_, W> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.title == other.title
            && self.style == other.style
            && self.borders == other.borders
            && self.border_set == other.border_set
            && self.border_style == other.border_style
            && self.resize_handle == other.resize_handle
    }
}

impl<W> Popup<'_, W> {
    /// Create a new popup with the given title and body with all the borders.
    ///
    /// # Parameters
    ///
    /// - `body` - The body of the popup. This can be any type that can be converted into a
    ///   [`Text`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_popup::Popup;
    ///
    /// let popup = Popup::new("Press any key to exit").title("tui-popup demo");
    /// ```
    pub fn new(body: W) -> Self {
        Self {
            body,
            borders: Borders::ALL,
            border_set: Set::default(),
            border_style: Style::default(),
            title: Line::default(),
            style: Style::default(),
            resize_handle: ResizeHandle::default(),
        }
    }
}

impl<W: KnownSize + WidgetRef> Widget for Popup<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidget::render(&self, area, buf, &mut state);
    }
}

impl<W: KnownSize + WidgetRef> Widget for &Popup<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<W: KnownSize + WidgetRef> StatefulWidget for Popup<'_, W> {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl<W: KnownSize + WidgetRef> StatefulWidget for &Popup<'_, W> {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = area.clamp(buf.area);
        state.set_terminal_size(area);

        let popup_area = self.popup_area(state, area);
        state.area.replace(popup_area);

        Clear.render(popup_area, buf);
        let block = Block::default()
            .borders(self.borders)
            .border_set(self.border_set)
            .border_style(self.border_style)
            .title(self.title.clone())
            .style(self.style);
        let inner_area = block.inner(popup_area);
        block.render(popup_area, buf);

        // Render enhanced resize handle in bottom-right corner
        if popup_area.width > 1 && popup_area.height > 1 {
            self.render_resize_handle(popup_area, buf, state);
        }

        self.body.render_ref(inner_area, buf);
    }
}

impl<W: KnownSize> Popup<'_, W> {
    fn popup_area(&self, state: &mut PopupState, area: Rect) -> Rect {
        if let Some(current) = state.area.take() {
            return current.clamp(area);
        }

        let has_top = self.borders.intersects(Borders::TOP);
        let has_bottom = self.borders.intersects(Borders::BOTTOM);
        let has_left = self.borders.intersects(Borders::LEFT);
        let has_right = self.borders.intersects(Borders::RIGHT);

        let border_height = usize::from(has_top) + usize::from(has_bottom);
        let border_width = usize::from(has_left) + usize::from(has_right);

        let height = self.body.height().saturating_add(border_height);
        let width = self.body.width().saturating_add(border_width);

        let height = u16::try_from(height).unwrap_or(area.height);
        let width = u16::try_from(width).unwrap_or(area.width);

        centered_rect(width, height, area)
    }

    /// Render the resize handle with enhanced visual feedback
    fn render_resize_handle(&self, area: Rect, buf: &mut Buffer, state: &PopupState) {
        let handle_x = area.x + area.width - 1;
        let handle_y = area.y + area.height - 1;

        // Determine if mouse is hovering over handle area
        let is_hovering = state.get_mouse_position().is_some_and(|mouse_pos| {
            let margin = self.resize_handle.active_margin;
            let hover_area = Rect {
                x: handle_x.saturating_sub(margin),
                y: handle_y.saturating_sub(margin),
                width: margin * 2 + 1,
                height: margin * 2 + 1,
            };
            hover_area.contains(mouse_pos.into())
        });

        // Base style enhanced with hover effect
        let handle_style = if is_hovering && self.resize_handle.show_hover {
            self.resize_handle
                .style
                .patch(Style::default().add_modifier(Modifier::REVERSED))
        } else {
            self.resize_handle.style
        };

        // Render the resize handle
        if let Some(cell) = buf.cell_mut((handle_x, handle_y)) {
            cell.set_symbol(self.resize_handle.symbol)
                .set_style(handle_style);
        }

        // Optional: Render subtle indicators around handle when hovering
        if is_hovering && self.resize_handle.show_hover {
            // Add subtle corner markers
            if let Some(cell) = buf.cell_mut((handle_x - 1, handle_y)) {
                cell.set_style(handle_style.patch(Style::default().dim()));
            }
            if let Some(cell) = buf.cell_mut((handle_x, handle_y - 1)) {
                cell.set_style(handle_style.patch(Style::default().dim()));
            }
        }
    }
}

/// Create a rectangle centered in the given area.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.width.saturating_sub(width) / 2,
        y: area.height.saturating_sub(height) / 2,
        width: min(width, area.width),
        height: min(height, area.height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use ratatui::style::Modifier;

    #[test]
    fn new() {
        let popup = Popup::new("Test Body");
        assert_eq!(
            popup,
            Popup {
                body: "Test Body", // &str is a widget
                borders: Borders::ALL,
                border_set: Set::default(),
                border_style: Style::default(),
                title: Line::default(),
                style: Style::default(),
                resize_handle: ResizeHandle::default(),
            }
        );
    }

    #[test]
    fn render() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
        let mut state = PopupState::default();

        // Create an expected buffer with the correct styles
        let mut expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────⟋    ",
            "                    ",
        ]);

        // Set the correct style for the resize handle (bold)
        let resize_pos = (15, 3);
        if let Some(cell) = expected.cell_mut((resize_pos.0, resize_pos.1)) {
            cell.set_style(Style::default().add_modifier(Modifier::BOLD));
        }

        // Rest of test...
        let popup = Popup::new("Hello World").title("Title");
        StatefulWidget::render(&popup, buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, expected, "\nBuffer contents differ");

        // ...existing test cases...
        // Check that a popup ref can render a widget defined by a ref value (e.g. `&str`).
        let popup = Popup::new("Hello World").title("Title");
        StatefulWidget::render(&popup, buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, expected);

        // Check that a popup ref can render a widget defined by a owned value (e.g. `String`).
        let popup = Popup::new("Hello World".to_string()).title("Title");
        StatefulWidget::render(&popup, buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, expected);

        // Check that an owned popup can render a widget defined by a ref value (e.g. `&str`).
        let popup = Popup::new("Hello World").title("Title");
        StatefulWidget::render(popup, buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, expected);

        // Check that an owned popup can render a widget defined by a owned value (e.g. `String`).
        let popup = Popup::new("Hello World".to_string()).title("Title");
        StatefulWidget::render(popup, buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, expected);

        // Check that a popup ref can render a ref value (e.g. `&str`), with default state.
        let popup = Popup::new("Hello World").title("Title");
        Widget::render(&popup, buffer.area, &mut buffer);
        assert_eq!(buffer, expected);

        // Check that a popup ref can render an owned value (e.g. `String`), with default state.
        let popup = Popup::new("Hello World".to_string()).title("Title");
        Widget::render(&popup, buffer.area, &mut buffer);
        assert_eq!(buffer, expected);

        // Check that an owned popup can render a ref value (e.g. `&str`), with default state.
        let popup = Popup::new("Hello World").title("Title");
        Widget::render(popup, buffer.area, &mut buffer);
        assert_eq!(buffer, expected);

        // Check that an owned popup can render an owned value (e.g. `String`), with default state.
        let popup = Popup::new("Hello World".to_string()).title("Title");
        Widget::render(popup, buffer.area, &mut buffer);
        assert_eq!(buffer, expected);
    }
}
