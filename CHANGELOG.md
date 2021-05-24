# Changelog

Notable library changes are documented here in a format based on
[Keep a Changelog](https://keepachangelog.com/). We generally follow
[Semantic Versioning](https://semver.org).

## Unreleased

### Changed

- Respect Unicode identifiers in
  [name sanitization](https://github.com/evolutics/iftree#name-sanitization).
  If you only use ASCII file paths, then this change has no effect. Essentially,
  non-ASCII characters that are valid in identifiers (from Rust 1.53.0) are
  preserved instead of replaced by an underscore `"_"`.

## 0.1.1 – 2021-05-14

### Fixed

- Activate traits for `syn::Path` missing during build of test project.

## 0.1.0 – 2021-05-14

Initial development release.
