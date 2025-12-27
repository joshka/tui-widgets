/// Bundle content and viewport lengths to avoid ambiguous arguments.
///
/// This struct is a convenience for readability. Use a struct literal so each field is named at
/// the call site:
///
/// ```rust
/// use tui_scrollbar::ScrollLengths;
///
/// let lengths = ScrollLengths {
///     content_len: 200,
///     viewport_len: 20,
/// };
/// ```
///
/// Zero values are accepted, and consumers like [`ScrollBar`] and [`ScrollMetrics`] will treat
/// them as 1.
///
/// [`ScrollBar`]: crate::ScrollBar
/// [`ScrollMetrics`]: crate::ScrollMetrics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollLengths {
    /// Total scrollable content length in logical units.
    pub content_len: usize,
    /// Visible viewport length in logical units.
    pub viewport_len: usize,
}
