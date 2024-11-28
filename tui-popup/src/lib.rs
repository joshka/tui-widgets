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
//!     let popup = Popup::new("Press any key to exit")
//!         .title("tui-popup demo")
//!        .style(Style::new().white().on_blue());
//!     frame.render_widget(&popup, frame.size());
//! }
//! ```
//!
//! ![demo](https://vhs.charm.sh/vhs-q5Kz0QP3zmrBlQ6dofjMh.gif)
//!
//! # Feature flags
#![doc = document_features::document_features!()]

mod popup;
mod popup_state;
mod sized_widget;
mod sized_wrapper;

pub use crate::{
    popup::Popup,
    popup_state::{DragState, PopupState},
    sized_widget::SizedWidgetRef,
    sized_wrapper::SizedWrapper,
};
