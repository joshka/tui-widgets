use std::cmp::min;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::{Popup, PopupState};

/// A Ratatui widget that renders a popup.
#[derive(Clone, Debug)]
pub struct PopupWidget<'content> {
    pub popup: &'content Popup<'content>,
}

impl Widget for PopupWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for PopupWidget<'_> {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let popup = self.popup;

        let area = if let Some(next) = state.area.take() {
            // ensure that the popup remains on screen
            let width = min(next.width, area.width);
            let height = min(next.height, area.height);
            let x = next.x.clamp(buf.area.x, area.right() - width);
            let y = next.y.clamp(buf.area.y, area.bottom() - height);

            Rect::new(x, y, width, height)
        } else {
            let height = popup
                .body
                .height()
                .saturating_add(2)
                .try_into()
                .unwrap_or(area.height);
            let width = popup
                .body
                .width()
                .saturating_add(2)
                .try_into()
                .unwrap_or(area.width);
            centered_rect(width, height, area)
        };

        state.area.replace(area);

        Clear.render(area, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.popup.title.clone());
        Paragraph::new(self.popup.body.clone())
            .block(block)
            .style(popup.style)
            .render(area, buf);
    }
}

/// Create a rectangle centered in the given area.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.width.saturating_sub(width) / 2,
        y: area.height.saturating_sub(height) / 2,
        width: min(width, area.width),
        height: min(height, area.height),
    }
}
