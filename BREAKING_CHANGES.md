# Breaking changes (rolling)

This document lists user-facing breaking changes for applications that depend on these crates.
Only changes that require app code updates are included.

## tui-popup: 0.6.2 -> 0.7.0

- Rendering `&Popup` now requires the body widget to implement `Widget` for references.
  If your body only implements `Widget` by value (like `&str`/`String`), render `Popup` by value or
  wrap the body in `Text`.

```rust
// Still works: render owned `Popup` with value bodies.
frame.render_widget(Popup::new("Hello"), area);
frame.render_stateful_widget(Popup::new("Hello"), area, &mut state);

// Still works: render by reference when the body supports `Widget` for references.
let popup = Popup::new(Text::from("Hello"));
frame.render_widget(&popup, area);
frame.render_stateful_widget(&popup, area, &mut state);
```

```rust
// No longer works: `&Popup` with `&str`/`String` bodies.
let popup = Popup::new("Hello");
frame.render_widget(&popup, area);
// Fix: wrap the body in `Text` to enable `&Popup` rendering.
let popup = Popup::new(Text::from("Hello"));
frame.render_widget(&popup, area);
```

```diff
-frame.render_stateful_widget(&popup, area, &mut state);
+frame.render_stateful_widget(popup, area, &mut state);
```

```diff
-frame.render_widget(&popup, area);
+frame.render_widget(popup, area);
```

- `WidgetRef` is no longer used by `Popup`. Use `Widget` and the normal render helpers.

```diff
-frame.render_stateful_widget_ref(&popup, area, &mut state);
+frame.render_stateful_widget(&popup, area, &mut state);
```

- `PopupState::handle_mouse_event` now takes `crossterm::event::MouseEvent`.

```diff
-use ratatui::crossterm::event::MouseEvent;
+use crossterm::event::MouseEvent;
```

```toml
[dependencies]
crossterm = "0.29"
```

## tui-prompts: 0.5.2 -> 0.6.0

- Public APIs now use `crossterm::event` types instead of `ratatui::crossterm::event`.

```diff
-use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
+use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
```

```toml
[dependencies]
crossterm = "0.29"
```
