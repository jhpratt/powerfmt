# Changelog

All notable changes to the time project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic Versioning].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

---

## 0.2.0 [2023-10-13]

### Added

- `padded_width_of!`
- `Metadata::padded_width`
- `impl SmartDisplay for char`

### Fixed

- The width of integers is now correct. It previous did not account for the sign of the integer.
- Flags for `FormatterOptions` work as expected. Previously, getters were implemented in such a way
  that effective prevented their use.

### Changed

- `SmartDisplay::metadata` returns `Metadata<'_, Self>` instead of `Metadata<'_, Self::Metadata>`.
  This permits `Metadata` to capture the lifetime of the type it is for, rather than relying on the
  user to provide (or infer) the correct lifetime.

  This affects any uses of wrapper types. If you need to return the metadata of an inner type, you
  can call `Metadata::reuse` to change the type. The metadata of both types must be the same, and
  the lifetime of the wrapper type must be at most the lifetime of the inner type. These
  requirements are enforced by the compiler.
- `Metadata::new` requires `self` be provided as a parameter for the same reason.
- The bounds for `Debug`, `Clone`, and `Copy` for `Metadata` are now what is strictly required.

#### Renamed

- `width` → `unpadded_width`
- `width_of` → `padded_width_of`

### Removed

- `Metadata::width_of_many` (`padded_width_of!` instead)

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
