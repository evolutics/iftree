# Changelog

All notable changes to this project are documented here in a format based on
[Keep a Changelog](https://keepachangelog.com). The project adheres to
[Semantic Versioning](https://semver.org).

## [Unreleased]

–

## [1.0.6] - 2025-01-05

This release only updates documentation.

## [1.0.5] - 2024-02-25

This release only updates documentation.

## [1.0.4] - 2023-02-01

This release only updates documentation.

## [1.0.3] - 2022-02-05

This release only updates documentation.

## [1.0.2] - 2021-08-29

This release only updates documentation.

## [1.0.1] - 2021-07-05

This release only updates documentation.

## [1.0.0] - 2021-05-29

### Changed

- Respect Unicode identifiers in
  [name sanitization](https://github.com/evolutics/iftree#name-sanitization).
  If you only use ASCII file paths, then this change has no effect. Essentially,
  non-ASCII characters that are valid in identifiers (from Rust 1.53.0) are
  preserved instead of replaced by an underscore `"_"`.

### Fixed

- Fix portability of generated relative paths by always separating components
  with a slash `/`, even on platforms that natively use a backslash `\` instead.

## [0.1.1] - 2021-05-14

### Fixed

- Activate traits for `syn::Path` missing during build of test project.

## [0.1.0] - 2021-05-14

Initial development release.
