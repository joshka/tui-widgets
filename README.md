<!-- cargo-rdme start -->

# Tui-scrollview

[![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
[![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
Discord]

`tui-scrollview` is a library for creating scrollable views in [Ratatui].

## Installation

```shell
cargo add tui-scrollview
```

## Usage

```rust
use std::iter;
use tui_scrollview::{ScrollView, ScrollViewState};
use ratatui::{layout::Size, prelude::*, widgets::*};

fn render(frame: &mut Frame) {
    let size = Size::new(10, 100);
    let mut scroll_view = ScrollView::new(size);
    let some_long_string =
        iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
           .take(100)
           .collect::<String>();
    let area = Rect::new(0, 0, 10, 100);
    scroll_view.render_widget(Paragraph::new(some_long_string), area);
    let mut state = ScrollViewState::default();
    frame.render_stateful_widget(scroll_view, area, &mut state);
}
```

## Example

[scrollview.rs](https://github.com/joshka/tui-scrollview/tree/main/examples/scrollview.rs)

![Demo](https://vhs.charm.sh/vhs-hkWFxmfWuJInxosjYrSg9.gif)

[Crates.io Badge]: https://img.shields.io/crates/v/tui-scrollview?logo=rust&style=for-the-badge
[License Badge]: https://img.shields.io/crates/l/tui-scrollview?style=for-the-badge
[Docs.rs Badge]: https://img.shields.io/docsrs/tui-scrollview?logo=rust&style=for-the-badge
[Deps.rs Badge]:
    https://deps.rs/repo/github/joshka/tui-scrollview/status.svg?style=for-the-badge
[Codecov.io Badge]:
    https://img.shields.io/codecov/c/github/joshka/tui-scrollview?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
[Discord Badge]:
    https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

[Crate]: https://crates.io/crates/tui-scrollview
[API Docs]: https://docs.rs/crate/tui-scrollview/
[Dependencies]: https://deps.rs/repo/github/joshka/tui-scrollview
[Coverage]: https://app.codecov.io/gh/joshka/tui-scrollview
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj

[Ratatui]: https://crates.io/crates/ratatui

<!-- cargo-rdme end -->

## TODO

- [x] Clamp scroll state to buffer area
- [x] Add scrollbar
- [x] Add example
- [ ] Conditionally show scrollbar
- [x] Add demo GIF
- [x] Fix scrollbar size (~~Ratatui bug~~ [intended behavior](https://discord.com/channels/1070692720437383208/1072880020713898004/1197509375183564841))
- [ ] Change offset to use `Position` once [Ratatui PR #790] is released (this will be breaking
  change for the ScrollViewState)

## License

Copyright (c) 2024 Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

[Ratatui PR #790]: https://github.com/ratatui-org/ratatui/pull/790
