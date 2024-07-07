use std::fmt::Debug;

use derive_setters::Setters;
use ratatui::{
    prelude::*,
    widgets::{Borders, WidgetRef},
};

/// Configuration for a popup.
///
/// This struct is used to configure a [`Popup`]. It can be created using
/// [`Popup::new`](Popup::new).
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
///     frame.render_widget(&popup, frame.size());
/// }
/// ```
#[derive(Debug, Setters)]
#[setters(into)]
#[non_exhaustive]
pub struct Popup<'content, W: SizedWidgetRef> {
    /// The body of the popup.
    pub body: W,
    /// The title of the popup.
    pub title: Line<'content>,
    /// The style to apply to the entire popup.
    pub style: Style,
    /// The borders of the popup.
    pub borders: Borders,
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
    pub fn new<L>(title: L, body: W) -> Self
    where
        L: Into<Line<'content>>,
    {
        Self {
            body,
            title: title.into(),
            style: Style::default(),
            borders: Borders::ALL,
        }
    }
}

impl SizedWidgetRef for Text<'_> {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
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
