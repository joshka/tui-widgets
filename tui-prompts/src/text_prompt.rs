use std::{borrow::Cow, vec};

use crate::prelude::*;

use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

// TODO style the widget
// TODO style each element of the widget.
// TODO handle multi-line input.
// TODO handle scrolling.
// TODO handle vertical movement.
// TODO handle bracketed paste.

/// A prompt widget that displays a message and a text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextPrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    /// The block to wrap the prompt in.
    block: Option<Block<'a>>,
    render_style: TextRenderStyle,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextRenderStyle {
    #[default]
    Default,
    Password,
    Invisible,
}

impl TextRenderStyle {
    #[must_use]
    pub fn render(&self, state: &TextState) -> String {
        match self {
            Self::Default => state.value().to_string(),
            Self::Password => "*".repeat(state.len()),
            Self::Invisible => String::new(),
        }
    }
}

impl<'a> TextPrompt<'a> {
    #[must_use]
    pub const fn new(message: Cow<'a, str>) -> Self {
        Self {
            message,
            block: None,
            render_style: TextRenderStyle::Default,
        }
    }

    #[must_use]
    // const causes: error[E0493]: destructor of `std::option::Option<ratatui::widgets::Block<'_>>` cannot be evaluated at compile-time
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    #[must_use]
    pub const fn with_render_style(mut self, render_style: TextRenderStyle) -> Self {
        self.render_style = render_style;
        self
    }
}

impl Prompt for TextPrompt<'_> {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the `Widget` trait implementation as we need the `Frame` to set the
    /// cursor position.
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.is_focused() {
            frame.set_cursor(state.cursor().0, state.cursor().1);
        }
    }
}

impl<'a> StatefulWidget for TextPrompt<'a> {
    type State = TextState<'a>;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_block(&mut area, buf);

        let width = area.width as usize;
        let height = area.height as usize;
        let value = self.render_style.render(state);
        let value_length = value.chars().count();

        let line = Line::from(vec![
            state.status().symbol(),
            " ".into(),
            self.message.bold(),
            " › ".cyan().dim(),
            Span::raw(value),
        ]);
        let prompt_length = line.width() - value_length;
        let lines = wrap(line, width).take(height).collect_vec();

        // constrain the position to the area
        let position = (state.position() + prompt_length).min(area.area() as usize - 1);
        let row = position / width;
        let column = position % width;
        // sizes are already constrained to the u16 range
        #[allow(clippy::cast_possible_truncation)]
        {
            *state.cursor_mut() = (area.x + column as u16, area.y + row as u16);
        }
        Paragraph::new(lines).render(area, buf);
    }
}

/// wraps a line into multiple lines of the given width.
///
/// This is a character based wrap, not a word based wrap.
///
/// TODO: move this into the `Line` type.
fn wrap(line: Line, width: usize) -> impl Iterator<Item = Line> {
    let mut line = line;
    std::iter::from_fn(move || {
        if line.width() > width {
            let (first, second) = line_split_at(line.clone(), width);
            line = second;
            Some(first)
        } else if line.width() > 0 {
            let first = line.clone();
            line = Line::default();
            Some(first)
        } else {
            None
        }
    })
}

/// splits a line into two lines at the given position.
///
/// TODO: move this into the `Line` type.
/// TODO: fix this so that it operates on multi-width characters.
fn line_split_at(line: Line, mid: usize) -> (Line, Line) {
    let mut first = Line::default();
    let mut second = Line::default();
    first.alignment = line.alignment;
    second.alignment = line.alignment;
    for span in line.spans {
        let first_width = first.width();
        let span_width = span.width();
        if first_width + span_width <= mid {
            first.spans.push(span);
        } else if first_width < mid && first_width + span_width > mid {
            let span_mid = mid - first_width;
            let (span_first, span_second) = span_split_at(span, span_mid);
            first.spans.push(span_first);
            second.spans.push(span_second);
        } else {
            second.spans.push(span);
        }
    }
    (first, second)
}

/// splits a span into two spans at the given position.
///
/// TODO: move this into the `Span` type.
/// TODO: fix this so that it operates on multi-width characters.
#[allow(clippy::needless_pass_by_value)]
fn span_split_at(span: Span, mid: usize) -> (Span, Span) {
    let (first, second) = span.content.split_at(mid);
    let first = Span {
        content: Cow::Owned(first.into()),
        style: span.style,
    };
    let second = Span {
        content: Cow::Owned(second.into()),
        style: span.style,
    };
    (first, second)
}

impl<'a> TextPrompt<'a> {
    fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) {
        if let Some(block) = self.block.take() {
            let inner = block.inner(*area);
            block.render(*area, buf);
            *area = inner;
        };
    }
}

impl<T> From<T> for TextPrompt<'static>
where
    T: Into<Cow<'static, str>>,
{
    fn from(message: T) -> Self {
        Self::new(message.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Status;
    use ratatui_macros::line;
    use rstest::{fixture, rstest};

    use super::*;
    use ratatui::{backend::TestBackend, widgets::Borders};

    #[test]
    fn new() {
        const PROMPT: TextPrompt<'_> = TextPrompt::new(Cow::Borrowed("Enter your name"));
        assert_eq!(PROMPT.message, "Enter your name");
        assert_eq!(PROMPT.block, None);
        assert_eq!(PROMPT.render_style, TextRenderStyle::Default);
    }

    #[test]
    fn default() {
        let prompt = TextPrompt::default();
        assert_eq!(prompt.message, "");
        assert_eq!(prompt.block, None);
        assert_eq!(prompt.render_style, TextRenderStyle::Default);
    }

    #[test]
    fn from() {
        let prompt = TextPrompt::from("Enter your name");
        assert_eq!(prompt.message, "Enter your name");
    }

    #[test]
    fn render() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line!["?".cyan(), " ", "prompt".bold(), " › ".cyan().dim(), "    ",];
        assert_eq!(buffer, Buffer::with_lines([line]));
        assert_eq!(state.cursor(), (11, 0));
    }

    #[test]
    fn render_emoji() {
        let prompt = TextPrompt::from("🔍");
        let mut state = TextState::new();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 11, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line!["?".cyan(), " ", "🔍".bold(), " › ".cyan().dim(), "    "];
        assert_eq!(buffer, Buffer::with_lines([line]));
        assert_eq!(state.cursor(), (7, 0));
    }

    #[test]
    fn render_with_done() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_status(Status::Done);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line![
            "✔".green(),
            " ",
            "prompt".bold(),
            " › ".cyan().dim(),
            "    "
        ];
        assert_eq!(buffer, Buffer::with_lines([line]));
    }

    #[test]
    fn render_with_aborted() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_status(Status::Aborted);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line!["✘".red(), " ", "prompt".bold(), " › ".cyan().dim(), "    "];
        assert_eq!(buffer, Buffer::with_lines([line]));
    }

    #[test]
    fn render_with_value() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line![
            "?".cyan(),
            " ",
            "prompt".bold(),
            " › ".cyan().dim(),
            "value              ".to_string()
        ];
        assert_eq!(buffer, Buffer::with_lines([line]));
    }

    #[test]
    fn render_with_block() {
        let prompt = TextPrompt::from("prompt")
            .with_block(Block::default().borders(Borders::ALL).title("Title"));
        let mut state = TextState::new();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec![
            "┌Title────────┐",
            "│? prompt ›   │",
            "└─────────────┘",
        ]);
        expected.set_style(Rect::new(1, 1, 1, 1), Color::Cyan);
        expected.set_style(Rect::new(3, 1, 6, 1), Modifier::BOLD);
        expected.set_style(Rect::new(9, 1, 3, 1), (Color::Cyan, Modifier::DIM));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_password() {
        let prompt = TextPrompt::from("prompt").with_render_style(TextRenderStyle::Password);
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line![
            "?".cyan(),
            " ",
            "prompt".bold(),
            " › ".cyan().dim(),
            "*****              ".to_string()
        ];
        assert_eq!(buffer, Buffer::with_lines([line]));
    }

    #[test]
    fn render_invisible() {
        let prompt = TextPrompt::from("prompt").with_render_style(TextRenderStyle::Invisible);
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let line = line![
            "?".cyan(),
            " ",
            "prompt".bold(),
            " › ".cyan().dim(),
            "                   ".to_string()
        ];
        assert_eq!(buffer, Buffer::with_lines([line]));
    }

    #[fixture]
    fn terminal() -> Terminal<TestBackend> {
        Terminal::new(TestBackend::new(17, 2)).unwrap()
    }

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[rstest]
    fn draw_not_focused<'a>(mut terminal: Terminal<impl Backend>) -> Result<()> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello");
        // The cursor is not changed when the prompt is not focused.
        let _ = terminal.draw(|frame| prompt.draw(frame, frame.size(), &mut state))?;
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.backend_mut().get_cursor().unwrap(), (0, 0));
        Ok(())
    }

    #[rstest]
    fn draw_focused<'a>(mut terminal: Terminal<impl Backend>) -> Result<()> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello");
        // The cursor is changed when the prompt is focused.
        state.focus();
        let _ = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.backend_mut().get_cursor().unwrap(), (11, 0));
        Ok(())
    }

    #[rstest]
    #[case::position_0(0, (11, 0))] // start of value
    #[case::position_3(2, (13, 0))] // middle of value
    #[case::position_4(4, (15, 0))] // last character of value
    #[case::position_5(5, (16, 0))] // one character beyond the value
    #[case::position_6(6, (0, 1))] // FIXME: should not go beyond the value
    #[case::position_7(7, (1, 1))] // FIXME: should not go beyond the value
    #[case::position_22(22, (16, 1))] // FIXME: should not go beyond the value
    #[case::position_99(99, (16, 1))] // FIXME: should not go beyond the value
    fn draw_unwrapped_position<'a>(
        #[case] position: usize,
        #[case] expected_cursor: (u16, u16),
        mut terminal: Terminal<impl Backend>,
    ) -> Result<()> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello");
        // expected: "? prompt › hello "
        //           "                 "
        // position:             012345
        // cursor:    01234567890123456
        // The cursor is changed when the prompt is focused and the position is changed.
        state.focus();
        *state.position_mut() = position;
        let _ = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_eq!(state.cursor(), expected_cursor);
        assert_eq!(terminal.get_cursor()?, expected_cursor);

        Ok(())
    }

    #[rstest]
    #[case::position_0(0, (11, 0))] // start of value
    #[case::position_1(3, (14, 0))] // middle of value
    #[case::position_5(5, (16, 0))] // end of line
    #[case::position_6(6, (0, 1))] // first character of the second line
    #[case::position_7(7, (1, 1))] // second character of the second line
    #[case::position_11(10, (4, 1))] // last character of the value
    #[case::position_12(12, (6, 1))] // one character beyond the value
    #[case::position_13(13, (7, 1))] // FIXME: should not go beyond the value
    #[case::position_22(22, (16, 1))] // FIXME: should not go beyond the value
    #[case::position_99(99, (16, 1))] // FIXME: should not go beyond the value
    fn draw_wrapped_position<'a>(
        #[case] position: usize,
        #[case] expected_cursor: (u16, u16),
        mut terminal: Terminal<impl Backend>,
    ) -> Result<()> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello world");
        // line 1:   "? prompt › hello "
        // position:             012345
        // cursor:    01234567890123456
        // line 2:   "world            "
        // position:  678901
        // cursor:    01234567890123456
        // The cursor is changed when the prompt is focused and the position is changed.
        state.focus();
        *state.position_mut() = position;
        let _ = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_eq!(state.cursor(), expected_cursor);
        assert_eq!(terminal.get_cursor()?, expected_cursor);

        Ok(())
    }
}
