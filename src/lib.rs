//! A popup widget for [Ratatui](https://ratatui.rs)
//!
//! The popup widget is a simple widget that renders a popup in the center of the screen.
//!
//! # Example
//!
//! ```rust
//! use ratatui::prelude::*;
//! use tui_popup::Popup;
//!
//! fn render_popup(frame: &mut Frame) {
//!     let popup = Popup::new("tui-popup demo", "Press any key to exit")
//!        .style(Style::new().white().on_blue());
//!     frame.render_widget(popup.to_widget(), frame.size());
//! }
//! ```

use std::cmp::min;

use derive_setters::Setters;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

/// Configuration for a popup.
///
/// This struct is used to configure a [`PopupWidget`]. It can be created using
/// [`Popup::new`](Popup::new). Convert it into a Ratatui widget using
/// [`Popup::to_widget`](Popup::to_widget).
///
/// # Example
///
/// ```rust
/// use ratatui::prelude::*;
/// use tui_popup::Popup;
///
/// fn render_popup(frame: &mut Frame) {
///     let popup = Popup::new("tui-popup demo", "Press any key to exit")
///         .style(Style::new().white().on_blue());
///     frame.render_widget(popup.to_widget(), frame.size());
/// }
/// ```
#[derive(Clone, Debug, Default, Setters)]
#[setters(into)]
#[non_exhaustive]
pub struct Popup<'content> {
    /// The title of the popup.
    pub title: Line<'content>,
    /// The body of the popup.
    pub body: Text<'content>,
    /// The style to apply to the entire popup.
    pub style: Style,
}

/// A Ratatui widget that renders a popup.
#[derive(Clone, Debug)]
pub struct PopupWidget<'content> {
    popup: &'content Popup<'content>,
}

impl<'content> Popup<'content> {
    /// Create a new popup with the given title and body.
    ///
    /// # Parameters
    ///
    /// - `title` - The title of the popup. This can be any type that can be converted into a
    ///   [`Line`].
    /// - `body` - The body of the popup. This can be any type that can be converted into a
    ///   [`Text`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_popup::Popup;
    ///
    /// let popup = Popup::new("tui-popup demo", "Press any key to exit");
    /// ```
    pub fn new<L, T>(title: L, body: T) -> Self
    where
        L: Into<Line<'content>>,
        T: Into<Text<'content>>,
    {
        Self {
            title: title.into(),
            body: body.into(),
            ..Default::default()
        }
    }

    /// Convert the popup into a Ratatui widget.
    #[must_use]
    pub const fn to_widget(&'content self) -> PopupWidget<'content> {
        PopupWidget { popup: self }
    }
}

impl Widget for PopupWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = self.popup;
        let height = u16::try_from(popup.body.height())
            .unwrap_or(area.height)
            .saturating_add(2);
        let width = u16::try_from(popup.body.width())
            .unwrap_or(area.width)
            .saturating_add(2);
        let area = centered_rect(width, height, area);
        Clear.render(area, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.popup.title.clone());
        Paragraph::new(self.popup.body.clone())
            .block(block)
            .style(popup.style)
            .render(area, buf);
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
