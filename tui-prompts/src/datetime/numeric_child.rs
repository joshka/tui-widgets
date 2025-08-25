use crate::prelude::*;
use ratatui::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct NumericChildState<'a> {
    status: Status,
    pub focus: FocusState,
    position: usize,
    default_char: char,
    value: Cow<'a, str>,
    max_width: usize,
}

impl<'a> NumericChildState<'a> {
    #[must_use]
    pub const fn new(max_width: usize, default_char: char, value: Cow<'a, str>) -> Self {
        Self {
            status: Status::Pending,
            focus: FocusState::Unfocused,
            default_char,
            position: 0,
            value,
            max_width,
        }
    }

    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub const fn with_focus(mut self, focus: FocusState) -> Self {
        self.focus = focus;
        self
    }

    #[must_use]
    pub fn with_value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = value.into();
        self
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    pub fn as_numeric(&self) -> u32 {
        self.value.parse::<u32>().unwrap()
    }

    pub fn from_numeric(&mut self, n: impl num::Integer + ToString) {
        self.value = n.to_string().into();
    }

    fn value_padded(&self) -> String {
        format!("{:0width$}", &self.value, width = self.max_width)
    }
}

impl StateCommon for NumericChildState<'_> {
    fn final_position(&self) -> usize {
        self.value().chars().count()
    }

    fn is_at_end(&self) -> bool {
        self.position() == self.max_width
    }

    fn status(&self) -> Status {
        self.status
    }

    fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }

    fn focus_state(&self) -> FocusState {
        self.focus
    }

    fn position(&self) -> usize {
        self.position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }

    fn is_valid_value(&self) -> bool {
        self.value.parse::<u32>().is_ok()
    }
}

impl TextualState for NumericChildState<'_> {
    fn len(&self) -> usize {
        self.value.len()
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        self.value.to_mut()
    }

    fn is_valid_char(&self, c: char) -> bool {
        c.is_ascii_digit()
    }
}

/// A prompt widget that displays a message and a text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NumericChildPrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    render_style: NumericRenderStyle,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericRenderStyle {
    #[default]
    Default,
}

impl NumericRenderStyle {
    #[must_use]
    pub fn render(&self, state: &NumericChildState) -> Span {
        match self {
            Self::Default => {
                match state.len() {
                    0 => {
                        Span::raw(state.default_char.to_string().repeat(state.max_width)).style(
                            Style::new()
                                .bg(Color::Rgb(30, 30, 30))
                                .fg(Color::Rgb(140, 140, 140)),
                        )
                    },
                    _ => {
                        Span::raw(state.value_padded().to_string())
                            .style(Style::new().bg(Color::Rgb(30, 30, 30)).fg(Color::White))
                    },
                }
            }
        }
    }
}

impl<'a> NumericChildPrompt<'a> {
    #[must_use]
    pub const fn new(message: Cow<'a, str>) -> Self {
        Self {
            message,
            render_style: NumericRenderStyle::Default,
        }
    }

    #[must_use]
    pub const fn with_render_style(mut self, render_style: NumericRenderStyle) -> Self {
        self.render_style = render_style;
        self
    }

    pub fn label_span(&self, style: Option<Style>) -> Span {
        let span = Span::raw(self.message.clone());
        match style {
            Some(st) => span.patch_style(st),
            None => span,
        }
    }

    pub fn value_span(&self, style: Option<Style>, state: &NumericChildState<'a>) -> Span {
        let span = self.render_style.render(state);
        match style {
            Some(st) => span.patch_style(st),
            None => span,
        }
    }
}

impl<T> From<T> for NumericChildPrompt<'static>
where
    T: Into<Cow<'static, str>>,
{
    fn from(message: T) -> Self {
        Self::new(message.into())
    }
}
