---
date: 2025-12-27T12:39:41-08:00
repository: tui-widgets
revision: 98c03222 (jj: nlovkkzo)
branch: (none)
status: draft
---

# tui-scrollbar Implementation Plan

## Overview

Add a new `tui-scrollbar` crate that renders a proportional scrollbar thumb using 1/8 block
characters, supports mouse interaction (click/drag/wheel), and is usable without Ratatui’s
`StatefulWidget` pattern by implementing `Widget` for `&ScrollBar` (and `&mut ScrollBar` only if we
truly need mutation during render).

This should serve as a generally useful scrollbar widget and (optionally) replace the current
Ratatui scrollbar usage inside `tui-scrollview`.

## Current state

- The workspace is a `tui-*` crate collection (`Cargo.toml:1`), and the root facade re-exports
  widgets behind feature flags (`Cargo.toml:60`).
- `tui-scrollview` currently uses Ratatui’s scrollbar widget and state:
  - Imports `ratatui_widgets::scrollbar::{Scrollbar, ScrollbarOrientation, ScrollbarState}`
    (`tui-scrollview/src/scroll_view.rs:1`).
  - Renders vertical/horizontal scrollbars via `ScrollbarState::new(...).position(...)` and
    `Scrollbar::new(...).render(...)` (`tui-scrollview/src/scroll_view.rs:341`).
  - Tests assert the rendered scrollbar glyphs like `▲`, `█`, `║`, `▼`
    (`tui-scrollview/src/scroll_view.rs:404`), implying the current visual design is inherited from
    Ratatui’s implementation.
- There is an established pattern for optional crossterm mouse handling behind a feature flag:
  `tui-popup` exposes `PopupState::handle_mouse_event` gated by a `crossterm` feature
  (`tui-popup/Cargo.toml:21`, `tui-popup/src/popup_state.rs:97`). This is useful prior art for
  feature gating, but the popup’s “store last rendered `Rect` + drag state” model may not be the
  best fit for a scrollbar.
- CI checks READMEs via `cargo rdme --check` per crate, so a new crate should be added there
  (`.github/workflows/check.yml:34`).

## Desired end state

### Rendering

- A `tui-scrollbar` widget renders a “track” plus a “thumb” whose size is proportional to
  `viewport_len / content_len`.
- Thumb position is proportional to `offset / (content_len - viewport_len)`.
- The thumb uses fractional cell rendering:
  - Vertical scrollbar uses Unicode block elements with 1/8 cell height increments.
  - Horizontal scrollbar uses Unicode block elements with 1/8 cell width increments.
- Optional arrow endcaps render at the start/end of the track to indicate scroll direction and
  support click-to-step; arrows are configurable and compatible with fractional thumb rendering.
- Edge cases behave predictably:
  - `content_len <= viewport_len` results in “no scrolling”; thumb either fills the track or the bar
    can be configured to hide/disable itself.
  - Very large `content_len` still renders a minimally visible thumb (at least 1 full cell, for
    easier targeting / Fitts’s law).
  - `area.width == 0` or `area.height == 0` renders nothing safely.

### API shape (no `StatefulWidget`)

- The widget can be rendered as `frame.render_widget(&scrollbar, area)` by implementing
  `Widget for &ScrollBar`.
- Mouse handling is provided via plain Rust state structs and methods (no Ratatui stateful widget):
  - Mouse event handling must map pointer input to scroll offset updates, which means developers
    need a clear “event in → new offset out” workflow at runtime.
  - A `crossterm` feature provides an adapter for crossterm mouse events (pattern from
    `tui-popup/Cargo.toml:24`), but the core interaction API should be backend-agnostic.

### “Rust friendly” goals

- Separate pure computation from rendering:
  - A small, well-tested “metrics” layer computes thumb size/position and hit testing independent
    of `Buffer`.
  - Rendering is a thin translation of computed “fill levels per cell” into glyphs and styles.

## Non-goals

- Replacing Ratatui’s scrollbar upstream.
- Supporting non-Unicode terminals beyond a configurable fallback glyph set (e.g., ASCII).
- Implementing inertial scrolling / kinetic physics.
- Supporting every event backend; crossterm integration is enough initially, with room for
  adapters.

## Approach

### Core model

Implement a scrollbar in “subcell units”:

- Define `SUBCELL = 8`.
- For a vertical scrollbar with track height `H` cells, the track length is `H * SUBCELL` subcells.
  Horizontal similarly uses track width.
- Compute thumb length in subcells:
  - `thumb_len = max(SUBCELL, floor(track_len * viewport_len / content_len))`, clamped to
    `track_len` (minimum is 1 full cell).
- Compute thumb start in subcells:
  - `max_offset = content_len.saturating_sub(viewport_len)`
  - `thumb_travel = track_len.saturating_sub(thumb_len)`
  - `thumb_start = if max_offset == 0 { 0 } else { thumb_travel * offset / max_offset }`

Then render each cell by determining how many subcells (0..=8) of the thumb occupy that cell, and
mapping that to the appropriate block glyph (vertical: lower/upper fractional blocks; horizontal:
left fractional blocks).

### Mouse interaction model

Provide these interactions (behind `crossterm` feature, plus non-crossterm helpers):

- Wheel up/down (and left/right) adjusts offset by a configurable step.
- Left click:
  - If click lands on thumb: start dragging (store grab offset in subcells).
  - Else: page up/down (or jump-to-click, configurable).
- Drag updates thumb position continuously and converts that to a new offset.

#### Why the popup-style state model may be a poor fit

The popup state stores the last rendered area and uses that to decide whether mouse events apply
(`tui-popup/src/popup_state.rs:6`, `tui-popup/src/popup_state.rs:97`). For a scrollbar:

- Developers typically already know the scrollbar `Rect` in their layout code each frame, so storing
  a previous `Rect` inside the scrollbar interaction state can be redundant.
- A scrollbar’s primary state is the scroll offset (owned by the application, not the widget). Any
  interaction API should make it easy to compute a new offset from an input event and the current
  geometry/content measurements.
- Dragging needs “capture” state (grabbed thumb + grab offset), but that state is logically separate
  from rendering and should be minimal.

#### Proposed interaction API (tied to runtime developer needs)

At runtime, developers need to:

1. Store the scroll offset (and usually content + viewport lengths).
2. Render the scrollbar using the current model.
3. Feed input events to the scrollbar and update their offset.

Design the API around those steps:

- Make offset updates explicit by returning a `ScrollCommand` (e.g., `SetOffset(usize)`,
  `Delta(isize)`) or `Option<usize>` (new offset), rather than mutating internal widget state.
- Keep a small `ScrollBarInteraction` that only tracks drag capture:
  - whether a drag is active
  - the grab offset within the thumb (in subcells)
- Require `area: Rect` (from the caller’s layout) and the current “scroll model” inputs
  (`content_len`, `viewport_len`, `offset`) to handle events; do not require remembering the last
  rendered `Rect`.
- Provide a backend-agnostic input type (e.g., `PointerEvent` + `ScrollWheel`), and gate only the
  crossterm adapter behind `feature = "crossterm"`.

### Integration strategy

- Build `tui-scrollbar` as standalone and validate via unit tests + an example.
- Then integrate into `tui-scrollview` as a follow-up phase by replacing the Ratatui scrollbar
  calls (`tui-scrollview/src/scroll_view.rs:341`).
- If integration goes well, `tui-scrollview` can likely drop `ratatui-widgets` as a dependency.

## Phases

## Phase 1: Create new crate skeleton

### Changes

- [x] Add `tui-scrollbar/` with:
  - [x] `tui-scrollbar/Cargo.toml` aligned with other widget crates.
  - [x] `tui-scrollbar/src/lib.rs` with crate docs (cargo-rdme markers like other crates’ READMEs).
  - [x] `tui-scrollbar/README.md`, `tui-scrollbar/CHANGELOG.md` consistent with existing crate
    layout.
- [x] Add root-facade plumbing:
  - [x] `Cargo.toml` add feature `scrollbar` and optional dependency `tui-scrollbar` (pattern in
    `Cargo.toml:60`).
  - [x] `src/lib.rs` re-export `tui_scrollbar` behind the new feature (pattern in `src/lib.rs:1`).
- [x] CI:
  - [x] Add `tui-scrollbar/Cargo.toml` to the README check job
    (`.github/workflows/check.yml:34`).

### Success criteria

#### Automated verification

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --all-targets --all-features --workspace`
- [x] `cargo test --all-features --workspace`
- [x] `cargo rdme --check --manifest-path tui-scrollbar/Cargo.toml`

#### Manual verification

- [x] `cargo doc --all-features --workspace` renders `tui-scrollbar` docs cleanly.

---

## Phase 2: Implement core metrics + fractional rendering (no mouse yet)

### Changes

- [x] Add a pure computation module (e.g., `tui-scrollbar/src/metrics.rs`):
  - [x] `ScrollMetrics::new(content_len, viewport_len, offset, track_cells)`
  - [x] Conversions: offset ↔ thumb subcell position
  - [x] Hit testing for thumb vs track (in track coordinates)
- [x] Add rendering module (e.g., `tui-scrollbar/src/scrollbar.rs`):
  - [x] `pub struct ScrollBar { orientation, content_len, viewport_len, offset, style, glyph_set,
    … }`
  - [x] `impl Widget for &ScrollBar` renders vertical and horizontal bars.
  - [x] `GlyphSet` supports:
    - [x] Unicode fractional blocks (default)
    - [ ] A coarse fallback (optional)
- [x] Unit tests:
  - [x] Snapshot-like tests using `Buffer::with_lines` similar to `tui-scrollview`’s tests
    (`tui-scrollview/src/scroll_view.rs:404`), but focused on fractional thumb shapes and
    proportional sizing.

### Success criteria

#### Automated verification

- [x] `cargo test -p tui-scrollbar --all-features -- --nocapture`
- [x] `cargo clippy --all-targets --all-features --workspace`

#### Manual verification

- [x] Add and run a minimal example rendering several scrollbars with varying content, viewport,
  and offset to visually confirm fractional glyph behavior.

---

## Lessons learned (Phase 2)

- The legacy glyph set needs mixed sources: standard blocks for lower/left partials and Symbols for
  Legacy Computing Supplement for upper/right partials. Final default glyphs:
  - `vertical_upper`: `['▔','🮂','🮃','▀','🮄','🮅','🮆','█']`
  - `horizontal_right`: `['▕','🮇','🮈','▐','🮉','🮊','🮋','█']`
- The example is most useful when it juxtaposes all 1/8 steps:
  - Left half: 34 horizontal bars (0–16 from the start, 0–16 from the end), labels inline.
  - Right half: 34 vertical bars left-to-right with the same 0–16 + end-cap sequence.
  - Labels are reduced to `index % 8` for compactness; bars truncate when space is limited.
  - Right column uses 2-cell bars when wide enough, falls back to 1-cell bars in narrow layouts.
  - Left column keeps a minimum width (label + 10 cells) while right column is capped.
- Metrics for the example are derived from renderable bar area (excluding label space):
  - `viewport_len = bar_cells * SUBCELL`
  - `content_len` computed to target a fixed thumb size per axis.
  - Target thumb sizes: horizontal = 6 cells, vertical = 3 cells.
- Reserve fixed width for the right half so all 34 vertical bars fit without scaling:
  - Right column width = `34 * 2` cells; left column uses remaining width.
- API ergonomics:
  - Use `ScrollLengths { content_len, viewport_len }` to avoid ambiguous parameter ordering.
  - `ScrollBar::new` now requires an explicit `ScrollBarOrientation`, with `vertical`/`horizontal`
    constructors for common usage.
  - Remove implicit conversion from lengths to scrollbar to force explicit orientation.
- Documentation and examples now prefer `ScrollBar::vertical`/`horizontal` and a named `lengths`
  variable for readability.
- The demo uses arrow keys to move in subcell steps so the fractional rendering is visible without
  mouse input.
- Input handling types are split into `input.rs`; glyph configuration lives in `glyphs.rs` for
  smaller modules.

## Phase 3: Add mouse support (feature-gated)

### Changes

- Add `features` to `tui-scrollbar/Cargo.toml`:
  - [x] `crossterm = ["dep:crossterm"]` following `tui-popup`’s pattern (`tui-popup/Cargo.toml:21`).
- Add a backend-agnostic interaction API:
  - [x] `ScrollBarInteraction` that tracks drag capture only (no stored `Rect`).
  - [x] `ScrollBar::handle_event(area, event, model, &mut interaction) -> Option<ScrollCommand>`.
- Add `handle_mouse_event` behind `cfg(feature = "crossterm")` as a thin adapter that:
  - [x] Converts `crossterm::event::MouseEvent` into the backend-agnostic event type.
  - [x] Calls the core `handle_event(...)`.
- [x] Add unit tests for hit testing + command generation (pure logic tests; no terminal needed).

### Success criteria

#### Automated verification

- [x] `cargo test -p tui-scrollbar --features crossterm -- --nocapture`

#### Manual verification

- [x] Example app where you can drag the thumb and scroll with the wheel; verify offset updates
  match thumb position.
  - Implemented `scrollbar_mouse` example showing horizontal + vertical bars on the bottom/right.

---

## Phase 4: Arrow endcaps (design + implementation)

### Changes

- [x] Add arrow glyphs to `GlyphSet` for vertical/horizontal start/end.
- [x] Add `ScrollBarArrows` (or similar) to configure arrow placement (`None`, `Start`, `End`, `Both`).
- [x] Add a `ScrollBar::arrows(...)` setter and store arrow configuration on the widget.
- [x] Update rendering to:
  - [x] Render arrows first.
  - [x] Compute an inner track area (area minus arrows) for `ScrollMetrics`.
  - [x] Offset cell indices so thumb/track rendering stays aligned inside the inner track.
- [x] Update hit testing to:
  - [x] Detect pointer events on arrow cells and emit a step command.
  - [x] Map pointer positions in the inner track to `ScrollMetrics`.
- [x] Handle tiny areas:
  - [x] If there is no room for an inner track, render arrows only and skip thumb rendering.
- [x] Document arrow behavior and update examples to show endcaps.
- [x] Update the `scrollbar_mouse` example with a title block and keyboard-driven scrolling for
  smooth captures.
- [x] Add a VHS script and embed the demo GIF in the README.
- [x] Add in-frame instructions for the mouse demo and a title bar with quit hint in the step demo.

### Success criteria

#### Automated verification

- [x] `cargo test -p tui-scrollbar --all-features -- --nocapture`
- [x] `cargo doc --all-features --workspace`

#### Manual verification

- [x] Example output shows arrows at both ends with a correctly sized thumb.
- [x] Arrow clicks step the offset as expected.

---

## Phase 4.1: Release publishing wiring

### Changes

- [x] Add a per-package release-plz config for `tui-scrollbar`.
- [x] Include `tui-scrollbar` in the release-plz workflow matrix.

### Success criteria

#### Automated verification

- [x] `release-plz` workflow picks up `tui-scrollbar` in the matrix.

#### Manual verification

- [ ] Confirm release-pr creation in CI.

---

## Phase 4.2: Module cleanup

### Changes

- [x] Split glyph configuration into `tui-scrollbar/src/glyphs.rs`.
- [x] Split input and interaction types into `tui-scrollbar/src/input.rs`.
- [x] Simplify `Widget::render` with focused helper methods.

### Success criteria

#### Automated verification

- [x] `cargo check --all-targets --all-features --workspace`

#### Manual verification

- [ ] Confirm module layout feels easier to navigate.

---

## Phase 5: Integrate into tui-scrollview (optional but recommended)

### Changes

- Replace Ratatui scrollbar usage in `tui-scrollview`:
  - Remove `ratatui_widgets::scrollbar` usage (`tui-scrollview/src/scroll_view.rs:4`).
  - Replace `render_vertical_scrollbar` / `render_horizontal_scrollbar` implementations
    (`tui-scrollview/src/scroll_view.rs:341`) to render `tui_scrollbar::ScrollBar` instead.
- Update `tui-scrollview` tests that assert old glyphs (`tui-scrollview/src/scroll_view.rs:404`)
  to assert the new visuals, including fractional cases.
- Consider removing the `ratatui-widgets` dependency from `tui-scrollview/Cargo.toml` if no longer
  needed.

### Success criteria

#### Automated verification

- [ ] `cargo test -p tui-scrollview --all-features -- --nocapture`
- [ ] `cargo test --all-features --workspace`

#### Manual verification

- [ ] Run existing `tui-scrollview` examples and confirm scrollbars look improved and remain
  proportional.

## Testing and validation

- Unit tests (tui-scrollbar):
  - Metrics correctness across edge cases (zero sizes, huge content, minimal area).
  - Rendering tests for proportionality and fractional blocks.
  - Mouse hit testing and drag mapping (feature `crossterm`).
- Integration tests (tui-scrollview, if Phase 4 is done):
  - Re-run/extend existing buffer-based assertions.
- Commands:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features --workspace`
  - `cargo test --all-features --workspace`
  - `cargo doc --all-features --workspace`

## Risks and mitigations

- Unicode glyph rendering differs by font/terminal.
  - Mitigation: allow `GlyphSet` selection; document that fractional blocks require a
    Unicode-capable font.
- Off-by-one errors in proportional mapping (especially at extremes).
  - Mitigation: centralize math in `metrics.rs`, clamp aggressively, and add targeted tests for
    “top”, “bottom”, and “last pixel” behavior.
- Mouse behavior expectations vary (jump-to-click vs page-to-click).
  - Mitigation: make click behavior configurable and document defaults.
- Arrow cells could shift thumb positions unexpectedly.
  - Mitigation: compute metrics using the inner track area and add tests for offsets 0 and max.
- Arrow click behavior could conflict with track click behavior.
  - Mitigation: treat arrow cells as a distinct hit region before track hit testing.

## References

- Workspace structure and feature gating: `Cargo.toml:1`, `Cargo.toml:60`
- Current scrollbar usage in scrollview: `tui-scrollview/src/scroll_view.rs:1`,
  `tui-scrollview/src/scroll_view.rs:341`
- Current scrollbar glyph expectations: `tui-scrollview/src/scroll_view.rs:404`
- Mouse handling pattern (feature-gated crossterm + drag state): `tui-popup/Cargo.toml:21`,
  `tui-popup/src/popup_state.rs:6`, `tui-popup/src/popup_state.rs:97`
- README CI checks: `.github/workflows/check.yml:34`
- Glyph selection and rendering internals: `tui-scrollbar/src/scrollbar.rs:206`,
  `tui-scrollbar/src/scrollbar.rs:622`, `tui-scrollbar/src/scrollbar.rs:816`
- Metrics track sizing: `tui-scrollbar/src/metrics.rs:94`
- Glyph configuration module: `tui-scrollbar/src/glyphs.rs`
- Input and interaction module: `tui-scrollbar/src/input.rs`
- Release automation config: `.github/release-plz/tui-scrollbar.toml`,
  `.github/workflows/release-plz.yml`
- Demo capture: `tui-scrollbar/examples/scrollbar_mouse.rs`,
  `tui-scrollbar/examples/scrollbar_mouse.tape`
