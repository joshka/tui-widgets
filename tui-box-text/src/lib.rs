use std::{collections::HashMap, iter::zip, sync::LazyLock};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct BoxChar(char);

impl BoxChar {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Widget for &BoxChar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let c = self
            .0
            .to_uppercase() // TODO: add support for lower case characters
            .next()
            .and_then(|c| CHARS.get(&c))
            .unwrap_or(&" ");
        let lines = c.lines().collect::<Vec<_>>();
        for (line, row) in zip(lines, area.rows()) {
            for (char, cell) in zip(line.chars(), row.columns()) {
                buf[cell.as_position()].set_symbol(&char.to_string());
            }
        }
    }
}

/// A macro for creating a hash table that maps single characters to strings.
macro_rules! char_table {
    ( $($char:expr => $repr:expr),* $(,)? ) => {
        {
            let mut table = ::std::collections::HashMap::new();
            $(
                table.insert($char, ::indoc::indoc! {$repr});
            )*
            table
        }
    };
}

/// A hash table that maps single characters to strings that are 3 lines high and made up of box
/// drawing characters.
static CHARS: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    char_table!(
        ' ' => " ",
        '!' => "│
                ╵
                ╵",
        '"' => "╭╭",
        '#' => "┼─┼
                ┼─┼",
        '$' => "╭┼╴
                └┼┐
                ╶┼╯",
        '%' => "o╱
                ╱o",
        '&' => "╭─╮
                ╭╲╯
                ╰─╲",
        '\'' => "╭",
        '(' => "╭
                │
                ╰",
        ')' => "╮
                │
                ╯",
        '*' => "
        
                *
                ",
        '+' => "
                 ╷
                ╶┼╴
                 ╵",
        ',' => "

                
                ╯",
        '-' => "

                ──
                 ",
        '.' => "

                .
                 ",
        '/' => "
                 ╱
                ╱
                ",
        '0' => "╭─╮
                │╱│
                ╰─╯",
        '1' => "
                 ╶┐
                  │
                 ─┴─",
        '2' => "╶─╮
                ┌─┘
                └─╴",
        '3' => "╶─╮
                ╶─┤
                ╶─╯",
        '4' => "╷ ╷
                ╰─┤
                  ╵",
        '5' => "┌─╴
                └─╮
                ╰─╯",
        '6' => "╭─╴
                ├─╮
                ╰─╯",
        '7' => "╶─┐
                 ╱
                ╵  ",
        '8' => "╭─╮
                ├─┤
                ╰─╯",
        '9' => "╭─╮
                ╰─┤
                ╶─╯",
        ':' => "╷
                ╵
                │
                 ",
        ';' => "╷
                ╵
                ╯",
        '<' => "
                 ╱
                 ╲
                 ",
        '=' => "
                ──
                ──",
        '>' => "
                 ╲
                 ╱
                 ",
        '?' => "
                ╶─╮
                 ╭╯
                 ╷",
        '@' => "╭─╮
                ╭╮│
                ╰┴╯",
        'A' => "╭─╮
                ├─┤
                ╵ ╵",
        'B' => "┌╮
                ├┴╮
                ╰─╯",
        'C' => "╭─╮
                │
                ╰─╯",
        'D' => "┌─╮
                │ │
                └─╯",
        'E' => "┌─╴
                ├─
                └─╴",
        'F' => "┌─╴
                ├─
                ╵  ",
        'G' => "╭─╮
                │─╮
                ╰─╯",
        'H' => "╷ ╷
                ├─┤
                ╵ ╵",
        'I' => "╶┬╴
                 │
                ╶┴╴",
        'J' => " ╶┐
                  │
                ╰─╯",
        'K' => "╷╭
                ├┴╮
                ╵ ╵",
        'L' => "╷
                │
                └──",
        'M' => "╭┬╮
                │││
                ╵╵╵",
        'N' => "╭─╮
                │ │
                ╵ ╵",
        'O' => "╭─╮
                │ │
                ╰─╯",
        'P' => "┌─╮
                ├─╯
                ╵  ",
        'Q' => "╭─╮
                │ │
                ╰─╳",
        'R' => "┌─╮
                ├┬╯
                ╵╰ ",
        'S' => "╭─╮
                ╰─╮ 
                ╰─╯",
        'T' => "
                ╶┬╴
                 │
                 ╵",
        'U' => "╷ ╷
                │ │
                ╰─╯",
        'V' => "╷ ╷
                │ │
                └─╯",
        'W' => "╷╷╷
                │││
                ╰┴╯",
        'X' => "╮ ╭
                ╰─╮ 
                ╯ ╰",
        'Y' => "╮ ╭
                ╰┬╯ 
                 ╵",
        'Z' => "╶─╮
                 ╱
                ╰─╴",
        '[' => "┌─
                │
                └─",
        '\\' => "
                 ╲
                  ╲
                ",
        ']' => "─┐
                 │
                ─┘",
        '^' => "╱╲",
        '_' => "

                ──",
        '`' => "╮",
        '{' => "
                ╭
                ┤
                ╰",
        '|' => "│
                │
                │",
        '}' => "╮
                ├
                ╯",
        '~' => "
                ╭╮
                 ╰╯",
    )
});
