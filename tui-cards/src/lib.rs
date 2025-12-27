//! A simple library to render playing cards in a terminal using tui-rs.
//!
//! # Example
//!
//! ```no_run
//! use tui_cards::{Card, Rank, Suit};
//!
//! # fn draw(frame: &mut ratatui::Frame) {
//! let card = Card::new(Rank::Ace, Suit::Spades);
//! frame.render_widget(&card, frame.area());
//! # }
//! ```
//!
//! # Demo
//!
//! ```shell
//! cargo run --example card
//! ```
//!
//! ![demo](https://vhs.charm.sh/vhs-34mhPM1Juk2XnnLTGpOtE9.gif)
use std::iter::zip;

use indoc::indoc;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Stylize};
use ratatui_core::widgets::Widget;
use strum::{Display, EnumIter};

/// A playing card.
///
/// # Example
///
/// ```rust
/// use tui_cards::{Card, Rank, Suit};
/// # fn draw(frame: &mut ratatui::Frame) {
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// frame.render_widget(&card, frame.area());
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn as_colored_symbol(&self) -> String {
        format!(
            "{}{}",
            self.rank.as_symbol(),
            self.suit.as_four_color_symbol()
        )
    }
}

impl Rank {
    pub const fn as_symbol(self) -> char {
        match self {
            Self::Ace => 'A',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Jack => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
}

impl Suit {
    pub const fn color(self) -> Color {
        match self {
            Self::Clubs => Color::Green,
            Self::Diamonds => Color::Blue,
            Self::Hearts => Color::Red,
            Self::Spades => Color::Black,
        }
    }

    pub const fn as_symbol(self) -> char {
        match self {
            Self::Clubs => '♣',
            Self::Diamonds => '♦',
            Self::Hearts => '♥',
            Self::Spades => '♠',
        }
    }

    pub const fn as_colored_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2663}\u{FE0F}",
            Self::Diamonds => "\u{2666}\u{FE0F}",
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }

    pub const fn as_four_color_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2618}\u{FE0F}",     // shamrock
            Self::Diamonds => "\u{1F537}\u{FE0F}", // blue diamond
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }
}

impl Rank {
    pub const fn template(self) -> &'static str {
        match self {
            Self::Ace => indoc! {"
                ╭────────────╮
                │ A          │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │          A │
                ╰────────────╯"},
            Self::Two => indoc! {"
                ╭────────────╮
                │ 2   xx     │
                │            │
                │            │
                │            │
                │            │
                │            │
                │     xx   2 │
                ╰────────────╯"},
            Self::Three => indoc! {"
                ╭────────────╮
                │ 3   xx     │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │     xx   3 │
                ╰────────────╯"},
            Self::Four => indoc! {"
                ╭────────────╮
                │ 4xx    xx  │
                │            │
                │            │
                │            │
                │            │
                │            │
                │  xx    xx4 │
                ╰────────────╯"},
            Self::Five => indoc! {"
                ╭────────────╮
                │ 5xx    xx  │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │  xx    xx5 │
                ╰────────────╯"},
            Self::Six => indoc! {"
                ╭────────────╮
                │ 6xx    xx  │
                │            │
                │            │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx6 │
                ╰────────────╯"},
            Self::Seven => indoc! {"
                ╭────────────╮
                │ 7xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx7 │
                ╰────────────╯"},
            Self::Eight => indoc! {"
                ╭────────────╮
                │ 8xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │     xx     │
                │            │
                │  xx    xx8 │
                ╰────────────╯"},
            Self::Nine => indoc! {"
                ╭────────────╮
                │ 9xx    xx  │
                │            │
                │  xx    xx  │
                │     xx     │
                │  xx    xx  │
                │            │
                │  xx    xx9 │
                ╰────────────╯
                "},
            Self::Ten => indoc! {"
                ╭────────────╮
                │10xx    xx  │
                │     xx     │
                │  xx    xx  │
                │            │
                │  xx    xx  │
                │     xx     │
                │  xx    xx10│
                ╰────────────╯"},
            Self::Jack => indoc! {"
                ╭────────────╮
                │ Jxx        │
                │       JJ   │
                │       JJ   │
                │       JJ   │
                │  JJ   JJ   │
                │   JJJJJ    │
                │        xxJ │
                ╰────────────╯"},
            Self::Queen => indoc! {"
                ╭────────────╮
                │ Qxx        │
                │   QQQQQ    │
                │  QQ   QQ   │
                │  QQ   QQ   │
                │  QQ   QQ   │
                │   QQQQ  Q  │
                │        xxQ │
                ╰────────────╯
            "},
            Self::King => indoc! {"
                ╭────────────╮
                │ Kxx        │
                │  KK    KK  │
                │  KK   KK   │
                │  KK KK     │
                │  KK   KK   │
                │  KK    KK  │
                │        xxK │
                ╰────────────╯"},
        }
    }
}

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let template = self.rank.template();
        let symbol = self.suit.as_four_color_symbol();
        let card = template.replace("xx", symbol);
        let color = self.suit.color();
        for (line, row) in zip(card.lines(), area.rows()) {
            let span = line.fg(color).bg(Color::White);
            span.render(row, buf);
        }
    }
}
