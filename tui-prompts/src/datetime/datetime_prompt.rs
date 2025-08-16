use std::borrow::Cow;
use std::vec;

use itertools::Itertools;
use ratatui::prelude::*;
use ratatui::widgets::{Block, StatefulWidget, Widget};

use crate::prelude::*;

// TODO style the widget
// TODO style each element of the widget.
// TODO handle multi-line input.
// TODO handle scrolling.
// TODO handle vertical movement.
// TODO handle bracketed paste.

/// A prompt widget that displays a message and a text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DateTimePrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    /// The block to wrap the prompt in.
    block: Option<Block<'a>>,
    spacer: Span<'a>,
    child_prompts: Vec<NumericChildPrompt<'a>>,
}

impl<'a> DateTimePrompt<'a> {
    #[must_use]
    pub const fn new(
        message: Cow<'a, str>,
        child_prompts: Vec<NumericChildPrompt<'a>>,
        spacer: Span<'a>,
    ) -> Self {
        Self {
            message,
            block: None,
            spacer,
            child_prompts,
        }
    }

    #[must_use]
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Prompt for DateTimePrompt<'_> {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the `Widget` trait implementation as we need the `Frame` to set the
    /// cursor position.
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.is_focused() {
            frame.set_cursor_position(state.cursor());
        }
    }
}

impl<'a> StatefulWidget for DateTimePrompt<'a> {
    type State = DateTimeState<'a>;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.element_count() != self.child_prompts.len() {
            panic!("DateTimePrompt: number of NumericChildPrompts must equal number of fields");
        }
        self.render_block(&mut area, buf);
        let mut values: Vec<Span> = self
            .child_prompts
            .iter()
            .zip(state.elements().iter())
            .flat_map(|(prompt, state)| {
                vec![prompt.label_span(None), prompt.value_span(None, state)]
            })
            .collect();
        values = Itertools::intersperse(values.into_iter(), self.spacer).collect();
        values.insert(0, state.status().symbol());
        values.insert(1, Span::raw("  "));

        let cursor_index = 2 + (state.position() * 4) + 1;

        let length_to_cursor: usize = values[..(cursor_index)].iter().map(|x| x.width()).sum();
        let pos_in_prompt: usize = state.current_element().position() + 1;

        let line = Line::from(values);
        line.render(area, buf);
        *state.cursor_mut() = (
            length_to_cursor as u16 + pos_in_prompt as u16 + area.x + 1_u16,
            area.y,
        );
    }
}

impl DateTimePrompt<'_> {
    fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) {
        if let Some(block) = self.block.take() {
            let inner = block.inner(*area);
            block.render(*area, buf);
            *area = inner;
        };
    }
}
