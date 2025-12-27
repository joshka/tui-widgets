//! A collection of useful widgets for building terminal user interfaces using [Ratatui].
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//!
//! This is a crate that combines multiple previously standalone crates into one in order simplify
//! maintenance and to make it easier to use the widgets together.
//!
//! Workspace crates:
//!
//! - [tui-widgets] (this crate)
//! - [tui-bar-graph]
//! - [tui-big-text]
//! - [tui-box-text]
//! - [tui-cards]
//! - [tui-popup]
//! - [tui-prompts]
//! - [tui-qrcode]
//! - [tui-scrollbar]
//! - [tui-scrollview]
//!
//! The widget crates are also available as standalone crates.
//!
//! # Screenshots
//!
//! ## [tui-bar-graph]
//!
//! [![Braille Rainbow](https://vhs.charm.sh/vhs-1sx9Ht6NzU6e28Cl51jJVv.gif)][tui-bar-graph]
//!
//! ## [tui-big-text]
//!
//! [![Demo](https://vhs.charm.sh/vhs-7DFJFGwBEnUjjLCFSqwEm9.gif)][tui-big-text]
//!
//! ## [tui-box-text]
//!
//! [![Demo](https://vhs.charm.sh/vhs-6ldj2r9v3mIaSzk8H7Jp8t.gif)][tui-box-text]
//!
//! ## [tui-cards]
//!
//! [![Demo](https://vhs.charm.sh/vhs-34mhPM1Juk2XnnLTGpOtE9.gif)][tui-cards]
//!
//! ## [tui-popup]
//!
//! [![Demo](https://vhs.charm.sh/vhs-q5Kz0QP3zmrBlQ6dofjMh.gif)][tui-popup]
//!
//! ## [tui-prompts]
//!
//! [![Text Prompt](https://vhs.charm.sh/vhs-7gLcGtWJWDlQZqcMlhrpRG.gif)][tui-prompts]
//!
//! ## [tui-qrcode]
//!
//! [![Demo](https://vhs.charm.sh/vhs-nUpcmCP1igCcGoJ5iio07.gif)][tui-qrcode]
//!
//! ## [tui-scrollview]
//!
//! [![Demo](https://vhs.charm.sh/vhs-6PuT3pdwSTp4aTvKrCBx9F.gif)][tui-scrollview]
//!
//! [tui-widgets]: https://crates.io/crates/tui-widgets
//! [tui-bar-graph]: https://crates.io/crates/tui-bar-graph
//! [tui-big-text]: https://crates.io/crates/tui-big-text
//! [tui-box-text]: https://crates.io/crates/tui-box-text
//! [tui-cards]: https://crates.io/crates/tui-cards
//! [tui-popup]: https://crates.io/crates/tui-popup
//! [tui-prompts]: https://crates.io/crates/tui-prompts
//! [tui-qrcode]: https://crates.io/crates/tui-qrcode
//! [tui-scrollbar]: https://crates.io/crates/tui-scrollbar
//! [tui-scrollview]: https://crates.io/crates/tui-scrollview
#![doc = document_features::document_features!()]

#[cfg(feature = "bar-graph")]
#[doc(inline)]
pub use tui_bar_graph as bar_graph;
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
#[cfg(feature = "scrollbar")]
#[doc(inline)]
pub use tui_scrollbar as scrollbar;
#[cfg(feature = "scrollview")]
#[doc(inline)]
pub use tui_scrollview as scrollview;
