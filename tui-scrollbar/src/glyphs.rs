//! Glyph configuration for scrollbar rendering.

/// Glyphs used to render the track, arrows, and thumb.
///
/// Arrays use indices 0..=7 to represent 1/8th through full coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlyphSet {
    /// Track glyph for vertical scrollbars.
    pub track_vertical: char,
    /// Track glyph for horizontal scrollbars.
    pub track_horizontal: char,
    /// Arrow glyph for the start of a vertical scrollbar (top).
    pub arrow_vertical_start: char,
    /// Arrow glyph for the end of a vertical scrollbar (bottom).
    pub arrow_vertical_end: char,
    /// Arrow glyph for the start of a horizontal scrollbar (left).
    pub arrow_horizontal_start: char,
    /// Arrow glyph for the end of a horizontal scrollbar (right).
    pub arrow_horizontal_end: char,
    /// Thumb glyphs for vertical lower fills (1/8th through full).
    pub thumb_vertical_lower: [char; 8],
    /// Thumb glyphs for vertical upper fills (1/8th through full).
    pub thumb_vertical_upper: [char; 8],
    /// Thumb glyphs for horizontal left fills (1/8th through full).
    pub thumb_horizontal_left: [char; 8],
    /// Thumb glyphs for horizontal right fills (1/8th through full).
    pub thumb_horizontal_right: [char; 8],
}

impl GlyphSet {
    /// Glyphs that mix standard block elements with legacy supplement glyphs.
    ///
    /// Use this to get full 1/8th coverage for upper and right edges that the standard block set
    /// lacks; these glyphs come from [Symbols for Legacy Computing].
    ///
    /// [Symbols for Legacy Computing]: https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing
    pub const fn symbols_for_legacy_computing() -> Self {
        let vertical_lower = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let vertical_upper = ['▔', '🮂', '🮃', '▀', '🮄', '🮅', '🮆', '█'];
        let horizontal_left = ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        let horizontal_right = ['▕', '🮇', '🮈', '▐', '🮉', '🮊', '🮋', '█'];
        Self {
            track_vertical: '│',
            track_horizontal: '─',
            arrow_vertical_start: '▲',
            arrow_vertical_end: '▼',
            arrow_horizontal_start: '◄',
            arrow_horizontal_end: '►',
            thumb_vertical_lower: vertical_lower,
            thumb_vertical_upper: vertical_upper,
            thumb_horizontal_left: horizontal_left,
            thumb_horizontal_right: horizontal_right,
        }
    }

    /// Glyphs using only standard Unicode block elements.
    ///
    /// Use this if your font lacks the legacy glyphs; upper/right partials will use the same
    /// glyphs as lower/left partials.
    pub const fn unicode() -> Self {
        let vertical = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let horizontal = ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        Self {
            track_vertical: '│',
            track_horizontal: '─',
            arrow_vertical_start: '▲',
            arrow_vertical_end: '▼',
            arrow_horizontal_start: '◄',
            arrow_horizontal_end: '►',
            thumb_vertical_lower: vertical,
            thumb_vertical_upper: vertical,
            thumb_horizontal_left: horizontal,
            thumb_horizontal_right: horizontal,
        }
    }
}

impl Default for GlyphSet {
    fn default() -> Self {
        Self::symbols_for_legacy_computing()
    }
}
