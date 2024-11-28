use ratatui::{text::Text, widgets::WidgetRef};

/// A trait for widgets that have a fixed size.
///
/// This trait allows the popup to automatically size itself based on the size of the body widget.
/// Implementing this trait for a widget allows it to be used as the body of a popup. You can also
/// wrap existing widgets in a newtype and implement this trait for the newtype to use them as the
/// body of a popup.
pub trait SizedWidgetRef: WidgetRef {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
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

impl SizedWidgetRef for String {
    fn width(&self) -> usize {
        Text::from(self.as_str()).width()
    }

    fn height(&self) -> usize {
        Text::from(self.as_str()).height()
    }
}
