# EguiStruct

[![crates.io](https://img.shields.io/crates/v/egui_struct.svg)](https://crates.io/crates/egui_struct)
[![Documentation](https://docs.rs/egui_struct/badge.svg)](https://docs.rs/egui_struct)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/PingPongun/egui_struct/blob/master/LICENSE-MIT)
[![APACHE 2.0](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/PingPongun/egui_struct/blob/master/LICENSE-APACHE)

EguiStruct is a rust derive macro that creates egui UI's from arbitrary structs and enums.
This is useful for generating data bindings that can be modified and displayed in an [egui](https://github.com/emilk/egui) ui.

Crate idea is similar to crates [egui-probe](https://github.com/zakarumych/egui-probe), [enum2egui](https://github.com/matthewjberger/enum2egui), [egui_inspect](https://github.com/Meisterlama/egui_inspect) and [egui-controls](https://github.com/aalekhpatel07/egui-controls), but has some nice unique features:

## EguiStruct vs similar crates

|                                   | EguiStruct                                                                   | egui-probe                                        | enum2egui             | egui_inspect                 | egui-controls                     |
| :-------------------------------- | :--------------------------------------------------------------------------- | ------------------------------------------------- | :-------------------- | :--------------------------- | :-------------------------------- |
| egui version[[1]](#ref1)          | 0.28 (0.23-0.28)                                                             | 0.27/0.28                                         | 0.23/0.24.1/0.26/0.28 | 0.20                         | N/A                               |
| Adheres to SemVer                 | ✅                                                                            | ✅                                                 | ❌                     | ✅                            | ✅                                 |
| Layout[[2]](#ref2)                | [ExGrid](https://crates.io/crates/exgrid)                                    | Grid                                              | Group/nested          | Nested                       | Grid                              |
| i18n support                      | ✅ (rust-i18n[[3]](#ref3))                                                    | ❌                                                 | ❌                     | ❌                            | ❌                                 |
| Field description                 | ✅ on hover hint (from attribute)                                             | ❌                                                 | ❌                     | ❌                            | ✅ third column (from doc comment) |
| Rename field/variant              | ✅                                                                            | ✅                                                 | ✅                     | ❌                            | ❌                                 |
| Mass name case conversion         | ✅                                                                            | ✅                                                 | ❌                     | ❌                            | ❌                                 |
| Callback on-change                | ✅                                                                            | ❌                                                 | ❌                     | ❌                            | ❌                                 |
| Reset button                      | ✅                                                                            | ❌                                                 | ❌                     | ❌                            | ❌                                 |
| Skip field                        | ✅                                                                            | ✅                                                 | ✅                     | ✅                            | ❌                                 |
|                                   |                                                                              |                                                   |                       |                              |                                   |
| Numerics & strings support        | ✅                                                                            | ✅                                                 | ✅                     | ✅                            | ✅                                 |
| Vec support                       | ✅                                                                            | ✅ std, smallvec1/2                                | ✅                     | ✅                            | ❌                                 |
| Other support                     | ✅ bool, Option, [T], [T;N]\(#TODO), tuple(#TODO)                             | ✅ bool, Option, [T;N], some of egui types         | ✅ bool, Option, tuple | ✅ bool, [T;N]                | ❌                                 |
| HashSet support                   | ✅ std, indexmap                                                              | ❌                                                 | ❌                     | ❌                            | ❌                                 |
| HashMap support                   | ✅ std, indexmap                                                              | ✅ std, hashbrown                                  | ✅ std, hashbrown      | ❌                            | ❌                                 |
|                                   |                                                                              |                                                   |                       |                              |                                   |
| Map field/override impl           | ✅                                                                            | ✅                                                 | ❌                     | ✅                            | ❌                                 |
| Struct derive                     | ✅                                                                            | ✅                                                 | ✅                     | ✅                            | ✅                                 |
| Enum derive                       | ✅                                                                            | ✅                                                 | ✅                     | ❌                            | ❌                                 |
| Custom types in derive            | ✅                                                                            | ✅                                                 | ✅                     | ✅                            | ❌                                 |
| Support foreign types[[4]](#ref4) | ✅                                                                            | ✅                                                 | ❌                     | ❌                            | ❌                                 |
|                                   |                                                                              |                                                   |                       |                              |                                   |
| Configuration numerics            | ✅ Slider(min,max), Slider(min,max,step), DragValue(min,max), DragValue, List | ✅ DragValue(min,max), DragValue                   | ❌                     | ✅ Slider(min,max), DragValue | ❌                                 |
| Configuration string              | ✅ multi/singleline, List                                                     | ✅ multi/singleline                                | ❌                     | ✅ multi/singleline           | ❌                                 |
| Configuration user types          | ✅                                                                            | ❌                                                 | ❌                     | ❌                            | ❌                                 |
| Configuration set/vec/map         | ✅ Enable add/remove/modify(both key&value)/reorder/pre-add-modify elements   | ✅ Enable add/remove elements                      | ❌                     | ❌                            | ❌                                 |
| Configuration others              | ❌                                                                            | ✅ Color32, bool, Enum(combobox or inline buttons) | ❌                     | ❌                            | ❌                                 |
| List/Combobox wrapper             | ✅ [[5]](#ref5)                                                               | ❌                                                 | ❌                     | ❌                            | ❌                                 |

[<a id="ref1">1</a>] : See section `Usage >> egui version` (`EguiStruct` supports all versions of egui through features; other crates support only "newest" one, support for other is by using legacy version)

[<a id="ref2">2</a>] : Layout

Grid benefits:

- Everything is put inside scroll&grid layout (with collapsible rows)
- Gui is less chaotic,
- all values are aligned,
- Gui is compact in width

[ExGrid](https://crates.io/crates/exgrid) benefits:

- Drop-in replacement for egui::Grid
- Benefits of Grid
- Handles collapsible/nested rows + few other things, simplifying implementation/function signatures
- Offers alternative view mode, dedicated for narrow displays (eg. smartphones)

[<a id="ref3">3</a>] : Integrated/with i18n in mind (with [rust-i18n](https://github.com/longbridgeapp/rust-i18n) crate (or if using extractor [modified rust-i18n](https://github.com/PingPongun/rust-i18n.git)))

[<a id="ref4">4</a>] : Foreign types are supported without need to create local wrappers (egui-probe: attribute `with`; egui-struct: attributes `map_*` or `wrapper_*` and `show_*`), this enables also overriding trait implementation

[<a id="ref5">5</a>] : Wrap `T: Clone + ToString + PartialEq` type into `Combobox<T>` and pass through `config` attribute iterator with all possible values → field will be shown as combobox

### Mutable Set/Map features

| type               | add | remove | mut value | mut prior add | reorder | limit length | mut key |
| ------------------ | --- | ------ | --------- | ------------- | ------- | ------------ | ------- |
| `[T]`              | ❌   | ❌      | ✅         | ∅             | ✅       | ∅            | ∅       |
| `Vec<T>`           | ✅   | ✅      | ✅         | ✅             | ✅       | ✅            | ∅       |
| `HashSet<T>`       | ✅   | ✅      | ❌         | ✅             | ❌       | ✅            | ∅       |
| `IndexSet<T>`      | ✅   | ✅      | ✅         | ✅             | ✅       | ✅            | ∅       |
| `HashMap<T>`#TODO  | ✅   | ✅      | ✅         | ✅             | ❌       | ✅            | ❌       |
| `IndexMap<T>`#TODO | ✅   | ✅      | ✅         | ✅             | ✅       | ✅            | ✅       |

Vec/Set/Map implementations have normally quite strict bounds (T: `Any`+`Send`+`Default`+`EguiStructImut`), if your type does not satisfy them, use [`wrappers`](https://docs.rs/egui_struct/latest/egui_struct/wrappers/index.html) module/[`wrapper`](https://docs.rs/egui_struct/latest/egui_struct/prelude/derive.EguiStructMut.html) macro attribute to loosen them.

## Usage

### Basic description

Add `egui_struct` to your `Cargo.toml`:

```toml
egui_struct = "0.5"
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

![demo- grid mode](https://github.com/PingPongun/egui_struct/assets/46752179/5c7281f7-4fba-4fc5-8a4d-de36000155f6)

### egui version

`egui_struct 0.5` by default depends on `egui 0.28`. To use other versions of egui use correct feature in `Cargo.toml`, eg. to make it work with egui 0.25:

```toml
egui_struct = { version = "0.5", default-features = false, features = [ "egui25" ] }
```

OR use `[patch]` section. Currently `egui_struct` supports `egui 0.23-0.28`.

Default egui version feature will be updated to newest egui on semver minor release(0.6).

## License

`egui_struct` is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE).
