# Tui-cards

<!-- cargo-rdme start -->

A simple library to render playing cards in a terminal using tui-rs.

## Example

```rust
use tui_cards::{Card, Rank, Suit};

let card = Card::new(Rank::Ace, Suit::Spades);
frame.render_widget(&card, area);
```

## Demo

```shell
cargo run --example card
```

![demo](https://vhs.charm.sh/vhs-34mhPM1Juk2XnnLTGpOtE9.gif)

<!-- cargo-rdme end -->

## TODO

- [ ] work out bug with background of cell after the suit character
- [ ] more configuration options

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
