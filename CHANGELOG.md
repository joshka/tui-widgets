# Changelog

All notable changes to this project will be documented in this file.

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
