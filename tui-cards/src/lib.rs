//! A simple library to render playing cards in a terminal using tui-rs.
//!
//! # Example
//!
//! ```no_run
//! use tui_cards::{Card, Rank, Suit};
//!
//! # fn draw(frame: &mut ratatui::Frame) {
//! let card = Card::new(Rank::Ace, Suit::Spades);
//! frame.render_widget(&card, area);
//! # }
//! ```
use std::iter::zip;

use indoc::indoc;
use ratatui::{
    style::{Color, Stylize},
    widgets::Widget,
};
use strum::{Display, EnumIter};

/// A playing card.
///
/// # Example
///
/// ```rust
/// use tui_cards::{Card, Rank, Suit};
/// # fn draw(frame: &mut ratatui::Frame) {
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// frame.render_widget(&card, area);
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
    pub fn new(rank: Rank, suit: Suit) -> Self {
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
    pub fn as_symbol(self) -> char {
        match self {
            Rank::Ace => 'A',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
        }
    }
}

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Suit::Clubs => Color::Green,
            Suit::Diamonds => Color::Blue,
            Suit::Hearts => Color::Red,
            Suit::Spades => Color::Black,
        }
    }

    pub fn as_symbol(self) -> char {
        match self {
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }

    pub fn as_colored_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2663}\u{FE0F}",
            Self::Diamonds => "\u{2666}\u{FE0F}",
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }

    pub fn as_four_color_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2618}\u{FE0F}",     // shamrock
            Self::Diamonds => "\u{1F537}\u{FE0F}", // blue diamond
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }
}

impl Rank {
    pub fn template(self) -> &'static str {
        match self {
            Rank::Ace => indoc! {"
                ╭────────────╮
                │ A          │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │          A │
                ╰────────────╯"},
            Rank::Two => indoc! {"
                ╭────────────╮
                │ 2   xx     │
                │            │
                │            │
                │            │
                │            │
                │            │
                │     xx   2 │
                ╰────────────╯"},
            Rank::Three => indoc! {"
                ╭────────────╮
                │ 3   xx     │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │     xx   3 │
                ╰────────────╯"},
            Rank::Four => indoc! {"
                ╭────────────╮
                │ 4xx    xx  │
                │            │
                │            │
                │            │
                │            │
                │            │
                │  xx    xx4 │
                ╰────────────╯"},
            Rank::Five => indoc! {"
                ╭────────────╮
                │ 5xx    xx  │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │  xx    xx5 │
                ╰────────────╯"},
            Rank::Six => indoc! {"
                ╭────────────╮
                │ 6xx    xx  │
                │            │
                │            │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx6 │
                ╰────────────╯"},
            Rank::Seven => indoc! {"
                ╭────────────╮
                │ 7xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx7 │
                ╰────────────╯"},
            Rank::Eight => indoc! {"
                ╭────────────╮
                │ 8xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │     xx     │
                │            │
                │  xx    xx8 │
                ╰────────────╯"},
            Rank::Nine => indoc! {"
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
            Rank::Ten => indoc! {"
                ╭────────────╮
                │10xx    xx  │
                │     xx     │
                │  xx    xx  │
                │            │
                │  xx    xx  │
                │     xx     │
                │  xx    xx10│
                ╰────────────╯"},
            Rank::Jack => indoc! {"
                ╭────────────╮
                │ Jxx        │
                │       JJ   │
                │       JJ   │
                │       JJ   │
                │  JJ   JJ   │
                │   JJJJJ    │
                │        xxJ │
                ╰────────────╯"},
            Rank::Queen => indoc! {"
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
            Rank::King => indoc! {"
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
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
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
