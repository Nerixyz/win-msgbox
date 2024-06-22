# Changelog

<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
Possible types in order: Added, Changed, Deprecated, Removed, Fixed, Security.

Don't update the links manually, but use cargo-release.
See https://github.com/crate-ci/cargo-release
-->

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- BREAKING: Detached unsafe, raw bindings and safe interface (#8).
  Migrating to the new version involves changing arguments to `title` and `message` from `PCWSTR` to `&str`,
  which, in case of constants, requires removing `w!`. The raw interface has moved to the `raw` module.
  Take a look at the [docs](https://docs.rs/win-msgbox) and [examples](https://github.com/Nerixyz/win-msgbox/tree/master/examples) for more info.

## [0.1.3] - 2024-02-25

### Added

- Added re-exports for `w!`, `Error`, and `Result` (#5).

### Changed

- Updated to `windows-sys` 0.52 (#3)

### Fixed

- Fixed flags not working by adding them in `show` (#4).

## [0.1.2] - 2023-01-05

### Changed

- Updated to `windows-sys` 0.45 (#1).

## [0.1.1] - 2023-01-05

### Fixed

- Fixed an issue in the documentation.

## [0.1.0] - 2023-01-05

Initial release.

<!-- next-url -->

[Unreleased]: https://github.com/Nerixyz/win-msgbox/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/Nerixyz/win-msgbox/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/Nerixyz/win-msgbox/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Nerixyz/win-msgbox/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Nerixyz/win-msgbox/commit/5ad4e0f16d20ab39858de5cd9271b9a8be5b7869
