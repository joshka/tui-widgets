use ratatui_core::style::Stylize;
use ratatui_core::text::Span;

/// The result of a prompt.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    #[default]
    Pending,
    Aborted,
    Done,
}

impl Status {
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    #[must_use]
    pub const fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted)
    }

    #[must_use]
    pub const fn is_done(&self) -> bool {
        matches!(self, Self::Done)
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::Done | Self::Aborted)
    }

    #[must_use]
    pub fn symbol(&self) -> Span<'static> {
        match self {
            Self::Pending => Symbols::default().pending,
            Self::Aborted => Symbols::default().aborted,
            Self::Done => Symbols::default().done,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbols {
    pub pending: Span<'static>,
    pub aborted: Span<'static>,
    pub done: Span<'static>,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            pending: "?".cyan(),
            aborted: "✘".red(),
            done: "✔".green(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_symbols() {
        assert_eq!(Status::Pending.symbol(), "?".cyan());
        assert_eq!(Status::Aborted.symbol(), "✘".red());
        assert_eq!(Status::Done.symbol(), "✔".green());
    }

    #[test]
    fn status_is_pending() {
        assert!(Status::Pending.is_pending());
        assert!(!Status::Aborted.is_pending());
        assert!(!Status::Done.is_pending());
    }

    #[test]
    fn status_is_aborted() {
        assert!(!Status::Pending.is_aborted());
        assert!(Status::Aborted.is_aborted());
        assert!(!Status::Done.is_aborted());
    }

    #[test]
    fn status_is_done() {
        assert!(!Status::Pending.is_done());
        assert!(!Status::Aborted.is_done());
        assert!(Status::Done.is_done());
    }

    #[test]
    fn status_is_finished() {
        assert!(!Status::Pending.is_finished());
        assert!(Status::Aborted.is_finished());
        assert!(Status::Done.is_finished());
    }

    #[test]
    fn status_default() {
        assert_eq!(Status::default(), Status::Pending);
    }

    #[test]
    fn symbols_default() {
        assert_eq!(
            Symbols::default(),
            Symbols {
                pending: "?".cyan(),
                aborted: "✘".red(),
                done: "✔".green(),
            }
        );
    }

    #[test]
    fn symbols_custom() {
        let symbols = Symbols {
            pending: "P".cyan(),
            aborted: "A".red(),
            done: "D".green(),
        };
        assert_eq!(symbols.pending, "P".cyan());
        assert_eq!(symbols.aborted, "A".red());
        assert_eq!(symbols.done, "D".green());
    }
}
