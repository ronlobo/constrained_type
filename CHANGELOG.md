## 0.2.3
- Fix readme

## 0.2.2
- Add some badges
- Update changelog

## 0.2.1
- Add explicit example for how to use this crate
- Improve existing testing examples

## 0.2
- Switch to fancy-regex crate to allow more fancy regex usage in string_like
- Add a doc block test for string_like by implementing a password value object
- Up the minor version number as a breaking change in 0.1.3

## 0.1.3

### Changed
- Use Display trait instead of ToString trait for error representation of int and float
- Fix max string length for string and string_option to character count instead of byte count
- Allow to redact the error value for string_like structs for security reasons, e.g. logging API keys, password, etc by accident
- Upgrade dependencies

## 0.1.2-alpha.3

### Changed
- Consolidate uint/int
- Use num-traits to allow all uint/int primitives
- Fix some docs 

## 0.1.2-alpha.2

### Changed
- Restructure the core type builder functions into their own files
- Remove unnecessary wrapping
- Add some errors and proper docs
- Cleanup licenses, changelog, code of conduct
- Yanking previous Cargo versions (still alpha for now)
- Kudos to all the clean crates providing great examples for learning Rust 