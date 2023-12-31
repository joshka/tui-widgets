# tui-popup

[![Crates.io badge]][tui-popup crate]
[![License badge]](./LICENSE)
[![Docs.rs badge]][tui-popup docs]
[![Deps.rs badge]][Deps.rs Dependency status]
[![Discord badge]][Ratatui Discord]
<!-- [![Codecov.io badge]][Code Coverage status] -->

<!-- cargo-rdme start -->

A popup widget for [Ratatui](https://ratatui.rs)

The popup widget is a simple widget that renders a popup in the center of the screen.

## Example

```rust
use ratatui::prelude::*;
use tui_popup::Popup;

fn render_popup(frame: &mut Frame) {
    let popup = Popup::new("tui-popup demo", "Press any key to exit")
       .style(Style::new().white().on_blue());
    frame.render_widget(popup.to_widget(), frame.size());
}
```

<!-- cargo-rdme end -->

![demo](./demo.png)

## Features

- [x] automatically centers
- [x] automatically sizes to content
- [x] style popup
- [ ] configure size / position
- [ ] handle text wrapping in body -> size
- [ ] set border set / style
- [ ] mouse / keyboard events for moving
- [ ] mouse / keyboard events for close action

[Crates.io badge]: https://img.shields.io/crates/v/tui-popup?logo=rust&style=for-the-badge
[tui-popup crate]: https://crates.io/crates/tui-popup
[License badge]: https://img.shields.io/crates/l/tui-popup?style=for-the-badge
[Docs.rs badge]: https://img.shields.io/docsrs/tui-popup?logo=rust&style=for-the-badge
[Deps.rs badge]: https://deps.rs/repo/github/joshka/tui-popup/status.svg?style=for-the-badge
[Discord badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
[tui-popup docs]: https://docs.rs/crate/tui-popup/
[Deps.rs Dependency status]: https://deps.rs/repo/github/joshka/tui-popup
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
