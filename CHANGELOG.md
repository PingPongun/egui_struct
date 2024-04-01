# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1] - 2024-04-01

### Added

- added feature with egui 0.27 support

## [0.4.0] - 2024-02-29

### Changed

- support more versions of egui (through features)(supports egui 0.21-0.26; default: 0.26)
- visual updates:
  - show_top(..) ScrollArea, will now auto shrink on both axes
  - show_top(..) Grid, will be stripped as set in style.visuals.stripped

## [0.3.0] - 2023-11-10

### Added

- attributes to map field to another type
- wrapper type `egui_struct::Combobox<T>(T)` that through `config: ConfigType` takes list of possible values
- new config variants for numerics (`SliderStep(..)`, `Combobox(list)`)
- attribute `on_change_struct` similar to `on_change` but takes reference to whole struct
- **Breaking** (if () was passed as ConfigTypeImut): Imutable String & str & numerics are now configurable:
  - `NonSelectable` (aka imut; `egui::Label`),
  - `Selectable` (default imut; aka imutable `egui::TextEdit`),
- **Breaking** (if () was passed as ConfigType): Mutable String is now configurable:
  - `SingleLine` (default mut; aka `egui::TextEdit`),
  - `MultiLine` (mut; aka `egui::TextEdit`),
  - `Combobox(list)` (mut; aka `egui_struct::Combobox`),

### Fixed

- fix enum combobox indented comparing to other widgets
- fix same key used to keep state of different collapsing
- `on_change` now works also on wrapper structs & enum tuple variants with single field
- use absolute paths inside macro

### Changed

- **Breaking**: EguiStruct & EguiStructImut are now independent:
  - macro EguiStruct derives EguiStruct&EguiStructClone&EguiStructEq, macro EguiStructImut derives EguiStructImut
  - EguiStruct attributes (no_imut, no_mut, no_eeq, no_eclone) -> EguiStruct (no_mut, no_eeq, no_eclone), EguiStructImut()
  - all trait items are now "duplicated"
- **Breaking**: more consistent naming:
  - EguiStruct functions/mutable view have now no suffix (`show_top_mut(..)` -> `show_top(..)`)
  - EguiStructImut functions/imutable view have now "_imut" suffix (`show_top(..)` -> `show_top_imut(..)`)
- **Breaking**: ConfigType/ConfigTypeImut has now single lifetime parameter
- **Breaking**: show_collapsing, show_childs take new param: `parent_id`; show_primitive new param: `id`
- **Breaking**: attribute `on_change`: now takes Expression (closure surounded by `()` OR function path) instead of stringified function path
- attribute `on_change` now has signature fn(&mut field_type)
- i18n keys may be different (eg. `Color.Renamed Custom.hint.This is named custom : This is named custom` is now `Color.NamedCustom.__hint : This is named custom`; use modifed rust-18n extractor v2.4+)

## [0.2.0] - 2023-10-30
