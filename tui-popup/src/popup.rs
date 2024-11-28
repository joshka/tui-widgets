use std::{cmp::min, fmt::Debug};

use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::border::Set,
    text::Line,
    widgets::{Block, Borders, Clear, StatefulWidget, Widget, WidgetRef},
};

use crate::{PopupState, SizedWidgetRef};

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
#[derive(Debug, Setters)]
#[setters(into)]
#[non_exhaustive]
pub struct Popup<'content, W: SizedWidgetRef> {
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

impl<W: SizedWidgetRef + PartialEq> PartialEq for Popup<'_, W> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.title == other.title
            && self.style == other.style
            && self.borders == other.borders
            && self.border_set == other.border_set
            && self.border_style == other.border_style
    }
}

impl<'content, W: SizedWidgetRef> Popup<'content, W> {
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

impl<W: SizedWidgetRef> WidgetRef for Popup<'_, W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<W: SizedWidgetRef> StatefulWidget for Popup<'_, W> {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl<W: SizedWidgetRef> StatefulWidget for &Popup<'_, W> {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = if let Some(next) = state.area.take() {
            // ensure that the popup remains on screen
            let width = min(next.width, area.width);
            let height = min(next.height, area.height);
            let x = next.x.clamp(buf.area.x, area.right() - width);
            let y = next.y.clamp(buf.area.y, area.bottom() - height);

            Rect::new(x, y, width, height)
        } else {
            let border_height = usize::from(self.borders.intersects(Borders::TOP))
                + usize::from(self.borders.intersects(Borders::BOTTOM));
            let border_width = usize::from(self.borders.intersects(Borders::LEFT))
                + usize::from(self.borders.intersects(Borders::RIGHT));

            let height = self
                .body
                .height()
                .saturating_add(border_height)
                .try_into()
                .unwrap_or(area.height);
            let width = self
                .body
                .width()
                .saturating_add(border_width)
                .try_into()
                .unwrap_or(area.width);
            centered_rect(width, height, area)
        };

        state.area.replace(area);

        Clear.render(area, buf);
        let block = Block::default()
            .borders(self.borders)
            .border_set(self.border_set)
            .border_style(self.border_style)
            .title(self.title.clone())
            .style(self.style);
        Widget::render(&block, area, buf);
        self.body.render_ref(block.inner(area), buf);
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
    use rstest::{fixture, rstest};

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
        )
    }

    #[fixture]
    fn buffer() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 5))
    }

    #[fixture]
    fn state() -> PopupState {
        PopupState::default()
    }

    /// Check that a popup ref can render a widget defined by a ref value (e.g. `&str`).
    #[rstest]
    fn stateful_ref_render_str(mut buffer: Buffer, mut state: PopupState) {
        let popup = Popup::new("Hello World").title("Title");
        StatefulWidget::render(&popup, buffer.area, &mut buffer, &mut state);
        let expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────┘    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// Check that a popup ref can render a widget defined by a owned value (e.g. `String`).
    #[rstest]
    fn stateful_ref_render_string(mut buffer: Buffer, mut state: PopupState) {
        let popup = Popup::new("Hello World".to_string()).title("Title");
        StatefulWidget::render(&popup, buffer.area, &mut buffer, &mut state);
        let expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────┘    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// Check that a popup can render a widget defined by a ref value (e.g. `&str`).
    #[rstest]
    fn stateful_render_str(mut buffer: Buffer, mut state: PopupState) {
        let popup = Popup::new("Hello World").title("Title");
        StatefulWidget::render(popup, buffer.area, &mut buffer, &mut state);
        let expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────┘    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// Check that a popup can render a widget defined by a owned value (e.g. `String`).
    #[rstest]
    fn stateful_render_string(mut buffer: Buffer, mut state: PopupState) {
        let popup = Popup::new("Hello World".to_string()).title("Title");
        StatefulWidget::render(popup, buffer.area, &mut buffer, &mut state);
        let expected = Buffer::with_lines([
            "                    ",
            "   ┌Title──────┐    ",
            "   │Hello World│    ",
            "   └───────────┘    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }
}
