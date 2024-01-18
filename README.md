<!-- cargo-rdme start -->

# Tui-scrollview

[![License Badge]](#license) [![Docs.rs Badge]][API Docs] [![Deps.rs Badge]][Dependencies]
[![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui Discord]

`tui-scrollview` is a library for creating scrollable views in [Ratatui].

## Installation

```shell
cargo add tui-scrollview
```

## Usage

```rust
use tui_scrollview::ScrollView;
use ratatui::{prelude::*, layout::Size};

fn render(Frame: &mut Frame) {
    let size = Size::new(10, 100);
    let mut scroll_view = ScrollView::new(size);
    let some_long_string =
        iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
           .take(100)
           .collect::<String>();
    let area = size.into();
    scroll_view.render_widget(Text::from(some_long_string), area);
    let state = ScrollViewState::default();
    frame.render_stateful_widget(scroll_view, area);
}
```

[License Badge]: https://img.shields.io/crates/l/tui-scrollview?style=for-the-badge
[Docs.rs Badge]: https://img.shields.io/docsrs/tui-scrollview?logo=rust&style=for-the-badge
[Deps.rs Badge]: https://deps.rs/repo/github/joshka/tui-scrollview/status.svg?style=for-the-badge
[Codecov.io Badge]: https://img.shields.io/codecov/c/github/joshka/tui-scrollview?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

[API Docs]: https://docs.rs/crate/tui-scrollview/
[Dependencies]: https://deps.rs/repo/github/joshka/tui-scrollview
[Coverage]: https://app.codecov.io/gh/joshka/tui-scrollview
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj

<!-- cargo-rdme end -->

## License

Copyright (c) 2023 Josh McKinney

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
