<h1>
<img src="https://user-images.githubusercontent.com/381361/252977280-49b9ff66-f78d-4e16-b5ed-29d771bfcab2.png"
    alt="logo">
</h1>

# tui-prompts

[![Crates.io](https://img.shields.io/crates/v/tui-prompts?logo=rust&style=for-the-badge)](https://crates.io/crates/tui-prompts)
[![License](https://img.shields.io/crates/l/tui-prompts?style=for-the-badge)](./LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/tui-prompts?logo=rust&style=for-the-badge)](https://docs.rs/crate/tui-prompts/)
[![Dependency Status](https://deps.rs/repo/github/joshka/tui-prompts/status.svg?style=for-the-badge)](https://deps.rs/repo/github/joshka/tui-prompts)
[![Codecov](https://img.shields.io/codecov/c/github/joshka/tui-prompts?logo=codecov&style=for-the-badge&token=BAQ8SOKEST)](https://app.codecov.io/gh/joshka/tui-prompts)
[![Discord](https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge)](https://discord.gg/pMCEU9hNEj)

`tui-prompts` is a Rust crate that provides prompt widgets for the Ratatui crate. It allows for easy
creation of interactive command-line interfaces with various types of prompts. Inspired by
<https://www.npmjs.com/package/prompts> and various other prompt libraries.

## Examples

### Text Prompt

<details>
<summary>Code</summary>

```rust
struct App<'a> {
    username_state: TextState<'a>,
    password_state: TextState<'a>,
    invisible_state: TextState<'a>,
}

impl<'a> App<'a> {
    fn draw_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let (username_area, password_area, invisible_area) = split_layout(frame.size())

        TextPrompt::from("Username")
            .draw(frame, username_area, &mut self.username_state);

        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, password_area, &mut self.password_state);

        TextPrompt::from("Invisible")
            .with_render_style(TextRenderStyle::Invisible)
            .draw(frame, invisible_area, &mut self.invisible_state);
    }
}
```

</details>

![Text Prompt](https://vhs.charm.sh/vhs-7gLcGtWJWDlQZqcMlhrpRG.gif)

See the [text example](./examples/text.rs) for more details.

### Soft Wrapping

Text is automatically character wrapped to fit in the render area.

![Multi-line](https://vhs.charm.sh/vhs-5zzgSyRXy6IjBahoe1esDi.gif)

See the [multi line example](./examples/multi_line.rs) for more details.

## Features

- [x] Text prompt
- [x] Password prompt
- [x] Invisible prompt
- [x] Readline / emacs style Key Bindings
- [x] Crossterm backend
- [x] Soft wrapping single lines
- [ ] Multi-line input
- [ ] Scrolling
- [ ] More prompt types:
  - [ ] Number
  - [ ] Confirm
  - [ ] List
  - [ ] Toggle
  - [ ] Select
  - [ ] Multi-select
  - [ ] Autocomplete
  - [ ] Autocomplete multi-select
  - [ ] Date
- [ ] Bracketed paste
- [ ] Validation
- [ ] Default initial value
- [ ] Custom style
- [ ] Themes
- [ ] Custom formatting
- [ ] Backend agnostic keyboard event handling ([Termion](https://crates.io/crates/termion) and
[Termwiz](https://crates.io/crates/termwiz))
- [ ] Customizable key bindings
- [ ] Handle more advanced multi-key bindings e.g. `^[b` and `^[f` for start / end of line
- [ ] Prompt chaining

## Installation

```shell
cargo add ratatui tui-prompts crossterm
```

Or add the following to your `Cargo.toml` file:

## Key Bindings

| Key | Action
| --- | ---
| Home, Ctrl+A | Move cursor to beginning of line
| End, Ctrl+E | Move cursor to end of line
| Left, Ctrl+B | Move cursor one character left
| Right, Ctrl+F | Move cursor one character right
| Backspace (Delete on Mac), Ctrl+H | Delete character before cursor
| Delete (Fn+Delete on Mac), Ctrl+D | Delete character at cursor
| Ctrl+K | Delete all characters from the cursor to the end of line
| Ctrl+U | Delete the entire line
| Enter | Complete the prompt
| Escape, Ctrl+C | Abort the prompt

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
