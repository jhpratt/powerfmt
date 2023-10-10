# Changelog

All notable changes to the time project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic Versioning].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

---

## 0.1.2 [2023-10-10]

### Added

- `SmartDisplay` is implemented for all integer types.

### Changed

- The minimum supported Rust version is now 1.67.0.

## 0.1.1 [2023-10-08]

### Fixed

The docs.rs build has been fixed, as it failed to deploy for v0.1.0.

## 0.1.0 [2023-10-08]

### Added

- `buf::WriteBuffer`
- `ext::FormatterExt`
- `smart_display::SmartDisplay`
- `smart_display::FormatterOptions`
- `smart_display::Metadata`
- `smart_display::ManySmartDisplay`
- `smart_display::delegate`
- `smart_display::private_metadata`
