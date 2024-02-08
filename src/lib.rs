//! [tui-big-text] is a rust crate that renders large pixel text as a [Ratatui] widget using the
//! glyphs from the [font8x8] crate.
//!
//! ![Hello World example](https://vhs.charm.sh/vhs-2UxNc2SJgiNqHoowbsXAMW.gif)
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui tui-big-text
//! ```
//!
//! # Usage
//!
//! Create a [`BigText`] widget using `BigTextBuilder` and pass it to [`Frame::render_widget`] to
//! render be rendered. The builder allows you to customize the [`Style`] of the widget and the
//! [`PixelSize`] of the glyphs. The [`PixelSize`] can be used to control how many character cells
//! are used to represent a single pixel of the 8x8 font.
//!
//! # Example
//!
//! ```rust
//! use anyhow::Result;
//! use ratatui::prelude::*;
//! use tui_big_text::{BigTextBuilder, PixelSize};
//!
//! fn render(frame: &mut Frame) -> Result<()> {
//!     let big_text = BigTextBuilder::default()
//!         .pixel_size(PixelSize::Full)
//!         .style(Style::new().blue())
//!         .lines(vec![
//!             "Hello".red().into(),
//!             "World".white().into(),
//!             "~~~~~".into(),
//!         ])
//!         .build()?;
//!     frame.render_widget(big_text, frame.size());
//!     Ok(())
//! }
//! ```
//!
//! [tui-big-text]: https://crates.io/crates/tui-big-text
//! [Ratatui]: https://crates.io/crates/ratatui
//! [font8x8]: https://crates.io/crates/font8x8
//! [`BigText`]: crate::BigText
//! [`PixelSize`]: crate::PixelSize
//! [`Frame::render_widget`]: ratatui::Frame::render_widget
//! [`Style`]: ratatui::style::Style

mod big_text;
mod pixel_size;

pub use big_text::{BigText, BigTextBuilder};
pub use pixel_size::PixelSize;
