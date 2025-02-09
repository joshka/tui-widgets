//! A collection of useful widgets for building terminal user interfaces using [Ratatui].
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//!
//! This is a crate that combines multiple previously standalone crates into one in order simplify
//! maintenance and to make it easier to use the widgets together.
//!
//! Includes the following widgets, which are each also available as standalone crates:
//!
//! - [tui-big-text](https://crates.io/crates/tui-big-text)
//! - [tui-box-text](https://crates.io/crates/tui-box-text)
//! - [tui-cards](https://crates.io/crates/tui-cards)
//! - [tui-popup](https://crates.io/crates/tui-popup)
//! - [tui-prompts](https://crates.io/crates/tui-prompts)
//! - [tui-qrcode](https://crates.io/crates/tui-qrcode)
//! - [tui-scrollview](https://crates.io/crates/tui-scrollview)
#![doc = document_features::document_features!()]

#[cfg(feature = "big-text")]
#[doc(inline)]
pub use tui_big_text as big_text;

#[cfg(feature = "box-text")]
#[doc(inline)]
pub use tui_box_text as box_text;

#[cfg(feature = "cards")]
#[doc(inline)]
pub use tui_cards as cards;

#[cfg(feature = "popup")]
#[doc(inline)]
pub use tui_popup as popup;

#[cfg(feature = "prompts")]
#[doc(inline)]
pub use tui_prompts as prompts;

#[cfg(feature = "qrcode")]
#[doc(inline)]
pub use tui_qrcode as qrcode;

#[cfg(feature = "scrollview")]
#[doc(inline)]
pub use tui_scrollview as scrollview;
