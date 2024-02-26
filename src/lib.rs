//! [tui-big-text] is a rust crate that renders large pixel text as a [Ratatui] widget using the
//! glyphs from the [font8x8] crate.
//!
//! ![Demo](https://vhs.charm.sh/vhs-35FZxQa32pCZdRW7pmpqf6.gif)
//!
//! [![Crate badge]][tui-big-text]
//! [![Docs.rs Badge]][API Docs]
//! [![Deps.rs Badge]][Dependency Status]<br>
//! [![License Badge]](./LICENSE-MIT)
//! [![Codecov.io Badge]][Code Coverage]
//! [![Discord Badge]][Ratatui Discord]
//!
//! [GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui tui-big-text
//! ```
//!
//! # Usage
//!
//! Create a [`BigText`] widget using [`BigText::builder`] and pass it to [`render_widget`] to
//! render be rendered. The builder allows you to customize the [`Style`] of the widget and the
//! [`PixelSize`] of the glyphs.
//!
//! # Examples
//!
//! ```rust
//! use anyhow::Result;
//! use ratatui::prelude::*;
//! use tui_big_text::{BigText, PixelSize};
//!
//! fn render(frame: &mut Frame) -> Result<()> {
//!     let big_text = BigText::builder()
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
//! The [`PixelSize`] can be used to control how many character cells are used to represent a single
//! pixel of the 8x8 font. It has six variants:
//!
//! - `Full` (default) - Each pixel is represented by a single character cell.
//! - `HalfHeight` - Each pixel is represented by half the height of a character cell.
//! - `HalfWidth` - Each pixel is represented by half the width of a character cell.
//! - `Quadrant` - Each pixel is represented by a quarter of a character cell.
//! - `ThirdHeight` - Each pixel is represented by a third of the height of a character cell.
//! - `Sextant` - Each pixel is represented by a sixth of a character cell.
//!
//! <details><summary>Pixel Size Example</summary>
//!
//! ![Pixel Size](https://vhs.charm.sh/vhs-2nLycKO16vHzqg3TxDNvq4.gif)
//!
//! </details>
//!
//! [tui-big-text]: https://crates.io/crates/tui-big-text
//! [Ratatui]: https://crates.io/crates/ratatui
//! [font8x8]: https://crates.io/crates/font8x8
//!
//! <!-- Note that these links are sensitive to breaking with cargo-rdme -->
//! [`BigText`]: crate::big_text::BigText
//! [`BigText::builder`]: crate::big_text::BigText#method.builder
//! [`PixelSize`]: crate::pixel_size::PixelSize
//! [`render_widget`]: https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.render_widget
//! [`Style`]: https://docs.rs/ratatui/latest/ratatui/style/struct.Style.html
//!
//! [Crate badge]: https://img.shields.io/crates/v/tui-big-text?logo=rust&style=for-the-badge
//! [Docs.rs Badge]: https://img.shields.io/docsrs/tui-big-text?logo=rust&style=for-the-badge
//! [Deps.rs Badge]: https://deps.rs/repo/github/joshka/tui-big-text/status.svg?style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/tui-big-text?style=for-the-badge
//! [Codecov.io Badge]: https://img.shields.io/codecov/c/github/joshka/tui-big-text?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//!
//! [API Docs]: https://docs.rs/crate/tui-big-text/
//! [Dependency Status]: https://deps.rs/repo/github/joshka/tui-big-text
//! [Code Coverage]: https://app.codecov.io/gh/joshka/tui-big-text
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//!
//! [GitHub Repository]: https://github.com/joshka/tui-big-text
//! [Examples]: https://github.com/joshka/tui-big-text/tree/main/examples
//! [Changelog]: https://github.com/joshka/tui-big-text/blob/main/CHANGELOG.md
//! [Contributing]: https://github.com/joshka/tui-big-text/blob/main/CONTRIBUTING.md

mod big_text;
mod pixel_size;

pub use big_text::{BigText, BigTextBuilder};
pub use pixel_size::PixelSize;
