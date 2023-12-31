//! A popup widget for [Ratatui](https://ratatui.rs)
//!
//! # Example
//!
//! ```rust
//! use ratatui::prelude::*;
//! use tui_popup::Popup;
//!
//! fn render(frame: &mut Frame) {
//!     let popup = Popup::new("tui-popup demo", "Press any key to exit");
//!     frame.render_widget(popup.to_widget(), frame.size());
//! }
//! ```

use std::cmp::min;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct Popup<'content> {
    pub title: Line<'content>,
    pub body: Text<'content>,
}

pub struct PopupWidget<'content> {
    popup: &'content Popup<'content>,
}

impl<'content> Popup<'content> {
    pub fn new<L, T>(title: L, body: T) -> Self
    where
        L: Into<Line<'content>>,
        T: Into<Text<'content>>,
    {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }

    pub fn to_widget(&'content self) -> PopupWidget<'content> {
        PopupWidget { popup: self }
    }
}

impl Widget for PopupWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = self.popup;
        let height = popup.body.height() as u16 + 2;
        let width = popup.body.width() as u16 + 2;
        let area = centered_rect(width, height, area);
        Clear.render(area, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.popup.title.clone());
        Paragraph::new(self.popup.body.clone())
            .block(block)
            .render(area, buf);
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.width.saturating_sub(width) / 2,
        y: area.height.saturating_sub(height) / 2,
        width: min(width, area.width),
        height: min(height, area.height),
    }
}
