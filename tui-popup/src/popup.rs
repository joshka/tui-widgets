use std::fmt::Debug;

use crate::PopupState;
use derive_setters::Setters;
use ratatui::{
    prelude::{Buffer, Line, Rect, Style, Text},
    symbols::border::Set,
    widgets::{Block, Borders, Clear, StatefulWidgetRef, Widget, WidgetRef},
};
use std::cmp::min;

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

/// A trait for widgets that have a fixed size.
///
/// This trait allows the popup to automatically size itself based on the size of the body widget.
/// Implementing this trait for a widget allows it to be used as the body of a popup. You can also
/// wrap existing widgets in a newtype and implement this trait for the newtype to use them as the
/// body of a popup.
pub trait SizedWidgetRef: WidgetRef + Debug {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
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

impl SizedWidgetRef for &Text<'_> {
    fn width(&self) -> usize {
        Text::width(self)
    }

    fn height(&self) -> usize {
        Text::height(self)
    }
}

impl SizedWidgetRef for &str {
    fn width(&self) -> usize {
        Text::from(*self).width()
    }

    fn height(&self) -> usize {
        Text::from(*self).height()
    }
}

#[derive(Debug)]
pub struct SizedWrapper<W: Debug> {
    pub inner: W,
    pub width: usize,
    pub height: usize,
}

impl<W: WidgetRef + Debug> WidgetRef for SizedWrapper<W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.inner.render_ref(area, buf);
    }
}

impl<W: WidgetRef + Debug> SizedWidgetRef for SizedWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W: SizedWidgetRef> WidgetRef for Popup<'_, W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidgetRef::render_ref(self, area, buf, &mut state);
    }
}

impl<W: SizedWidgetRef> StatefulWidgetRef for Popup<'_, W> {
    type State = PopupState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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
