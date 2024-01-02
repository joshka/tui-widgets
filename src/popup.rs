use derive_setters::Setters;
use ratatui::prelude::*;

use crate::PopupWidget;

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
