//! # Tui-scrollview
//!
//! [![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
//! [![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
//! Discord]
//!
//! `tui-scrollview` is a library for creating scrollable views in [Ratatui].
//!
//! ## Installation
//!
//! ```shell
//! cargo add tui-scrollview
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use std::iter;
//! use tui_scrollview::{ScrollView, ScrollViewState};
//! use ratatui::{layout::Size, prelude::*, widgets::*};
//!
//! fn render(frame: &mut Frame) {
//!     let size = Size::new(10, 100);
//!     let mut scroll_view = ScrollView::new(size);
//!     let some_long_string =
//!         iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
//!            .take(100)
//!            .collect::<String>();
//!     let area = Rect::new(0, 0, 10, 100);
//!     scroll_view.render_widget(Paragraph::new(some_long_string), area);
//!     let mut state = ScrollViewState::default();
//!     frame.render_stateful_widget(scroll_view, area, &mut state);
//! }
//! ```
//!
//! ## Example
//!
//! This example shows a scrollable view with two paragraphs of text, one for the line numbers and
//! one for the text. On top of this a Gauge widget is rendered to show that this can be used in
//! combination with any other widget.
//!
//! ![Demo](https://vhs.charm.sh/vhs-6PuT3pdwSTp4aTvKrCBx9F.gif)
//!
//! [scrollview.rs](https://github.com/joshka/tui-scrollview/tree/main/examples/scrollview.rs)
//!
//! [Crates.io Badge]: https://img.shields.io/crates/v/tui-scrollview?logo=rust&style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/tui-scrollview?style=for-the-badge
//! [Docs.rs Badge]: https://img.shields.io/docsrs/tui-scrollview?logo=rust&style=for-the-badge
//! [Deps.rs Badge]:
//!     https://deps.rs/repo/github/joshka/tui-scrollview/status.svg?style=for-the-badge
//! [Codecov.io Badge]:
//!     https://img.shields.io/codecov/c/github/joshka/tui-scrollview?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//!
//! [Crate]: https://crates.io/crates/tui-scrollview
//! [API Docs]: https://docs.rs/crate/tui-scrollview/
//! [Dependencies]: https://deps.rs/repo/github/joshka/tui-scrollview
//! [Coverage]: https://app.codecov.io/gh/joshka/tui-scrollview
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//!
//! [Ratatui]: https://crates.io/crates/ratatui

mod scroll_view;
mod state;

pub use scroll_view::ScrollView;
pub use state::ScrollViewState;
