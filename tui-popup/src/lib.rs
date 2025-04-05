//! A popup widget for [Ratatui](https://ratatui.rs)
//!
//! The popup widget is a simple widget that renders a popup in the center of the screen.
//!
//! # Example
//!
//! ```rust
//! use ratatui::{
//!     style::{Style, Stylize},
//!     Frame,
//! };
//! use tui_popup::Popup;
//!
//! fn render_popup(frame: &mut Frame) {
//!     let popup = Popup::new("Press any key to exit")
//!         .title("tui-popup demo")
//!         .style(Style::new().white().on_blue());
//!     frame.render_widget(&popup, frame.area());
//! }
//! ```
//!
//! ![demo](https://vhs.charm.sh/vhs-q5Kz0QP3zmrBlQ6dofjMh.gif)
//!
//! # Feature flags
#![doc = document_features::document_features!()]

mod known_size;
mod known_size_wrapper;
mod popup;
mod popup_state;

pub use crate::{
    known_size::KnownSize,
    known_size_wrapper::KnownSizeWrapper,
    popup::Popup,
    popup_state::{InteractionState, PopupState},
};
