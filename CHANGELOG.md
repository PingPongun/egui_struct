# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- attribute `on_change_struct` similar to `on_change` but takes expr (may use &mut self (whole struct), not only single field reference)

### Fixed

- fix enum combobox indented comparing to other widgets
- fix same key used to keep state of different collapsing

### Changed

- **Breaking**: EguiStruct & EguiStructImut are now independent:
  - macro EguiStruct derives EguiStruct&EguiStructClone&EguiStructEq, macro EguiStructImut derives EguiStructImut
  - EguiStruct attributes (no_imut, no_mut, no_eeq, no_eclone) -> EguiStruct (no_mut, no_eeq, no_eclone), EguiStructImut()
  - all trait items are now "duplicated"
- attribute `on_change` now has signature fn(&mut field_type)
- i18n keys may be different (eg. `Color.Renamed Custom.hint.This is named custom : This is named custom` is now `Color.NamedCustom.__hint : This is named custom`; use modifed rust-18n extractor v2.4+)

## [0.2.0] - 2023-10-30
