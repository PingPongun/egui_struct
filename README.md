# EguiStruct

[![crates.io](https://img.shields.io/crates/v/egui_struct.svg)](https://crates.io/crates/egui_struct)
[![Documentation](https://docs.rs/egui_struct/badge.svg)](https://docs.rs/egui_struct)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/PingPongun/egui_struct/blob/master/LICENSE)

EguiStruct is a rust derive macro that creates egui UI's from arbitrary structs and enums.
This is useful for generating data bindings that can be modified and displayed in an [egui](https://github.com/emilk/egui) ui.

Crate idea is similar to crates [enum2egui](https://github.com/matthewjberger/enum2egui), [egui_inspect](https://github.com/Meisterlama/egui_inspect) and [egui-controls](https://github.com/aalekhpatel07/egui-controls), but there are some important differences:

## EguiStruct vs similar crates

|                            | EguiStruct                                                                   | enum2egui        | egui_inspect                 | egui-controls                     |
| :------------------------- | :--------------------------------------------------------------------------- | :--------------- | :--------------------------- | :-------------------------------- |
| egui version               | 0.26 (0.21-0.27) ****                                                        | 0.23/0.24.1/0.26 | 0.20                         | N/A                               |
| Layout*                    | Grid                                                                         | Group/nested     | Nested                       | Grid                              |
| i18n support               | ✅ (rust-i18n**)                                                              | ❌                | ❌                            | ❌                                 |
| Field description          | ✅ on hover hint (from attribute)                                             | ❌                | ❌                            | ✅ third column (from doc comment) |
| Rename field/variant       | ✅                                                                            | ✅/❌ (enum only)  | ❌                            | ❌                                 |
| Mass name case conversion  | ✅                                                                            | ❌                | ❌                            | ❌                                 |
| Callback on-change         | ✅                                                                            | ❌                | ❌                            | ❌                                 |
| Reset button               | ✅                                                                            | ❌                | ❌                            | ❌                                 |
| Skip field                 | ✅                                                                            | ✅                | ✅                            | ❌                                 |
|                            |                                                                              |                  |                              |
| Numerics & strings support | ✅                                                                            | ✅                | ✅                            | ✅                                 |
| Vec support                | ✅/❌ (does not support adding/removing elements)                              | ✅                | ✅                            | ❌                                 |
| Other support              | ✅ bool, Option, [T;N]                                                        | ✅ bool, Option   | ✅ bool, [T;N]                | ❌                                 |
| HashMap/Set support        | ✅ std, indexmap                                                              | ✅ std, hashbrown | ❌                            | ❌                                 |
| Map field/override impl    | ✅                                                                            | ❌                | ✅                            | ❌                                 |
| Struct derive              | ✅                                                                            | ✅                | ✅                            | ✅                                 |
| Enum derive                | ✅                                                                            | ✅                | ❌                            | ❌                                 |
| Custom types in derive     | ✅                                                                            | ✅                | ✅                            | ❌                                 |
|                            |                                                                              |                  |                              |
| Configuration numerics     | ✅ Slider(min,max), Slider(min,max,step), DragValue(min,max), DragValue, List | ❌                | ✅ Slider(min,max), DragValue | ❌                                 |
| Configuration string       | ✅ multi/singleline, List                                                     | ❌                | ✅ multi/singleline           | ❌                                 |
| Configuration user types   | ✅                                                                            | ❌                | ❌                            | ❌                                 |
| List/Combobox wrapper      | ✅ ***                                                                        | ❌                | ❌                            | ❌                                 |

\* Everything is put inside scroll&grid layout (with collapsable rows)

- Gui is less chaotic,
- all values are aligned,
- Gui is comact in width

** integrated/with i18n in mind (with [rust-i18n](https://github.com/longbridgeapp/rust-i18n) crate (or if using extractor [modified rust-i18n](https://github.com/PingPongun/rust-i18n.git)))

*** Wrap `T: Clone + ToString + PartialEq` type into `Combobox<T>` and pass through `config` attribute iterator with all possible values → field will be shown as combobox

**** See section `Usage >> egui version`

## Usage

### Basic description

Add `egui_struct` to your `Cargo.toml`:

```toml
egui_struct = "0.4"
```

Add derive macro `EguiStructMut` to struct you want to show (and all nested types):

```Rust
#[derive(EguiStructMut)]
pub struct TupleStruct(u8, u32, String, SubData);
```

then to show data, you only need to call `show_top_mut(..)` on top level struct:

```Rust
egui::CentralPanel::default().show(ctx, |ui| {
  data.show_top_mut(ui, RichText::new("Data").heading(), None);
});
```

### Detailed description

See [docs](https://docs.rs/egui_struct/latest/egui_struct/index.html).

### Example

See ./demo

![obraz](https://github.com/PingPongun/egui_struct/assets/46752179/5c7281f7-4fba-4fc5-8a4d-de36000155f6)

### egui version

`egui_struct 0.4` by default depends on `egui 0.26`. To use other versions of egui use correct feature in `Cargo.toml`, eg. to make it work with egui 0.25:

```toml
egui_struct = { version = "0.4", default-features = false, features = [ "egui25" ] }
```

OR use `[patch]` section.

Default egui version feature will be updated to newest egui on semver minor release(0.5).  

## TODO

- elegant error/invalid input handling & helpful messages (macro)
  - add bounds
- tests
- code cleanup & simplify
- support adding/removing elements for Vec&Hashmap's
- (requires specialization) EguiStructEq/EguiStructClone default impl
