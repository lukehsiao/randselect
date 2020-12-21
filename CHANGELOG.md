# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2020-12-21

### Changed
- [lukehsiao][lh]: Simplified the codebase using structopt, pretty\_env\_logger,
  etc.
- [lukehsiao][lh]: Input and Output directories are now arguments, not flags.
- [lukehsiao][lh]: Added Apache licensing option.

### Removed
- [lukehsiao][lh]: Removed the verbosity flag.
- [lukehsiao][lh]: Removed the no_color flag.

## [0.2.4] - 2018-01-07

### Changed
- [lukehsiao][lh]: Refactor to meet clippy standards.
- [lukehsiao][lh]: Update deps (e.g., `colored`). Consequently remove redundant
  NO_COLOR implementation.

## [0.2.3] - 2018-12-06

### Changed
- [lukehsiao][lh]: Updated for Rust 2018.

## [0.2.2] - 2018-11-27

### Fixed
- [lukehsiao][lh]: Fixed colorization of `++` and `--`.

## [0.2.1] - 2018-11-26

### Fixed
- [lukehsiao][lh]: Fixed no_color behavior.

## [0.2.0] - 2018-11-25

### Added
- [lukehsiao][lh]: Colorize output for clarity.

### Changed
- [lukehsiao][lh]: Make dry-run the default and change --dry-run to --go.

## [0.1.1] - 2018-11-25

### Fixed
- [lukehsiao][lh]: Improve docs.

## [0.1.0] - 2018-11-24
Initial release


[lh]: https://github.com/lukehsiao

[Unreleased]: https://github.com/lukehsiao/randselect/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/lukehsiao/randselect/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/lukehsiao/randselect/releases/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/lukehsiao/randselect/releases/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/lukehsiao/randselect/releases/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/lukehsiao/randselect/releases/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/lukehsiao/randselect/releases/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/lukehsiao/randselect/releases/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/lukehsiao/randselect/releases/tag/v0.1.0
