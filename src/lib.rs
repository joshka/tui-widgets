#![allow(clippy::module_name_repetitions)]
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

mod popup;
mod state;
mod widget;

pub use popup::Popup;
pub use state::PopupState;
pub use widget::PopupWidget;
