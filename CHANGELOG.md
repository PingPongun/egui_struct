# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## TODO [0.6.0] - Planning ( non exhaustive)

- revisit Config/Reset values passing down data hierarchy

## TODO [0.5.1] - Planning ( non exhaustive)

- Added content preview (eg. Vec displays first few elements as immutable in its primitive)
- Support `Tuples`, `[T,N]`
- Better `EguiStructEq` for `HashSet`

## [0.5.0] - Major API update- Unreleased ( changelog non exhaustive)

### Common

- Layout switched from `egui::Grid` to `exgrid::ExGrid` (to force grid layout always use `eguis_mut().view_mode(egui_struct::exgrid::GridMode::Traditional)`, otherwise layout might switch on narrow windows)
- [Breaking] Default egui bumped to v0.28 (was v0.26)
- [Breaking] Minimal egui bumped to v0.23 (was v0.21)
- [Breaking] Library has been grouped into `prelude` (macro generation & showing), `config` (configuration structs), `wrappers`(Wrappers to provide alternative EguiStructMut implementation) and `trait_implementor_set` (used when manually implementing traits)
- [Breaking] New show API: `data.show_top(ui, ..)` -> `data.eguis_mut().show(ui)` (with `prelude::EguiStruct` trait in scope)
- Removed empty first row when label passed in show_top is empty
- `i18n` is no longer default feature

### Macro derive usage (no manual implementations)

- [Breaking] fix attribute name `resetable`->`resettable`
- [Breaking] Sets&Maps impl updated:
  - EguiStructMut impl added for HashSet&IndexSet
  - Sets/Map got configurable features like: add, remove, mut elements, mut prior add, reorder, limit length (See readme for feature support for types)  #TODO
  - Bounds have changed slightly (`Vec<T>` fallbacks to `[T]` if bounds are not met, so no regression, but to get new features use `wrappers` module )
- [Added] macro attributes: eguis(wrapper(dummyC), show_childs_mut=), eguis(wrapper(dummyS), show_primitive_mut=), eguis(wrapper(setMinimal)) #TODO

### Manual trait implementations

- [Breaking] Mutable EguiStruct trait&functions suffixed with mut
  - EguiStruct -> EguiStructMut
  - EguiStructMut::has_{childs, primitive} -> EguiStructMut::has_{$1}_mut
  - EguiStructMut::show_{collapsing, childs, primitive} -> EguiStructMut::has_{$1}_mut
  - EguiStructMut::start_collapsed -> EguiStructMut::start_collapsed_mut
  - EguiStructMut::{SIMPLE, COLUMN_COUNT} -> EguiStructMut::{$1}_MUT
  - EguiStructMut::ConfigType -> EguiStructMut::ConfigTypeMut
- [Breaking] Most functions take `exgrid::ExUi` instead of `egui::Ui` (generally function signature update should suffice)
- [Breaking] EguiStruct{Imut, Mut}::show_\* functions signatures have been simplified (indent_level, \*id are skipped, as they are handled by exgrid)
- [Breaking] `EguiStruct*::show_childs*` now takes `config: &mut Self::ConfigType*` argument #TODO use &
- [Breaking] Sets&Maps impl updated:
  - `[T]::ConfigTypeMut` changed from `()` to `T::ConfigTypeMut`
  - `{Vec<T>, HashSet<T>, IndexSet<T>}::ConfigTypeMut` changed from `()` to `ConfigSetMut<T>`
  - `{HashMap<Q,V>, IndexMap<Q,V>}::ConfigTypeMut` changed from `()` to `ConfigMapMut<Q,V>`
- [Breaking] `Option<T>::ConfigType*` changed from `()` to `T::ConfigType*`
- [Removed] EguiStruct{Mut, Imut}::COLUMN_COUNT_{MUT, IMUT} (it was used only internally, highly unlikely anyone will notice difference)

## [0.4.2] - 2024-07-09

### Added

- added feature with egui 0.28 support

## [0.4.1] - 2024-04-02

### Added

- added feature with egui 0.27 support
- `start_collapsed(..)`/`start_collapsed_imut(..)` functions to `EguiStruct`/`EguiStructImut` traits
- struct-level attribute (`start_collapsed`) that sets `start_collapsed()` implementation
- field-level attribute (`start_collapsed`) that field always starts collapsed/uncollapsed (overrides fields `start_collapsed()` return)

### Changed

- Vecs, slices, hashsets & hashmaps are now by default collapsed if they have more than 16 elements
- Remove detailed description section from README.md- use docs.rs instead

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
