//! A collection of useful widgets for building terminal user interfaces using [Ratatui].
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//!
//! This is a crate that combines multiple previously standalone crates into one in order simplify
//! maintenance and to make it easier to use the widgets together.
//!
#![doc = document_features::document_features!()]

#[doc(inline)]
pub use tui_big_text as big_text;

#[doc(inline)]
pub use tui_popup as popup;

#[doc(inline)]
pub use tui_prompts as prompts;

#[doc(inline)]
pub use tui_scrollview as scrollview;
