# tui-popup

<!-- cargo-rdme start -->

A popup widget for tui-rs

## Example

```rust
use ratatui::prelude::*;
use tui_popup::Popup;

fn render(frame: &mut Frame) {
    let popup = Popup::new("tui-popup demo", "Press any key to exit");
    frame.render_widget(popup.to_widget(), frame.size());
}
```

<!-- cargo-rdme end -->

![demo](./demo.png)

## Features

- [x] automatically centers
- [x] automatically sizes to content
- [ ] style text
- [ ] configure size / position
- [ ] handle text wrapping in body -> size
- [ ] set border set / style
- [ ] mouse / keyboard events for moving
- [ ] mouse / keyboard events for close action
