# Changelog

All notable changes to this project will be documented in this file.

## [0.2.7] - 2024-08-12

### ⚙️ Miscellaneous Tasks

- Bump ratatui-macros to 0.5.0

## [0.2.6] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui-macros and re-enable multiple versions lint

## [0.2.5] - 2024-08-09

### 🐛 Bug Fixes

- Add missing cfg ([#28](https://github.com/joshka/tui-widgets/pull/28))

### ⚙️ Miscellaneous Tasks

- *(tui-big-text)* Release v0.5.5 ([#25](https://github.com/joshka/tui-widgets/pull/25))
  > ## 🤖 New release
  > * `tui-big-text`: 0.5.4 -> 0.5.5
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > ## `tui-big-text`
  > <blockquote>
  >
  > ## [0.5.5] - 2024-08-09
  >
  > ### 🐛 Bug Fixes
  >
  > - Update to ratatui 0.28
  > ([[#24](https://github.com/joshka/tui-widgets/pull/24)](https://github.com/joshka/tui-widgets/pull/24))
  > > Note that for projects that rely on crossterm, Ratatui 0.28.0 now
  > relies internally on Crossterm 0.28.0.
  > > Ratatui release notes highlights: https://ratatui.rs/highlights/v028/
  > > See https://github.com/ratatui-org/ratatui/issues/1298 for notes about
  > crossterm compatibility
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/MarcoIeni/release-plz/).

## [0.2.4] - 2024-08-06

### Other

- *(deps)* Update crossterm requirement from 0.27.0 to 0.28.1 ([#22](https://github.com/joshka/tui-widgets/pull/22))
  > Updates the requirements on
  > [crossterm](https://github.com/crossterm-rs/crossterm) to permit the
  > latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/crossterm-rs/crossterm/releases">crossterm's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.27.0</h2>
  > <h1>Version 0.27</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add <code>NO_COLOR</code> support (<a
  > href="https://no-color.org/">https://no-color.org/</a>)</li>
  > <li>Add option to force overwrite <code>NO_COLOR</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/802">[#802](https://github.com/joshka/tui-widgets/pull/802)</a>)</li>
  > <li>Add support for scroll left/right events on windows and unix systems
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/788">[#788](https://github.com/joshka/tui-widgets/pull/788)</a>).</li>
  > <li>Add <code>window_size</code> function to fetch pixel width/height of
  > screen for more sophisticated rendering in terminals.</li>
  > <li>Add support for deserializing hex color strings to `Color`` e.g
  > #fffff.</li>
  > </ul>
  > <h2>Changes</h2>
  > <ul>
  > <li>Make the events module an optional feature <code>events</code> (to
  > make crossterm more lightweight) (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/776">[#776](https://github.com/joshka/tui-widgets/pull/776)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Set minimum rustc version to 1.58 (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/798">[#798](https://github.com/joshka/tui-widgets/pull/798)</a>)</li>
  > <li>Change all error types to <code>std::io::Result</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/765">[#765](https://github.com/joshka/tui-widgets/pull/765)</a>)</li>
  > </ul>
  > <p><a href="https://github.com/Gronis"><code>@​Gronis</code></a>, <a
  > href="https://github.com/kevin-vigor"><code>@​kevin-vigor</code></a>, <a
  > href="https://github.com/Wilfred"><code>@​Wilfred</code></a>, <a
  > href="https://github.com/benjajaja"><code>@​benjajaja</code></a>, <a
  > href="https://github.com/blt-r"><code>@​blt-r</code></a>, <a
  > href="https://github.com/Piturnah"><code>@​Piturnah</code></a>, <a
  > href="https://github.com/kdheepak"><code>@​kdheepak</code></a>, <a
  > href="https://github.com/DeathVenom54"><code>@​DeathVenom54</code></a>,
  > <a href="https://github.com/senekor"><code>@​senekor</code></a>, <a
  > href="https://github.com/joseluis"><code>@​joseluis</code></a>, <a
  > href="https://github.com/gibbz00"><code>@​gibbz00</code></a>, <a
  > href="https://github.com/lesleyrs"><code>@​lesleyrs</code></a>, <a
  > href="https://github.com/jhartzell42"><code>@​jhartzell42</code></a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md">crossterm's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h1>Unreleased</h1>
  > <h1>Version 0.28.1</h1>
  > <h2>Fixed 🐛</h2>
  > <ul>
  > <li>Fix broken build on linux when using <code>use-dev-tty</code> with
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/906">[#906](https://github.com/joshka/tui-widgets/pull/906)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Fix desync with mio and signalhook between repo and published crate.
  > (upgrade to mio 1.0)</li>
  > </ul>
  > <h1>Version 0.28</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Capture double click mouse events on windows (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/826">[#826](https://github.com/joshka/tui-widgets/pull/826)</a>)</li>
  > <li>(De)serialize Reset color (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/824">[#824](https://github.com/joshka/tui-widgets/pull/824)</a>)</li>
  > <li>Add functions to allow constructing <code>Attributes</code> in a
  > const context (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/817">[#817](https://github.com/joshka/tui-widgets/pull/817)</a>)</li>
  > <li>Implement <code>Display</code> for <code>KeyCode</code> and
  > <code>KeyModifiers</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/862">[#862](https://github.com/joshka/tui-widgets/pull/862)</a>)</li>
  > </ul>
  > <h2>Changed ⚙️</h2>
  > <ul>
  > <li>Use Rustix by default instead of libc. Libc can be re-enabled if
  > necessary with the <code>libc</code> feature flag (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/892">[#892](https://github.com/joshka/tui-widgets/pull/892)</a>)</li>
  > <li><code>FileDesc</code> now requires a lifetime annotation.</li>
  > <li>Improve available color detection (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/885">[#885](https://github.com/joshka/tui-widgets/pull/885)</a>)</li>
  > <li>Speed up <code>SetColors</code> by ~15-25% (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/879">[#879](https://github.com/joshka/tui-widgets/pull/879)</a>)</li>
  > <li>Remove unsafe and unnecessary size argument from
  > <code>FileDesc::read()</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/821">[#821](https://github.com/joshka/tui-widgets/pull/821)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Fix duplicate bit masks for caps lock and num lock (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/863">[#863](https://github.com/joshka/tui-widgets/pull/863)</a>).
  > This breaks serialization of <code>KeyEventState</code></li>
  > </ul>
  > <h1>Version 0.27.1</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add support for (de)serializing <code>Reset</code>
  > <code>Color</code></li>
  > </ul>
  > <h1>Version 0.27</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add <code>NO_COLOR</code> support (<a
  > href="https://no-color.org/">https://no-color.org/</a>)</li>
  > <li>Add option to force overwrite <code>NO_COLOR</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/802">[#802](https://github.com/joshka/tui-widgets/pull/802)</a>)</li>
  > <li>Add support for scroll left/right events on windows and unix systems
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/788">[#788](https://github.com/joshka/tui-widgets/pull/788)</a>).</li>
  > <li>Add <code>window_size</code> function to fetch pixel width/height of
  > screen for more sophisticated rendering in terminals.</li>
  > <li>Add support for deserializing hex color strings to
  > <code>Color</code> e.g #fffff.</li>
  > </ul>
  > <h2>Changed ⚙️</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/crossterm-rs/crossterm/compare/0.27.0...0.27.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.
  >
  > [//]:# (dependabot-automerge-start)
  >
  > [//]:# (dependabot-automerge-end)
  >
  > ---
  >
  > <details>
  > <summary>Dependabot commands and options</summary>
  > <br />
  >
  > You can trigger Dependabot actions by commenting on this PR:
  > - `@dependabot rebase` will rebase this PR
  > - `@dependabot recreate` will recreate this PR, overwriting any edits
  > that have been made to it
  > - `@dependabot merge` will merge this PR after your CI passes on it
  > - `@dependabot squash and merge` will squash and merge this PR after
  > your CI passes on it
  > - `@dependabot cancel merge` will cancel a previously requested merge
  > and block automerging
  > - `@dependabot reopen` will reopen this PR if it is closed
  > - `@dependabot close` will close this PR and stop Dependabot recreating
  > it. You can achieve the same result by closing it manually
  > - `@dependabot show <dependency name> ignore conditions` will show all
  > of the ignore conditions of the specified dependency
  > - `@dependabot ignore this major version` will close this PR and stop
  > Dependabot creating any more for this major version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this minor version` will close this PR and stop
  > Dependabot creating any more for this minor version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this dependency` will close this PR and stop
  > Dependabot creating any more for this dependency (unless you reopen the
  > PR or upgrade to it yourself)
  >
  >
  > </details>
  >
  > ---------

- *(deps)* Update rstest requirement from 0.21.0 to 0.22.0 ([#21](https://github.com/joshka/tui-widgets/pull/21))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.21.0</h2>
  > <p>Use <code>crate-name</code> feature to enable the crate rename
  > support (enabled by default)</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.21.0] 2024/6/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Add feature <code>crate-name</code> enabled by default to opt-in
  > crate rename
  > support. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/258">[#258](https://github.com/joshka/tui-widgets/pull/258)</a></li>
  > </ul>
  > <h2>[0.20.0] 2024/5/30</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[by_ref]</code> attribute to take get a local
  > lifetime for test arguments.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/joshka/tui-widgets/pull/241)</a>
  > for more details. Thanks to
  > <a href="https://github.com/narpfel"><code>@​narpfel</code></a> for
  > suggesting it and useful discussions.</li>
  > <li>Support for import <code>rstest</code> with another name. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/221">[#221](https://github.com/joshka/tui-widgets/pull/221)</a></li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Don't remove Lifetimes from test function if any. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/230">[#230](https://github.com/joshka/tui-widgets/pull/230)</a>
  > <a href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/joshka/tui-widgets/pull/241)</a>
  > for more details.</li>
  > <li><a
  > href="https://doc.rust-lang.org/std/path/struct.PathBuf.html"><code>PathBuf</code></a>
  > does no longer need to be
  > in scope when using <code>#[files]</code> (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/242">[#242](https://github.com/joshka/tui-widgets/pull/242)</a>)</li>
  > <li><code>#[from(now::accept::also::path::for::fixture)]</code> See <a
  > href="https://redirect.github.com/la10736/rstest/issues/246">[#246](https://github.com/joshka/tui-widgets/pull/246)</a>
  > for more details</li>
  > </ul>
  > <h2>[0.19.0] 2024/4/9</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Defined <code>rust-version</code> for each crate (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/227">[#227](https://github.com/joshka/tui-widgets/pull/227)</a>)</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>
  > <p><code>#[once]</code> fixtures now require the returned type to be
  > <a
  > href="https://doc.rust-lang.org/std/marker/trait.Sync.html"><code>Sync</code></a>
  > to prevent UB
  > when tests are executed in parallel. (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/235">[#235](https://github.com/joshka/tui-widgets/pull/235)</a>
  > for more details)</p>
  > </li>
  > <li>
  > <p><code>#[future(awt)]</code> and <code>#[awt]</code> now properly
  > handle mutable (<code>mut</code>) parameters by treating futures as
  > immutable and
  > treating the awaited rebinding as mutable.</p>
  > </li>
  > </ul>
  > <h2>[0.18.2] 2023/8/13</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now <code>#[files]</code> accept also parent folders (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/205">[#205](https://github.com/joshka/tui-widgets/pull/205)</a>
  > for more details).</li>
  > </ul>
  > <h2>[0.18.1] 2023/7/5</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.21.0...v0.21.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.
  >
  > [//]:# (dependabot-automerge-start)
  >
  > [//]:# (dependabot-automerge-end)
  >
  > ---
  >
  > <details>
  > <summary>Dependabot commands and options</summary>
  > <br />
  >
  > You can trigger Dependabot actions by commenting on this PR:
  > - `@dependabot rebase` will rebase this PR
  > - `@dependabot recreate` will recreate this PR, overwriting any edits
  > that have been made to it
  > - `@dependabot merge` will merge this PR after your CI passes on it
  > - `@dependabot squash and merge` will squash and merge this PR after
  > your CI passes on it
  > - `@dependabot cancel merge` will cancel a previously requested merge
  > and block automerging
  > - `@dependabot reopen` will reopen this PR if it is closed
  > - `@dependabot close` will close this PR and stop Dependabot recreating
  > it. You can achieve the same result by closing it manually
  > - `@dependabot show <dependency name> ignore conditions` will show all
  > of the ignore conditions of the specified dependency
  > - `@dependabot ignore this major version` will close this PR and stop
  > Dependabot creating any more for this major version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this minor version` will close this PR and stop
  > Dependabot creating any more for this minor version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this dependency` will close this PR and stop
  > Dependabot creating any more for this dependency (unless you reopen the
  > PR or upgrade to it yourself)
  >
  >
  > </details>

## [0.2.3] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/joshka/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

### ⚙️ Miscellaneous Tasks

- Remove changelog footer ([#19](https://github.com/joshka/tui-widgets/pull/19))
  > wrt https://github.com/joshka/tui-widgets/pull/18/files#r1701302921
  >
  > not working as expected with `release-plz`

## [0.2.2] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: tui-big-text

## [0.2.1] - 2024-07-25

### 📚 Documentation

- Update readme / lib.rs links

### ⚙️ Miscellaneous Tasks

- Remove anyhow dependency
  > Replaced with color_eyre generally

- Update bacon config

- Update READMEs and licensing info

## [0.2.0] - 2024-07-25

### 🚀 Features

- *(tui-big-text)* Add alignment helper methods
  > Adds helper methods to the `BigTextBuilder` struct to set the alignment
  > of the text. This makes it simpler to set the alignment of the text.
  >
  > ```rust
  > let left = BigText::builder()
  >     .left_aligned()
  >     .lines(vec!["Left".white().into()])
  >     .build()?;
  >
  > let right = BigText::builder()
  >     .right_aligned()
  >     .lines(vec!["Right".green().into()])
  >     .build()?;
  >
  > let centered = BigText::builder()
  >     .centered()
  >     .lines(vec!["Centered".red().into()])
  >     .build()?;
  > ```

- *(tui-big-text)* [**breaking**] Make `BigText` builder infallible ([#14](https://github.com/joshka/tui-widgets/pull/14))
  > BigTextBuilder.build() no longer returns a Result. Instead it returns
  > the BigText widget directly. This change is made to simplify rendering
  > code which often otherwise doesn't have any error conditions.
  >
  > This also makes the fields on BigText public (marked as non-exhaustive)
  >
  > BREAKING CHANGE:BigTextBuilder.build() no longer returns a Result.
  >
  > Remove the `?` / `expect` / `unwrap` calls code which calls the build
  > method.
  >
  > ```diff
  >  let big_text = BigText::builder()
  >      .lines(vec![Line::from("SingleLine")])
  > -    .build()?;
  > +    .build();
  > ```

### 📚 Documentation

- Fixup readme

- Simplify tui-big-text examples

### ⚙️ Miscellaneous Tasks

- Include commit body in changelog

## [0.1.5] - 2024-07-25

### Other

- Add tui-popup to widgets

## [0.1.4] - 2024-07-24

### 🐛 Bug Fixes

- Remove cargo.lock file
- Delete and backspace behavior for multi-byte characters ([#57](https://github.com/joshka/tui-widgets/pull/57))
- Fixup tui-prompts version to match crates.io

### ⚙️ Miscellaneous Tasks

- Various fixes / clippy lints ([#6](https://github.com/joshka/tui-widgets/pull/6))

## [0.1.3](https://github.com/joshka/tui-widgets/compare/tui-widgets-v0.1.2...tui-widgets-v0.1.3) - 2024-07-24

### Fixed
- *(deps)* update minimal version for futures

### Other
- add workflows and dependabot settings
- Move to tui-widgets repository
