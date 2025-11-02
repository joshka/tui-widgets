# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-11-02

### 🚀 Features

- Add new tui-bar-graph crate ([#63](https://github.com/joshka/tui-widgets/issues/63))
  > ![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)
  >
  > ![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)
  >
  > ```rust
  > use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
  >
  > let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
  > let bar_graph = BarGraph::new(data)
  >     .with_gradient(colorgrad::preset::turbo())
  >     .with_bar_style(BarStyle::Braille)
  >     .with_color_mode(ColorMode::VerticalGradient);
  > frame.render_widget(bar_graph, area);
  > ```

- Calculate area of QRCodeWidget ([#68](https://github.com/joshka/tui-widgets/issues/68))

- *(tui-bar-graph)* [**breaking**] Support boxed gradients ([#66](https://github.com/joshka/tui-widgets/issues/66))
  > This patch adds support for boxed gradients in the `BarGraph` widget.
  > This makes it possible to choose gradients of different types at runtime
  > without having to change the type of the `BarGraph` struct.

- *(tui-bar-graph)* Add Quadrant style ([#80](https://github.com/joshka/tui-widgets/issues/80))
  > This style uses the block drawing 2x2 quadrant characters.
  > In contrast to the braille style, it renders solid rather than dots.
  > In contrast to the solid style, it renders two columns and rows per bar.
  >
  > ![Quadrant Magma](https://vhs.charm.sh/vhs-1rx6XQ9mLiO8qybSBXRGwn.gif)

### 🐛 Bug Fixes

- Broken bar graph test

- Clippy lints ([#81](https://github.com/joshka/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- Use f64:midpoint ([#83](https://github.com/joshka/tui-widgets/issues/83))
  > MSRV is now 1.87

- More clippy lints ([#84](https://github.com/joshka/tui-widgets/issues/84))

- *(tui-prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/joshka/tui-widgets/issues/93)) ([#94](https://github.com/joshka/tui-widgets/issues/94))

### 🚜 Refactor

- Simplify BarGraph rendering logic

- Simplify color / gradient handling logic

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- Remove needless lifetimes ([#60](https://github.com/joshka/tui-widgets/issues/60))

- Fix git-cliff config ([#61](https://github.com/joshka/tui-widgets/issues/61))

- Prepare tui-bar-graph 0.1.1

- Remove leftover github workflow files ([#73](https://github.com/joshka/tui-widgets/issues/73))

- Commit cargo.lock file

- *(tui-big-text)* Support disabling crossterm ([#70](https://github.com/joshka/tui-widgets/issues/70))

- Use semver compatible dependency versions ([#77](https://github.com/joshka/tui-widgets/issues/77))
  > Use 0.x and x.y instead of 0.x.y and x.y.z for deps to reduce
  > incompatibilities

- *(deps)* Use less specific versions of color-eyre and clap ([#82](https://github.com/joshka/tui-widgets/issues/82))

### 🛡️ Security

- *(deps)* Bump derive_setters from 0.1.7 to 0.1.8 ([#86](https://github.com/joshka/tui-widgets/issues/86))
  > Bumps [derive_setters](https://github.com/Lymia/derive_setters) from
  > 0.1.7 to 0.1.8.
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/Lymia/derive_setters/commits/v0.1.8">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=derive_setters&package-manager=cargo&previous-version=0.1.7&new-version=0.1.8)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.40 to 4.5.41 ([#87](https://github.com/joshka/tui-widgets/issues/87))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.40 to 4.5.41.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.41] - 2025-07-09</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Styles::context</code> and
  > <code>Styles::context_value</code> to customize the styling of
  > <code>[default: value]</code> like notes in the <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/92fcd83b7687a16005f91465ad64ca647929e76f"><code>92fcd83</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/aca91b99c1f934c1f6b29924bb052e2c51854d05"><code>aca91b9</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/8434510cee78d9591277c187c128c6ca7db8acc1"><code>8434510</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5869">#5869</a>
  > from tw4452852/patch-1</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/33b1fc304ec6f551e0f2b082eafe1b6f44212179"><code>33b1fc3</code></a>
  > fix(complete): Fix env leakage in elvish dynamic completion</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/e5f1f4884c48fd472529baa253c6384929f2ac0d"><code>e5f1f48</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/9466a552fbf938f7969245f5bac99c38ea446e9b"><code>9466a55</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/d74b79351212ad10eb89b9f842e678b8b2fdbee9"><code>d74b793</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5865">#5865</a>
  > from gifnksm/nushell-completion-value-types</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/ecbc775d3b4d8874786738fa4f342e6796446ff0"><code>ecbc775</code></a>
  > fix(nu): Set argument type based on <code>ValueHint</code></li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/6784054536a18549d90221ecd300084f02ca6386"><code>6784054</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5857">#5857</a>
  > from epage/empty</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/cca5f32b3a9dc0982fbc63e856a49ad3c7688b68"><code>cca5f32</code></a>
  > test(complete): Show empty option-value behavior</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.40...clap_complete-v4.5.41">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.40&new-version=4.5.41)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump strum from 0.27.1 to 0.27.2 ([#89](https://github.com/joshka/tui-widgets/issues/89))
  > Bumps [strum](https://github.com/Peternator7/strum) from 0.27.1 to
  > 0.27.2.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/Peternator7/strum/releases">strum's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v0.27.2</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Adding support for doc comments on <code>EnumDiscriminants</code>
  > generated type… by <a
  > href="https://github.com/linclelinkpart5"><code>@​linclelinkpart5</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/141">Peternator7/strum#141</a></li>
  > <li>Drop needless <code>rustversion</code> dependency by <a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">Peternator7/strum#446</a></li>
  > <li>Upgrade <code>phf</code> to v0.12 by <a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/448">Peternator7/strum#448</a></li>
  > <li>allow discriminants on empty enum by <a
  > href="https://github.com/crop2000"><code>@​crop2000</code></a> in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">Peternator7/strum#435</a></li>
  > <li>Remove broken link to EnumTable docs by <a
  > href="https://github.com/schneems"><code>@​schneems</code></a> in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/427">Peternator7/strum#427</a></li>
  > <li>Change enum table callbacks to FnMut. by <a
  > href="https://github.com/ClaytonKnittel"><code>@​ClaytonKnittel</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">Peternator7/strum#443</a></li>
  > <li>Add <code>#[automatically_derived]</code> to the <code>impl</code>s
  > by <a
  > href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a> in
  > <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></li>
  > <li>Implement a <code>suffix</code> attribute for serialization of enum
  > variants by <a
  > href="https://github.com/amogh-dambal"><code>@​amogh-dambal</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">Peternator7/strum#440</a></li>
  > <li>Expound upon use_phf docs by <a
  > href="https://github.com/Peternator7"><code>@​Peternator7</code></a> in
  > <a
  > href="https://redirect.github.com/Peternator7/strum/pull/449">Peternator7/strum#449</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">Peternator7/strum#446</a></li>
  > <li><a href="https://github.com/crop2000"><code>@​crop2000</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">Peternator7/strum#435</a></li>
  > <li><a href="https://github.com/schneems"><code>@​schneems</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/427">Peternator7/strum#427</a></li>
  > <li><a
  > href="https://github.com/ClaytonKnittel"><code>@​ClaytonKnittel</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">Peternator7/strum#443</a></li>
  > <li><a
  > href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></li>
  > <li><a
  > href="https://github.com/amogh-dambal"><code>@​amogh-dambal</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">Peternator7/strum#440</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2">https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/Peternator7/strum/blob/master/CHANGELOG.md">strum's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>0.27.2</h2>
  > <ul>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/141">#141</a>:
  > Adding support for doc comments on <code>EnumDiscriminants</code>
  > generated type.</p>
  > <ul>
  > <li>The doc comment will be copied from the variant on the type
  > itself.</li>
  > </ul>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">#435</a>:allow
  > discriminants on empty enum.</p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">#443</a>:
  > Change enum table callbacks to FnMut.</p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">#444</a>:
  > Add <code>#[automatically_derived]</code> to the <code>impl</code>s by
  > <a href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></p>
  > <ul>
  > <li>This should make the linter less noisy with warnings in generated
  > code.</li>
  > </ul>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">#440</a>:
  > Implement a <code>suffix</code> attribute for serialization of enum
  > variants.</p>
  > <pre lang="rust"><code>#[derive(strum::Display)]
  > #[strum(suffix=&quot;.json&quot;)]
  > #[strum(serialize_all=&quot;snake_case&quot;)]
  > enum StorageConfiguration {
  >   PostgresProvider,
  >   S3StorageProvider,
  >   AzureStorageProvider,
  > }
  > <p>fn main() {
  > let response = SurveyResponse::Other(&quot;It was good&quot;.into());
  > println!(&quot;Loading configuration from: {}&quot;,
  > StorageConfiguration::PostgresProvider);
  > // prints: Loaded Configuration from: postgres_provider.json
  > }
  > </code></pre></p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">#446</a>:
  > Drop needless <code>rustversion</code> dependency.</p>
  > </li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/38f66210e7ca0bb156f3632dcf24a2548959c379"><code>38f6621</code></a>
  > Expound upon use_phf docs (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/449">#449</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/bb1339026b44773e395913340f4e60972fa5e6a1"><code>bb13390</code></a>
  > Implement a <code>suffix</code> attribute for serialization of enum
  > variants (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/440">#440</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/c9e52bfd2865c8c766e0379f9e7bf57621a104e3"><code>c9e52bf</code></a>
  > Add <code>#[automatically_derived]</code> to the <code>impl</code>s (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/444">#444</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/1b00f899e52f43fa35c4d406c901d33b1e9645e2"><code>1b00f89</code></a>
  > Change enum table callbacks to FnMut. (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/443">#443</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/6e2ca25fba8ebdfa403ada6a2bf2f3b15403b2cf"><code>6e2ca25</code></a>
  > Remove broken link to EnumTable docs (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/427">#427</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/95037811412792c9cd70586598aa88d7f514c0ac"><code>9503781</code></a>
  > allow discriminants on empty enum (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/435">#435</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/8553ba2845989337d88a7170f7f0c419945bf156"><code>8553ba2</code></a>
  > Upgrade <code>phf</code> to v0.12 (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/448">#448</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/2eba5c2a5c0b827317bafcb1f545af67b5ce9110"><code>2eba5c2</code></a>
  > Drop needless <code>rustversion</code> dependency (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/446">#446</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/f301b67d9122b271e0531ab7f167c4585cefa484"><code>f301b67</code></a>
  > Merge branch 'linclelinkpart5-master-2'</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/455b2bf859640dc27442b9d38f58ce8da7e3bd6e"><code>455b2bf</code></a>
  > Merge branch 'master' of <a
  > href="https://github.com/linclelinkpart5/strum">https://github.com/linclelinkpart5/strum</a>
  > into lincle...</li>
  > <li>See full diff in <a
  > href="https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=strum&package-manager=cargo&previous-version=0.27.1&new-version=0.27.2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump rand from 0.9.1 to 0.9.2 ([#88](https://github.com/joshka/tui-widgets/issues/88))
  > Bumps [rand](https://github.com/rust-random/rand) from 0.9.1 to 0.9.2.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/rust-random/rand/blob/master/CHANGELOG.md">rand's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.9.2 — 2025-07-20]</h2>
  > <h3>Deprecated</h3>
  > <ul>
  > <li>Deprecate <code>rand::rngs::mock</code> module and
  > <code>StepRng</code> generator (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1634">#1634</a>)</li>
  > </ul>
  > <h3>Additions</h3>
  > <ul>
  > <li>Enable <code>WeightedIndex&lt;usize&gt;</code> (de)serialization (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1646">#1646</a>)</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/d3dd4157052e5431ce42e157b544968560a68b95"><code>d3dd415</code></a>
  > Prepare rand_core 0.9.2 (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1605">#1605</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/99fabd20e9b39d0af7c2ed6c31dbcad83a1703fd"><code>99fabd2</code></a>
  > Prepare rand_core 0.9.2</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/c7fe1c43b5ba53aacad5fbac94a8b88788564049"><code>c7fe1c4</code></a>
  > rand: re-export <code>rand_core</code> (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1604">#1604</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/db2b1e0bb41b0a1435b9fecaa1b7bdb531183204"><code>db2b1e0</code></a>
  > rand: re-export <code>rand_core</code></li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/ee1d96f9f527dbe6f873c8a5ccf47d60a6b8f7b7"><code>ee1d96f</code></a>
  > rand_core: implement reborrow for <code>UnwrapMut</code> (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1595">#1595</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/e0eb2ee0fcc0b07afb901465f4a8ba7f07128f87"><code>e0eb2ee</code></a>
  > rand_core: implement reborrow for <code>UnwrapMut</code></li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/975f602f5dbbdab0a024e0c5e8b527907426bd8c"><code>975f602</code></a>
  > fixup clippy 1.85 warnings</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/775b05be1b8a4fdef17c6601cd223551fbf67edc"><code>775b05b</code></a>
  > Relax <code>Sized</code> requirements for blanket impls (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1593">#1593</a>)</li>
  > <li>See full diff in <a
  > href="https://github.com/rust-random/rand/compare/rand_core-0.9.1...rand_core-0.9.2">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=rand&package-manager=cargo&previous-version=0.9.1&new-version=0.9.2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tokio from 1.45.1 to 1.46.1 ([#85](https://github.com/joshka/tui-widgets/issues/85))
  > Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.45.1 to 1.46.1.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tokio/releases">tokio's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Tokio v1.46.1</h2>
  > <h1>1.46.1 (July 4th, 2025)</h1>
  > <p>This release fixes incorrect spawn locations in runtime task hooks
  > for tasks spawned using <code>tokio::spawn</code> rather than
  > <code>Runtime::spawn</code>. This issue only effected the spawn location
  > in <code>TaskMeta::spawned_at</code>, and did not effect task locations
  > in Tracing events.</p>
  > <h2>Unstable</h2>
  > <ul>
  > <li>runtime: add <code>TaskMeta::spawn_location</code> tracking where a
  > task was spawned (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7440">tokio-rs/tokio#7440</a></p>
  > <h2>Tokio v1.46.0</h2>
  > <h1>1.46.0 (July 2nd, 2025)</h1>
  > <h3>Fixed</h3>
  > <ul>
  > <li>net: fixed <code>TcpStream::shutdown</code> incorrectly returning an
  > error on macOS (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7290">#7290</a>)</li>
  > </ul>
  > <h2>Added</h2>
  > <ul>
  > <li>sync: <code>mpsc::OwnedPermit::{same_channel,
  > same_channel_as_sender}</code> methods (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7389">#7389</a>)</li>
  > <li>macros: <code>biased</code> option for <code>join!</code> and
  > <code>try_join!</code>, similar to <code>select!</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7307">#7307</a>)</li>
  > <li>net: support for cygwin (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7393">#7393</a>)</li>
  > <li>net: support <code>pope::OpenOptions::read_write</code> on Android
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7426">#7426</a>)</li>
  > <li>net: add <code>Clone</code> implementation for
  > <code>net::unix::SocketAddr</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7422">#7422</a>)</li>
  > </ul>
  > <h2>Changed</h2>
  > <ul>
  > <li>runtime: eliminate unnecessary lfence while operating on
  > <code>queue::Local&lt;T&gt;</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7340">#7340</a>)</li>
  > <li>task: disallow blocking in <code>LocalSet::{poll,drop}</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7372">#7372</a>)</li>
  > </ul>
  > <h2>Unstable</h2>
  > <ul>
  > <li>runtime: add <code>TaskMeta::spawn_location</code> tracking where a
  > task was spawned (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7417">#7417</a>)</li>
  > <li>runtime: removed borrow from <code>LocalOptions</code> parameter to
  > <code>runtime::Builder::build_local</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7346">#7346</a>)</li>
  > </ul>
  > <h2>Documented</h2>
  > <ul>
  > <li>io: clarify behavior of seeking when <code>start_seek</code> is not
  > used (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7366">#7366</a>)</li>
  > <li>io: document cancellation safety of
  > <code>AsyncWriteExt::flush</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7364">#7364</a>)</li>
  > <li>net: fix docs for <code>recv_buffer_size</code> method (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7336">#7336</a>)</li>
  > <li>net: fix broken link of <code>RawFd</code> in <code>TcpSocket</code>
  > docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7416">#7416</a>)</li>
  > <li>net: update <code>AsRawFd</code> doc link to current Rust stdlib
  > location (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7429">#7429</a>)</li>
  > <li>readme: fix double period in reactor description (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7363">#7363</a>)</li>
  > <li>runtime: add doc note that <code>on_*_task_poll</code> is unstable
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7311">#7311</a>)</li>
  > <li>sync: update broadcast docs on allocation failure (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7352">#7352</a>)</li>
  > <li>time: add a missing panic scenario of <code>time::advance</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7394">#7394</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7290">#7290</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7290">tokio-rs/tokio#7290</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7307">#7307</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7307">tokio-rs/tokio#7307</a></p>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/ab3ff69cf2258a8c696b2dca89a2cef4ff114c1c"><code>ab3ff69</code></a>
  > chore: prepare to release v1.46.1 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7444">#7444</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/a0d5b8ab308bbeaa8090d411550d6c887d699096"><code>a0d5b8a</code></a>
  > runtime(unstable): fix task hook spawn locations for
  > <code>tokio::spawn</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/a1ee3ef218894f2441b5719812ab218ae0539c8d"><code>a1ee3ef</code></a>
  > chore: fix some minor typos in the comments (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7442">#7442</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/171cd148a37da40dcbb8b06bf2c67634b2ba1f87"><code>171cd14</code></a>
  > changelog: fix typo in <code>pipe::OpenOptions</code> for 1.46.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7439">#7439</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3f1f268583a16c11560f8e310d5a35e9aa55b547"><code>3f1f268</code></a>
  > chore: prepare Tokio v1.46.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7437">#7437</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3e890cc0171ddb210acdcfec831b7c7bcbb0d2d9"><code>3e890cc</code></a>
  > rt(unstable): add spawn <code>Location</code> to <code>TaskMeta</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7417">#7417</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/69290a64327a017fd9a0cedefaac60c4993c3b54"><code>69290a6</code></a>
  > net: derive <code>Clone</code> for <code>net::unix::SocketAddr</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7422">#7422</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/e2b175848b2cb25e99cd3a0486e506f889379db5"><code>e2b1758</code></a>
  > fuzz: cfg fuzz tests under cfg(test) (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7428">#7428</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/b7a75b5be349aab2cee9b224c0610d7cf4fea73e"><code>b7a75b5</code></a>
  > net: update <code>AsRawFd</code> doc link to current Rust stdlib
  > location (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7429">#7429</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/6b705b3053d2c777e05cb60c758202ff9d4b2e7d"><code>6b705b3</code></a>
  > net: allow <code>pipe::OpenOptions::read_write</code> on Android (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7426">#7426</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tokio/compare/tokio-1.45.1...tokio-1.46.1">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.45.1&new-version=1.46.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.41 to 4.5.43 ([#97](https://github.com/joshka/tui-widgets/issues/97))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.41 to 4.5.43.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.43</h2>
  > <h2>[4.5.43] - 2025-08-06</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> In long help, list Possible Values before defaults,
  > rather than after, for a more consistent look</li>
  > </ul>
  > <h2>v4.5.42</h2>
  > <h2>[4.5.42] - 2025-07-30</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Include subcommand visible long aliases in <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.43] - 2025-08-06</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> In long help, list Possible Values before defaults,
  > rather than after, for a more consistent look</li>
  > </ul>
  > <h2>[4.5.42] - 2025-07-30</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Include subcommand visible long aliases in <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c4105bd90c314ac21dd9e008de8b88ab0175fdf7"><code>c4105bd</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/a029b20be631aab1d3a963872df2158b97f61427"><code>a029b20</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/cf15d48b59cf39cafc3e3797dec293edaf9cf533"><code>cf15d48</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5893">#5893</a>
  > from 8LWXpg/patch-2</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7e54542de972c4af98d3035377dcde83c5a5734e"><code>7e54542</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5892">#5892</a>
  > from 8LWXpg/patch-1</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/6ffc88f8c97be82e71d5d6101c98e1042708ab69"><code>6ffc88f</code></a>
  > fix(complete): Check if help string is empty</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7d8470ed9cf1d5503482938cea62f8f363579f12"><code>7d8470e</code></a>
  > fix(complete): Fix single quote escaping in PowerShell</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/eadcc8f66c128272ea309fed3d53d45b9c700b6f"><code>eadcc8f</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7ce0f7bea34011ca888a762bdd95d2371006c97a"><code>7ce0f7b</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/fea7c5487bb602a9b7151c40069afc6f34bda442"><code>fea7c54</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5888">#5888</a>
  > from epage/tut</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c297ddd56e2601d9d1b0a0ba13a9086e8f3ac43c"><code>c297ddd</code></a>
  > docs(tutorial): Experiment with a flat layout</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.41...clap_complete-v4.5.43">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.41&new-version=4.5.43)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tokio from 1.46.1 to 1.47.1 ([#96](https://github.com/joshka/tui-widgets/issues/96))
  > Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.46.1 to 1.47.1.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tokio/releases">tokio's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Tokio v1.47.1</h2>
  > <h1>1.47.1 (August 1st, 2025)</h1>
  > <h3>Fixed</h3>
  > <ul>
  > <li>process: fix panic from spurious pidfd wakeup (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>)</li>
  > <li>sync: fix broken link of Python <code>asyncio.Event</code> in
  > <code>SetOnce</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7485">tokio-rs/tokio#7485</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7494">tokio-rs/tokio#7494</a></p>
  > <h2>Tokio v1.47.0</h2>
  > <h1>1.47.0 (July 25th, 2025)</h1>
  > <p>This release adds <code>poll_proceed</code> and
  > <code>cooperative</code> to the <code>coop</code> module for
  > cooperative scheduling, adds <code>SetOnce</code> to the
  > <code>sync</code> module which provides
  > similar functionality to [<code>std::sync::OnceLock</code>], and adds a
  > new method
  > <code>sync::Notify::notified_owned()</code> which returns an
  > <code>OwnedNotified</code> without
  > a lifetime parameter.</p>
  > <h2>Added</h2>
  > <ul>
  > <li>coop: add <code>cooperative</code> and <code>poll_proceed</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7405">#7405</a>)</li>
  > <li>sync: add <code>SetOnce</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7418">#7418</a>)</li>
  > <li>sync: add <code>sync::Notify::notified_owned()</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>)</li>
  > </ul>
  > <h2>Changed</h2>
  > <ul>
  > <li>deps: upgrade windows-sys 0.52 → 0.59 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7117">#7117</a>)</li>
  > <li>deps: update to socket2 v0.6 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7443">#7443</a>)</li>
  > <li>sync: improve <code>AtomicWaker::wake</code> performance (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7450">#7450</a>)</li>
  > </ul>
  > <h2>Documented</h2>
  > <ul>
  > <li>metrics: fix listed feature requirements for some metrics (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7449">#7449</a>)</li>
  > <li>runtime: improve safety comments of <code>Readiness&lt;'_&gt;</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7415">#7415</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7405">#7405</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7405">tokio-rs/tokio#7405</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7415">#7415</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7415">tokio-rs/tokio#7415</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7418">#7418</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7418">tokio-rs/tokio#7418</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7449">#7449</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7449">tokio-rs/tokio#7449</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7450">#7450</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7450">tokio-rs/tokio#7450</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7465">tokio-rs/tokio#7465</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/be8ee45b3fc2d107174e586141b1cb12c93e2ddf"><code>be8ee45</code></a>
  > chore: prepare Tokio v1.47.1 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7504">#7504</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/d9b19166cde30b8d4a65f31a94b5ee09d2dd7b8c"><code>d9b1916</code></a>
  > Merge 'tokio-1.43.2' into 'tokio-1.47.x' (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7503">#7503</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/db8edc620fb369f6cc92dd9dcfdd03b832c2b02f"><code>db8edc6</code></a>
  > chore: prepare Tokio v1.43.2 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7502">#7502</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/4730984d66e708b36efe84245cbf15bd483a886f"><code>4730984</code></a>
  > readme: add 1.47 as LTS release (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7497">#7497</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/1979615cbf1cc4b4d296814957394703827362d0"><code>1979615</code></a>
  > process: fix panic from spurious pidfd wakeup (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/f669a609cf1eaa94d2bc135212f57ff913eca898"><code>f669a60</code></a>
  > ci: add lockfile for LTS branch</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/ce41896f8dcbc6249df3279600f45f7a65915cf6"><code>ce41896</code></a>
  > sync: fix broken link of Python <code>asyncio.Event</code> in
  > <code>SetOnce</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/c8ab78a84fff284958dc84b77b5222fecd0f44b2"><code>c8ab78a</code></a>
  > changelog: fix incorrect PR number for 1.47.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7484">#7484</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3911cb8523f190142f61c64b66881c07c0d3e7be"><code>3911cb8</code></a>
  > chore: prepare Tokio v1.47.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7482">#7482</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/d545aa2601e3008ce49c8c0191b0f172ce577452"><code>d545aa2</code></a>
  > sync: add <code>sync::Notify::notified_owned()</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tokio/compare/tokio-1.46.1...tokio-1.47.1">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.46.1&new-version=1.47.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.43 to 4.5.48 ([#103](https://github.com/joshka/tui-widgets/issues/103))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.43 to 4.5.48.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.48</h2>
  > <h2>[4.5.48] - 2025-09-19</h2>
  > <h3>Documentation</h3>
  > <ul>
  > <li>Add a new CLI Concepts document as another way of framing clap</li>
  > <li>Expand the <code>typed_derive</code> cookbook entry</li>
  > </ul>
  > <h2>v4.5.47</h2>
  > <h2>[4.5.47] - 2025-09-02</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Added <code>impl FromArgMatches for ()</code></li>
  > <li>Added <code>impl Args for ()</code></li>
  > <li>Added <code>impl Subcommand for ()</code></li>
  > <li>Added <code>impl FromArgMatches for Infallible</code></li>
  > <li>Added <code>impl Subcommand for Infallible</code></li>
  > </ul>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(derive)</em> Update runtime error text to match
  > <code>clap</code></li>
  > </ul>
  > <h2>v4.5.46</h2>
  > <h2>[4.5.46] - 2025-08-26</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Expose <code>StyledStr::push_str</code></li>
  > </ul>
  > <h2>v4.5.45</h2>
  > <h2>[4.5.45] - 2025-08-12</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(unstable-v5)</em> <code>ValueEnum</code> variants now use the
  > full doc comment, not summary, for <code>PossibleValue::help</code></li>
  > </ul>
  > <h2>v4.5.44</h2>
  > <h2>[4.5.44] - 2025-08-11</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Command::mut_subcommands</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.48] - 2025-09-19</h2>
  > <h3>Documentation</h3>
  > <ul>
  > <li>Add a new CLI Concepts document as another way of framing clap</li>
  > <li>Expand the <code>typed_derive</code> cookbook entry</li>
  > </ul>
  > <h2>[4.5.47] - 2025-09-02</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Added <code>impl FromArgMatches for ()</code></li>
  > <li>Added <code>impl Args for ()</code></li>
  > <li>Added <code>impl Subcommand for ()</code></li>
  > <li>Added <code>impl FromArgMatches for Infallible</code></li>
  > <li>Added <code>impl Subcommand for Infallible</code></li>
  > </ul>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(derive)</em> Update runtime error text to match
  > <code>clap</code></li>
  > </ul>
  > <h2>[4.5.46] - 2025-08-26</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Expose <code>StyledStr::push_str</code></li>
  > </ul>
  > <h2>[4.5.45] - 2025-08-12</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(unstable-v5)</em> <code>ValueEnum</code> variants now use the
  > full doc comment, not summary, for <code>PossibleValue::help</code></li>
  > </ul>
  > <h2>[4.5.44] - 2025-08-11</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Command::mut_subcommands</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c3a1ddc1182fa7cf2cfe6d6dba4f76db83d48178"><code>c3a1ddc</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/4460ff44b375c8d596fb70b848ff401fe12942c0"><code>4460ff4</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/54947a1b4bc70745cd5787fb92a830081c6ed291"><code>54947a1</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5981">#5981</a>
  > from mernen/fix-bash-clap-complete-space</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/fd3f6d271defef2aa2f111555a005689f71f6acb"><code>fd3f6d2</code></a>
  > fix(complete): Restore nospace in bash</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/2f6a1083d94b832af96b791fc934beb043a969cb"><code>2f6a108</code></a>
  > test(complete): Demonstrate current behavior</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f88be5738e33018f3298fabb7b67835eefbc55e0"><code>f88be57</code></a>
  > style: Ensure consistent newlines</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f209bce2203498e743b171b7ac64f0fb9d3ae590"><code>f209bce</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f33ff7f81ab78c227a127fbd2dbd0fed1455a6fb"><code>f33ff7f</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/bf06e6f8f6efc5af03a52c5e4cfea39c682aa500"><code>bf06e6f</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5974">#5974</a>
  > from kryvashek/support-clearing-args-matches</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/5d357ada532d430290c2de14c918833564f12795"><code>5d357ad</code></a>
  > feat(parser): Added ArgMatches::try_clear_id()</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.43...clap_complete-v4.5.48">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.43&new-version=4.5.48)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tracing-subscriber from 0.3.19 to 0.3.20 ([#100](https://github.com/joshka/tui-widgets/issues/100))
  > Bumps [tracing-subscriber](https://github.com/tokio-rs/tracing) from
  > 0.3.19 to 0.3.20.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tracing/releases">tracing-subscriber's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>tracing-subscriber 0.3.20</h2>
  > <p><strong>Security Fix</strong>: ANSI Escape Sequence Injection
  > (CVE-TBD)</p>
  > <h2>Impact</h2>
  > <p>Previous versions of tracing-subscriber were vulnerable to ANSI
  > escape sequence injection attacks. Untrusted user input containing ANSI
  > escape sequences could be injected into terminal output when logged,
  > potentially allowing attackers to:</p>
  > <ul>
  > <li>Manipulate terminal title bars</li>
  > <li>Clear screens or modify terminal display</li>
  > <li>Potentially mislead users through terminal manipulation</li>
  > </ul>
  > <p>In isolation, impact is minimal, however security issues have been
  > found in terminal emulators that enabled an attacker to use ANSI escape
  > sequences via logs to exploit vulnerabilities in the terminal
  > emulator.</p>
  > <h2>Solution</h2>
  > <p>Version 0.3.20 fixes this vulnerability by escaping ANSI control
  > characters in when writing events to destinations that may be printed to
  > the terminal.</p>
  > <h2>Affected Versions</h2>
  > <p>All versions of tracing-subscriber prior to 0.3.20 are affected by
  > this vulnerability.</p>
  > <h2>Recommendations</h2>
  > <p>Immediate Action Required: We recommend upgrading to
  > tracing-subscriber 0.3.20 immediately, especially if your
  > application:</p>
  > <ul>
  > <li>Logs user-provided input (form data, HTTP headers, query parameters,
  > etc.)</li>
  > <li>Runs in environments where terminal output is displayed to
  > users</li>
  > </ul>
  > <h2>Migration</h2>
  > <p>This is a patch release with no breaking API changes. Simply update
  > your Cargo.toml:</p>
  > <pre lang="toml"><code>[dependencies]
  > tracing-subscriber = &quot;0.3.20&quot;
  > </code></pre>
  > <h2>Acknowledgments</h2>
  > <p>We would like to thank <a href="http://github.com/zefr0x">zefr0x</a>
  > who responsibly reported the issue at
  > <code>security@tokio.rs</code>.</p>
  > <p>If you believe you have found a security vulnerability in any
  > tokio-rs project, please email us at <code>security@tokio.rs</code>.</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/4c52ca5266a3920fc5dfeebda2accf15ee7fb278"><code>4c52ca5</code></a>
  > fmt: fix ANSI escape sequence injection vulnerability (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3368">#3368</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/f71cebe41e4c12735b1d19ca804428d4ff7d905d"><code>f71cebe</code></a>
  > subscriber: impl Clone for EnvFilter (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3360">#3360</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/3a1f571102b38bcdca13d59f3c454989d179055d"><code>3a1f571</code></a>
  > Fix CI (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3361">#3361</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/e63ef57f3d686abe3727ddd586eb9af73d6715b7"><code>e63ef57</code></a>
  > chore: prepare tracing-attributes 0.1.30 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3316">#3316</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/6e59a13b1a7bcdd78b8b5a7cbcf70a0b2cdd76f0"><code>6e59a13</code></a>
  > attributes: fix tracing::instrument regression around shadowing (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3311">#3311</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/e4df76127538aa8370d7dee32a6f84bbec6bbf10"><code>e4df761</code></a>
  > tracing: update core to 0.1.34 and attributes to 0.1.29 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3305">#3305</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/643f392ebb73c4fb856f56a78c066c82582dd22c"><code>643f392</code></a>
  > chore: prepare tracing-attributes 0.1.29 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3304">#3304</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/d08e7a6eea1833810ea527e18ea03b08cd402c9d"><code>d08e7a6</code></a>
  > chore: prepare tracing-core 0.1.34 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3302">#3302</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/6e70c571d319a033d5f37c885ccf99aa675a9eac"><code>6e70c57</code></a>
  > tracing-subscriber: count numbers of enters in <code>Timings</code> (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/2944">#2944</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/c01d4fd9def2fb061669a310598095c789ca0a32"><code>c01d4fd</code></a>
  > fix docs and enable CI on <code>main</code> branch (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3295">#3295</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tracing/compare/tracing-subscriber-0.3.19...tracing-subscriber-0.3.20">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tracing-subscriber&package-manager=cargo&previous-version=0.3.19&new-version=0.3.20)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump actions/checkout from 4 to 5 ([#99](https://github.com/joshka/tui-widgets/issues/99))
  > Bumps [actions/checkout](https://github.com/actions/checkout) from 4 to
  > 5.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/actions/checkout/releases">actions/checkout's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v5.0.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Update actions checkout to use node 24 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2226">actions/checkout#2226</a></li>
  > <li>Prepare v5.0.0 release by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2238">actions/checkout#2238</a></li>
  > </ul>
  > <h2>⚠️ Minimum Compatible Runner Version</h2>
  > <p><strong>v2.327.1</strong><br />
  > <a
  > href="https://github.com/actions/runner/releases/tag/v2.327.1">Release
  > Notes</a></p>
  > <p>Make sure your runner is updated to this version or newer to use this
  > release.</p>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4...v5.0.0">https://github.com/actions/checkout/compare/v4...v5.0.0</a></p>
  > <h2>v4.3.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>docs: update README.md by <a
  > href="https://github.com/motss"><code>@​motss</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li>Add internal repos for checking out multiple repositories by <a
  > href="https://github.com/mouismail"><code>@​mouismail</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li>Documentation update - add recommended permissions to Readme by <a
  > href="https://github.com/benwells"><code>@​benwells</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li>Adjust positioning of user email note and permissions heading by <a
  > href="https://github.com/joshmgross"><code>@​joshmgross</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2044">actions/checkout#2044</a></li>
  > <li>Update README.md by <a
  > href="https://github.com/nebuk89"><code>@​nebuk89</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li>Update CODEOWNERS for actions by <a
  > href="https://github.com/TingluoHuang"><code>@​TingluoHuang</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/2224">actions/checkout#2224</a></li>
  > <li>Update package dependencies by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > <li>Prepare release v4.3.0 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2237">actions/checkout#2237</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/motss"><code>@​motss</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li><a href="https://github.com/mouismail"><code>@​mouismail</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li><a href="https://github.com/benwells"><code>@​benwells</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li><a href="https://github.com/nebuk89"><code>@​nebuk89</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li><a href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4...v4.3.0">https://github.com/actions/checkout/compare/v4...v4.3.0</a></p>
  > <h2>v4.2.2</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li><code>url-helper.ts</code> now leverages well-known environment
  > variables by <a href="https://github.com/jww3"><code>@​jww3</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/1941">actions/checkout#1941</a></li>
  > <li>Expand unit test coverage for <code>isGhes</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1946">actions/checkout#1946</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4.2.1...v4.2.2">https://github.com/actions/checkout/compare/v4.2.1...v4.2.2</a></p>
  > <h2>v4.2.1</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Check out other refs/* by commit if provided, fall back to ref by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1924">actions/checkout#1924</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/Jcambass"><code>@​Jcambass</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1919">actions/checkout#1919</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4.2.0...v4.2.1">https://github.com/actions/checkout/compare/v4.2.0...v4.2.1</a></p>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/actions/checkout/blob/main/CHANGELOG.md">actions/checkout's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h1>Changelog</h1>
  > <h2>V5.0.0</h2>
  > <ul>
  > <li>Update actions checkout to use node 24 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2226">actions/checkout#2226</a></li>
  > </ul>
  > <h2>V4.3.0</h2>
  > <ul>
  > <li>docs: update README.md by <a
  > href="https://github.com/motss"><code>@​motss</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li>Add internal repos for checking out multiple repositories by <a
  > href="https://github.com/mouismail"><code>@​mouismail</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li>Documentation update - add recommended permissions to Readme by <a
  > href="https://github.com/benwells"><code>@​benwells</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li>Adjust positioning of user email note and permissions heading by <a
  > href="https://github.com/joshmgross"><code>@​joshmgross</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2044">actions/checkout#2044</a></li>
  > <li>Update README.md by <a
  > href="https://github.com/nebuk89"><code>@​nebuk89</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li>Update CODEOWNERS for actions by <a
  > href="https://github.com/TingluoHuang"><code>@​TingluoHuang</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/2224">actions/checkout#2224</a></li>
  > <li>Update package dependencies by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > </ul>
  > <h2>v4.2.2</h2>
  > <ul>
  > <li><code>url-helper.ts</code> now leverages well-known environment
  > variables by <a href="https://github.com/jww3"><code>@​jww3</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/1941">actions/checkout#1941</a></li>
  > <li>Expand unit test coverage for <code>isGhes</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1946">actions/checkout#1946</a></li>
  > </ul>
  > <h2>v4.2.1</h2>
  > <ul>
  > <li>Check out other refs/* by commit if provided, fall back to ref by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1924">actions/checkout#1924</a></li>
  > </ul>
  > <h2>v4.2.0</h2>
  > <ul>
  > <li>Add Ref and Commit outputs by <a
  > href="https://github.com/lucacome"><code>@​lucacome</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1180">actions/checkout#1180</a></li>
  > <li>Dependency updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a>- <a
  > href="https://redirect.github.com/actions/checkout/pull/1777">actions/checkout#1777</a>,
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1872">actions/checkout#1872</a></li>
  > </ul>
  > <h2>v4.1.7</h2>
  > <ul>
  > <li>Bump the minor-npm-dependencies group across 1 directory with 4
  > updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1739">actions/checkout#1739</a></li>
  > <li>Bump actions/checkout from 3 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1697">actions/checkout#1697</a></li>
  > <li>Check out other refs/* by commit by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1774">actions/checkout#1774</a></li>
  > <li>Pin actions/checkout's own workflows to a known, good, stable
  > version. by <a href="https://github.com/jww3"><code>@​jww3</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1776">actions/checkout#1776</a></li>
  > </ul>
  > <h2>v4.1.6</h2>
  > <ul>
  > <li>Check platform to set archive extension appropriately by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1732">actions/checkout#1732</a></li>
  > </ul>
  > <h2>v4.1.5</h2>
  > <ul>
  > <li>Update NPM dependencies by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1703">actions/checkout#1703</a></li>
  > <li>Bump github/codeql-action from 2 to 3 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1694">actions/checkout#1694</a></li>
  > <li>Bump actions/setup-node from 1 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1696">actions/checkout#1696</a></li>
  > <li>Bump actions/upload-artifact from 2 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1695">actions/checkout#1695</a></li>
  > <li>README: Suggest <code>user.email</code> to be
  > <code>41898282+github-actions[bot]@users.noreply.github.com</code> by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1707">actions/checkout#1707</a></li>
  > </ul>
  > <h2>v4.1.4</h2>
  > <ul>
  > <li>Disable <code>extensions.worktreeConfig</code> when disabling
  > <code>sparse-checkout</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1692">actions/checkout#1692</a></li>
  > <li>Add dependabot config by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1688">actions/checkout#1688</a></li>
  > <li>Bump the minor-actions-dependencies group with 2 updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1693">actions/checkout#1693</a></li>
  > <li>Bump word-wrap from 1.2.3 to 1.2.5 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1643">actions/checkout#1643</a></li>
  > </ul>
  > <h2>v4.1.3</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/actions/checkout/commit/08c6903cd8c0fde910a37f88322edcfb5dd907a8"><code>08c6903</code></a>
  > Prepare v5.0.0 release (<a
  > href="https://redirect.github.com/actions/checkout/issues/2238">#2238</a>)</li>
  > <li><a
  > href="https://github.com/actions/checkout/commit/9f265659d3bb64ab1440b03b12f4d47a24320917"><code>9f26565</code></a>
  > Update actions checkout to use node 24 (<a
  > href="https://redirect.github.com/actions/checkout/issues/2226">#2226</a>)</li>
  > <li>See full diff in <a
  > href="https://github.com/actions/checkout/compare/v4...v5">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/checkout&package-manager=github_actions&previous-version=4&new-version=5)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

### Other

- *(deps)* Update rstest requirement from 0.24.0 to 0.25.0 ([#62](https://github.com/joshka/tui-widgets/issues/62))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.24.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>refactor: use <code>core</code> instead of <code>std</code> by <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/283">la10736/rstest#283</a></li>
  > <li>Fix msrv and complete no_std support by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/285">la10736/rstest#285</a></li>
  > <li>replace futures with futures-util by <a
  > href="https://github.com/mati865"><code>@​mati865</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/288">la10736/rstest#288</a></li>
  > <li>Introduce Context by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/289">la10736/rstest#289</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/rnbguy"><code>@​rnbguy</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/283">la10736/rstest#283</a></li>
  > <li><a href="https://github.com/mati865"><code>@​mati865</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/288">la10736/rstest#288</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/la10736/rstest/compare/v0.23.0...v0.24.0">https://github.com/la10736/rstest/compare/v0.23.0...v0.24.0</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.24.0] 2025/1/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>MSRV to 1.70.0 (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/284">#284</a>
  > thanks to <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a>)</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li><code>#![no_std]</code> support: now you can use <code>rstest</code>
  > also in <code>no_std</code> lib
  > (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/282">#282</a>
  > thanks to <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a>)</li>
  > <li><code>#[context]</code> to have test function name and other useful
  > thighs on
  > the tip of your fingers (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/177">#177</a>)</li>
  > </ul>
  > <h2>[0.23.0] 2024/9/29</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>You can now use environment variables in <code>#[files]</code> with
  > an optional default value (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/277">#277</a>).</li>
  > <li>You can now set a base_dir for <code>#[files]</code> with the
  > <code>$[base_dir = &quot;...&quot;]</code> attribute (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/277">#277</a>).</li>
  > </ul>
  > <h2>[0.22.0] 2024/8/4</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now it's possible destructuring input values both for cases, values
  > and fixtures. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/231">#231</a>
  > for details</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[ignore]</code> attribute to ignore test
  > parameters during fixtures resolution/injection. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/228">#228</a>
  > for details</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Lot of typo in code</li>
  > </ul>
  > <h2>[0.21.0] 2024/6/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Add feature <code>crate-name</code> enabled by default to opt-in
  > crate rename
  > support. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/258">#258</a></li>
  > </ul>
  > <h2>[0.20.0] 2024/5/30</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[by_ref]</code> attribute to take get a local
  > lifetime for test arguments.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/issues/241">#241</a>
  > for more details. Thanks to
  > <a href="https://github.com/narpfel"><code>@​narpfel</code></a> for
  > suggesting it and useful discussions.</li>
  > <li>Support for import <code>rstest</code> with another name. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/221">#221</a></li>
  > </ul>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.24.0...v0.24.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- Added render_stateful_widget method to ScrollView ([#65](https://github.com/joshka/tui-widgets/issues/65))

- Bump msrv to 1.82.0 ([#74](https://github.com/joshka/tui-widgets/issues/74))


## [0.4.1] - 2024-11-23

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: tui-scrollview

## [0.3.1] - 2024-10-20

### 🚀 Features

- *(cards)* Add new tui-cards library for playing cards

### 🐛 Bug Fixes

- Broken links from move to tui-widgets

### Other

- Remove patch from main Cargo.toml file that was pointing at a local path ([#38](https://github.com/joshka/tui-widgets/pull/38))

- *(deps)* Update rstest requirement from 0.22.0 to 0.23.0 ([#41](https://github.com/joshka/tui-widgets/pull/41))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Version 0.22.0</h2>
  > <p>Destructuring input data</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.22.0] 2024/8/4</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now it's possible destructuring input values both for cases, values
  > and fixtures. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/231">[#231](https://github.com/joshka/tui-widgets/pull/231)</a>
  > for details</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[ignore]</code> attribute to ignore test
  > parameters during fixtures resolution/injection. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/228">[#228](https://github.com/joshka/tui-widgets/pull/228)</a>
  > for details</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Lot of typo in code</li>
  > </ul>
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
  > <li><code>#[once]</code> fixtures now require the returned type to be
  > <a
  > href="https://doc.rust-lang.org/std/marker/trait.Sync.html"><code>Sync</code></a>
  > to prevent UB
  > when tests are executed in parallel. (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/235">[#235](https://github.com/joshka/tui-widgets/pull/235)</a></li>
  > </ul>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.22.0...v0.22.0">compare
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

## [0.3.0] - 2024-08-12

Ratatui-0.28.0 compatible release

### ⚙️ Miscellaneous Tasks

- Bump ratatui-macros to 0.5.0
- Bump tui-big-text to 0.6.0
- Bump tui-popup to 0.5.0
- Bump tui-prompts to 0.4.0
- Bump tui-scrollview to 0.4.0

## [0.2.6] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui-macros and re-enable multiple versions lint

## [0.2.5] - 2024-08-09

### 🐛 Bug Fixes

- Add missing cfg ([#28](https://github.com/joshka/tui-widgets/pull/28))

### ⚙️ Miscellaneous Tasks

- *(tui-big-text)* Release v0.5.5 ([#25](https://github.com/joshka/tui-widgets/pull/25))

  > ## 🤖 New release
  >
  > - `tui-big-text`: 0.5.4 -> 0.5.5
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > ## `tui-big-text`
  >
  > <blockquote>
  >
  > ## [0.5.5] - 2024-08-09
  >
  > ### 🐛 Bug Fixes
  >
  > - Update to ratatui 0.28
  > ([[#24](https://github.com/joshka/tui-widgets/pull/24)](<https://github.com/joshka/tui-widgets/pull/24>))
  > > Note that for projects that rely on crossterm, Ratatui 0.28.0 now
  > relies internally on Crossterm 0.28.0.
  > > Ratatui release notes highlights: <https://ratatui.rs/highlights/v028/>
  > > See <https://github.com/ratatui-org/ratatui/issues/1298> for notes about
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
  > wrt <https://github.com/joshka/tui-widgets/pull/18/files#r1701302921>
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
