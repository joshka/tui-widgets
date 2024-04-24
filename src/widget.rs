use std::cmp::min;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, StatefulWidgetRef, Widget, WidgetRef},
};

use crate::{popup::SizedWidgetRef, Popup, PopupState};

impl<W: SizedWidgetRef> WidgetRef for Popup<'_, W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = PopupState::default();
        StatefulWidgetRef::render_ref(self, area, buf, &mut state);
    }
}

impl<W: SizedWidgetRef> StatefulWidgetRef for Popup<'_, W> {
    type State = PopupState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = if let Some(next) = state.area.take() {
            // ensure that the popup remains on screen
            let width = min(next.width, area.width);
            let height = min(next.height, area.height);
            let x = next.x.clamp(buf.area.x, area.right() - width);
            let y = next.y.clamp(buf.area.y, area.bottom() - height);

            Rect::new(x, y, width, height)
        } else {
            let height = self
                .body
                .height()
                .saturating_add(2)
                .try_into()
                .unwrap_or(area.height);
            let width = self
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
            .title(self.title.clone())
            .style(self.style);
        block.render_ref(area, buf);
        self.body.render_ref(block.inner(area), buf);
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
