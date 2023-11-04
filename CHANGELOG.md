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

- attribute `on_change` now has signature fn(&mut field_type)

## [0.2.0] - 2023-10-30
