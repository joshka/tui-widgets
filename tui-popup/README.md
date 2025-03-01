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
    frame.render_widget(&popup, frame.size());
}
```

![demo](https://vhs.charm.sh/vhs-q5Kz0QP3zmrBlQ6dofjMh.gif)

<!-- cargo-rdme end -->

## State

The widget supports storing the position of the popup in PopupState. This is experimental and the
exact api for this will likely change.

```rust
use ratatui::prelude::*;
use tui_popup::Popup;

fn render_stateful_popup(frame: &mut Frame, popup_state: &mut PopupState) {
    let popup = Popup::new("tui-popup demo", "Press any key to exit")
       .style(Style::new().white().on_blue());
    frame.render_stateful_widget_ref(popup, frame.size(), popup_state);
}

fn handle_key(event: KeyEvent, &mut state) {
    match event.code {
        KeyCode::Up => state.move_up(1),
        KeyCode::Down => state.move_down(1),
        KeyCode::Left => state.move_left(1),
        KeyCode::Right => state.move_right(1),
    }
}
```

The popup can automatically handle being moved around by the mouse, by passing in the column and
row of Mouse Up / Down / Drag events. The current implemntation of this checks whether the click is
in the first row of the popup, otherwise ignores the event.

```rust
match event.read()? {
    Event::Mouse(event) => {
        match event.kind {
            event::MouseEventKind::Down(MouseButton::Left) => {
                popup_state.mouse_down(event.column, event.row)
            }
            event::MouseEventKind::Up(MouseButton::Left) => {
                popup_state.mouse_up(event.column, event.row);
            }
            event::MouseEventKind::Drag(MouseButton::Left) => {
                popup_state.mouse_drag(event.column, event.row);
            }
            _ => {}
        };
    }
    // -- snip --
}
```

![state demo](https://vhs.charm.sh/vhs-73faTQbCkAHOv7dt0MQJAd.gif)

The popup also supports rendering arbitrary widgets by implementing SizedWidgetRef (or wrapping them
with the provided SizedWrapper). This makes it possible to support wrapping and scrolling in using a
`Paragraph` widget, or scrolling any amount of widgets using
[tui-scrollview](https://crates.io/crates/tui-scrollview).

```rust
let lines: Text = (0..10).map(|i| Span::raw(format!("Line {}", i))).collect();
let paragraph = Paragraph::new(lines).scroll((scroll, 0));
let sized_paragraph = SizedWrapper {
    inner: paragraph,
    width: 21,
    height: 5,
};
let popup = Popup::new("scroll: ↑/↓ quit: Esc", sized_paragraph)
    .style(Style::new().white().on_blue());
frame.render_widget_ref(popup, area);
```

![paragraph example](https://vhs.charm.sh/vhs-A3mwcn9IngIc0hpl2AsXM.gif)

## Features

- [x] automatically centers
- [x] automatically sizes to content
- [x] style popup
- [x] move the popup (using state)
- [x] handle mouse events for dragging
- [x] move to position
- [x] resize
- [ ] set border set / style
- [ ] add close button
- [ ] add nicer styling of header etc.
- [ ] configure text wrapping in body to conform to a specific size

## License

Copyright (c) Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: /LICENSE-APACHE
[LICENSE-MIT]: /LICENSE-MIT

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](/CONTRIBUTING.md).

[Crates.io badge]: https://img.shields.io/crates/v/tui-popup?logo=rust&style=for-the-badge
[tui-popup crate]: https://crates.io/crates/tui-popup
[License badge]: https://img.shields.io/crates/l/tui-popup?style=for-the-badge
[Docs.rs badge]: https://img.shields.io/docsrs/tui-popup?logo=rust&style=for-the-badge
[Deps.rs badge]: https://deps.rs/repo/github/joshka/tui-popup/status.svg?style=for-the-badge
[Discord badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
[tui-popup docs]: https://docs.rs/crate/tui-popup/
[Deps.rs Dependency status]: https://deps.rs/repo/github/joshka/tui-popup
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
