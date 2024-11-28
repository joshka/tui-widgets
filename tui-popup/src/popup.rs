use std::{cmp::min, fmt};

use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
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
///     frame.render_widget(&popup, frame.size());
/// }
/// ```
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
    }
}

impl<'content, W> Popup<'content, W> {
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

        self.body.render_ref(inner_area, buf);
    }
}

impl<'content, W: KnownSize> Popup<'content, W> {
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
    use pretty_assertions::assert_eq;

    use super::*;

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
            }
        );
    }

    #[test]
    fn render() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
        let mut state = PopupState::default();
        let expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────┘    ",
            "                    ",
        ]);

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
