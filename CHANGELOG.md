# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.3.0] - 2025-08-16

### Added

- Add support for PowerShell output ([#113](https://github.com/fujiapple852/claptrap/issues/113))
- Add support for `fish` shell output ([#121](https://github.com/fujiapple852/claptrap/issues/121))
- Add support for parsing `PossibleValue` ([#116](https://github.com/fujiapple852/claptrap/issues/116))

## [0.2.0] - 2025-07-20

### Added

- Add support for multi args in `generate` subcommand for
  `zsh` ([#64](https://github.com/fujiapple852/claptrap/issues/64))
- Add short code `-f` for `--spec-format` ([#76](https://github.com/fujiapple852/claptrap/issues/76))

### Changed

- Downgrade MSRV to 1.82.0 (2021 edition) for better
  distro compatibility ([#79](https://github.com/fujiapple852/claptrap/pull/79))

### Fixed

- Invalid output from `script` subcommand when no global
  arguments ([#73](https://github.com/fujiapple852/claptrap/issues/73))
- Fix `command::[color|disable_colored_help]` not respected ([#88](https://github.com/fujiapple852/claptrap/issues/88))
- Fix `--spec-format=auto` not working with spec from stdin ([#75](https://github.com/fujiapple852/claptrap/issues/75))

## [0.1.0] - 2025-07-18

### Added

- Initial implementation

[Unreleased]: https://github.com/fujiapple852/claptrap/compare/0.3.0...master
[0.3.0]: https://github.com/fujiapple852/claptrap/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/fujiapple852/claptrap/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/fujiapple852/claptrap/compare/0.0.0...0.1.0
