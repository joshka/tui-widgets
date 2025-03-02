//! TUI QR Code is a library for rendering QR codes in a terminal using the [Ratatui] crate.
//!
//! [![Crate badge]][tui-qrcode]
//! [![Docs.rs Badge]][API Docs]
//! [![Deps.rs Badge]][Dependency Status]
//! [![License Badge]](./LICENSE-MIT)
//! [![Discord Badge]][Ratatui Discord]
//!
//! [GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]
//!
//! ![Demo](https://vhs.charm.sh/vhs-nUpcmCP1igCcGoJ5iio07.gif)
//!
//! # Usage
//!
//! Add qrcode and tui-qrcode to your Cargo.toml. You can disable the default features of qrcode as
//! we don't need the code which renders the QR code to an image.
//!
//! ```shell
//! cargo add qrcode tui-qrcode --no-default-features
//! ```
//!
//! # Example
//!
//! This example can be found in the `examples` directory of the repository.
//!
//! ```no_run
//! use qrcode::QrCode;
//! use ratatui::{crossterm::event, DefaultTerminal, Frame};
//! use tui_qrcode::{Colors, QrCodeWidget};
//!
//! fn main() -> color_eyre::Result<()> {
//!     color_eyre::install()?;
//!     let terminal = ratatui::init();
//!     let result = run(terminal);
//!     ratatui::restore();
//!     result
//! }
//!
//! fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
//!     loop {
//!         terminal.draw(render)?;
//!         if matches!(event::read()?, event::Event::Key(_)) {
//!             break Ok(());
//!         }
//!     }
//! }
//!
//! fn render(frame: &mut Frame) {
//!     let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
//!     let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
//!     frame.render_widget(widget, frame.area());
//! }
//! ```
//!
//! Renders the following QR code:
//!
//! ```text
//! █████████████████████████████████
//! █████████████████████████████████
//! ████ ▄▄▄▄▄ █▄ ▄▄▄ ████ ▄▄▄▄▄ ████
//! ████ █   █ █▄▄▄█▀▄██ █ █   █ ████
//! ████ █▄▄▄█ █▀   ▄▀ ███ █▄▄▄█ ████
//! ████▄▄▄▄▄▄▄█▄▀▄█ ▀▄▀ █▄▄▄▄▄▄▄████
//! ████ █▄▀▀▀▄▄▀▄▄  ▄█▀▄█▀ █▀▄▀ ████
//! ██████▀█  ▄▀▄▄▀▀ ▄ ▄█ ▄▄█ ▄█▄████
//! ████▄▀▀▀▄▄▄▄▀█▄▄█  ▀ ▀ ▀███▀ ████
//! ████▄▄ ▀█▄▄▀▄▄ ▄█▀█▄▀█▄▀▀ ▄█▄████
//! ████▄▄█▄██▄█ ▄▀▄ ▄█  ▄▄▄ ██▄▀████
//! ████ ▄▄▄▄▄ █▄▄▄▀ ▄ ▀ █▄█ ███ ████
//! ████ █   █ ██ ███  ▄▄ ▄▄ █▀ ▄████
//! ████ █▄▄▄█ █▄▀ ▄█▀█▀ ▄█  ▄█▄▄████
//! ████▄▄▄▄▄▄▄█▄▄█▄▄▄██▄█▄██▄██▄████
//! █████████████████████████████████
//! ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
//! ```
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//! [Crate badge]: https://img.shields.io/crates/v/tui-qrcode.svg?style=for-the-badge
//! [tui-qrcode]: https://crates.io/crates/tui-qrcode
//! [Docs.rs Badge]: https://img.shields.io/badge/docs.rs-tui--qrcode-blue?style=for-the-badge
//! [API Docs]: https://docs.rs/tui-qrcode
//! [Deps.rs Badge]: https://deps.rs/repo/github/joshka/tui-qrcode/status.svg?style=for-the-badge
//! [Dependency Status]: https://deps.rs/repo/github/joshka/tui-qrcode
//! [License Badge]: https://img.shields.io/crates/l/tui-qrcode?style=for-the-badge
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [GitHub Repository]: https://github.com/joshka/tui-widgets
//! [Examples]: https://github.com/joshka/tui-widgets/tree/main/tui-qrcode/examples
//! [Changelog]: https://github.com/joshka/tui-widgets/blob/main/tui-qrcode/CHANGELOG.md
//! [Contributing]: https://github.com/joshka/tui-widgets/blob/main/CONTRIBUTING.md

use qrcode::{render::unicode::Dense1x2, QrCode};
use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled},
    text::Text,
    widgets::Widget,
};

/// A [Ratatui] widget that renders a QR code.
///
/// This widget can be used to render a QR code in a terminal. It uses the [qrcode] crate to
/// generate the QR code.
///
/// # Examples
///
/// ```no_run
/// use qrcode::QrCode;
/// use tui_qrcode::QrCodeWidget;
///
/// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
/// let widget = QrCodeWidget::new(qr_code);
/// ```
///
/// The widget can be customized using the `quiet_zone`, `scaling`, `colors`, and `style` methods.
/// Additionally, the widget implements the `Styled` trait, so all the methods from Ratatui's
/// [Stylize](ratatui_core::style::Stylize) trait can be used.
///
/// ```no_run
/// use qrcode::QrCode;
/// use tui_qrcode::{Colors, QrCodeWidget, QuietZone, Scaling};
/// use ratatui::{Frame, style::{Style, Stylize}};
///
/// fn render(frame: &mut Frame) {
///     let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
///     let widget = QrCodeWidget::new(qr_code)
///         .quiet_zone(QuietZone::Disabled)
///         .scaling(Scaling::Max)
///         .colors(Colors::Inverted)
///         .red()
///         .on_light_yellow();
///     frame.render_widget(widget, frame.area());
/// }
/// ```
///
/// [Ratatui]: https://crates.io/crates/ratatui
pub struct QrCodeWidget {
    qr_code: QrCode,
    quiet_zone: QuietZone,
    scaling: Scaling,
    colors: Colors,
    style: Style,
}

/// The quiet zone (border) of a QR code.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum QuietZone {
    /// The quiet zone is enabled.
    #[default]
    Enabled,
    /// The quiet zone is disabled.
    Disabled,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scaling {
    /// The QR code will be scaled to at least the size of the widget.
    ///
    /// Note that this usually results in a QR code that is larger than the widget, which is not
    /// ideal.
    Min,

    /// The QR code will be scaled to be at most the size of the widget.
    ///
    /// Note that this may result in a QR code which is scaled more horizontally or vertically than
    /// the other, which may not be ideal.
    Max,

    /// The QR code will be scaled so each pixel is the size of the given dimensions.
    ///
    /// The minimum dimensions are 1x1 (width x height).
    Exact(u32, u32),
}

impl Default for Scaling {
    fn default() -> Self {
        Self::Exact(1, 1)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Colors {
    /// The default colors. (Black on white)
    #[default]
    Normal,

    /// The colors are inverted. (White on black)
    Inverted,
}

impl QrCodeWidget {
    /// Create a new QR code widget.
    #[must_use]
    pub fn new(qr_code: QrCode) -> Self {
        Self {
            qr_code,
            quiet_zone: QuietZone::default(),
            scaling: Scaling::default(),
            colors: Colors::default(),
            style: Style::default(),
        }
    }

    /// Set whether the QR code should have a quiet zone.
    ///
    /// This is the white border around the QR code. By default, the quiet zone is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{QrCodeWidget, QuietZone};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).quiet_zone(QuietZone::Disabled);
    /// ```
    #[must_use]
    pub fn quiet_zone(mut self, quiet_zone: QuietZone) -> Self {
        self.quiet_zone = quiet_zone;
        self
    }

    /// Set how the QR code should be scaled.
    ///
    /// By default, the QR code will be scaled so each pixel is 1x1.
    ///
    /// The `Min` variant will scale the QR code so it is at least the size of the widget. This may
    /// result in a QR code that is larger than the widget, which is not ideal. The `Max` variant
    /// will scale the QR code so it is at most the size of the widget. This may result in a QR code
    /// which is scaled more horizontally or vertically than the other, which may not be ideal. The
    /// `Exact` variant will scale the QR code so each pixel is the size of the given dimensions.
    /// The minimum dimensions are 1x1 (width x height).
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{QrCodeWidget, Scaling};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).scaling(Scaling::Max);
    /// ```
    #[must_use]
    pub fn scaling(mut self, scaling: Scaling) -> Self {
        self.scaling = scaling;
        self
    }

    /// Set the colors of the QR code.
    ///
    /// By default, the colors are normal (light on dark).
    ///
    /// The `Normal` variant will use the default colors. The `Inverted` variant will invert the
    /// colors (dark on light).
    ///
    /// To set the foreground and background colors of the widget, use the `style` method.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{QrCodeWidget, Colors};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
    /// ```
    #[must_use]
    pub fn colors(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }

    /// Set the style of the widget.
    ///
    /// This will set the foreground and background colors of the widget.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::QrCodeWidget;
    /// use ratatui::style::{Style, Stylize};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let style = Style::new().red().on_light_yellow();
    /// let widget = QrCodeWidget::new(qr_code).style(style);
    /// ```
    #[must_use]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl Styled for QrCodeWidget {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for QrCodeWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &QrCodeWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut renderer = self.qr_code.render::<Dense1x2>();
        match self.quiet_zone {
            QuietZone::Enabled => renderer.quiet_zone(true),
            QuietZone::Disabled => renderer.quiet_zone(false),
        };
        match self.scaling {
            Scaling::Min => renderer.min_dimensions(area.width as u32, area.height as u32 * 2),
            Scaling::Max => renderer.max_dimensions(area.width as u32, area.height as u32 * 2),
            Scaling::Exact(width, height) => renderer.module_dimensions(width, height),
        };
        match self.colors {
            Colors::Normal => renderer
                .dark_color(Dense1x2::Dark)
                .light_color(Dense1x2::Light),
            Colors::Inverted => renderer
                .dark_color(Dense1x2::Light)
                .light_color(Dense1x2::Dark),
        };
        Text::raw(renderer.build())
            .style(self.style)
            .render(area, buf);
    }
}
