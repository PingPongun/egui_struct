# EguiStruct

[![crates.io](https://img.shields.io/crates/v/egui_struct.svg)](https://crates.io/crates/egui_struct)
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

Add derive macro `EguiStruct` to struct you want to show (and all nested types):

```Rust
#[derive(EguiStruct)]
pub struct TupleStruct(u8, u32, String, SubData);
```

then to show data, you only need to call `show_top(..)` on top level struct:

```Rust
egui::CentralPanel::default().show(ctx, |ui| {
  data.show_top(ui, RichText::new("Data").heading(), None);
});
```

### Detailed description

Crate consists of 4 traits (`EguiStructImut` -> `EguiStructEq`+`EguiStructClone` -> `EguiStruct`) and one derive macro (`EguiStruct`).

- `EguiStructImut`:
  - for end user ofers one function `show_top_imut(..)`, which displays struct inside scroll area.
  - when implementing (most of bellow has some default impl):
    - `show_primitive_imut(..)` - ui elements shown in the same line as label
    - `show_childs_imut(..)` - ui elements related to nested data, that is show inside collapsible rows
    - `has_childs_imut(..)` && `has_primitive_imut(..)` - indicates if data has at the moment childs/primitive section
    - `const SIMPLE_IMUT` - flag that indicates that data can be shown in the same line as parent (set to true if data is shown as single&simple widget)
    - `type ConfigTypeImut` - type that will pass some data to cutomise how data is shown, in most cases this will be ()
    - `show_collapsing_imut(..)` - do not overide this method, use it when implementing `show_childs_imut(..)` to display single nested element
    - `start_collapsed_imut(self)` - controls if struct is collapsed/uncollapsed at the begining (if "show_childs" is shown by default); eg. Collections (vecs, slices, hashmaps, ..) are initially collapsed if they have more than 16 elements
- `EguiStructEq`/`EguiStructClone` are similar to std `PartialEq`/`Clone` traits, but they respect `eguis(skip)`. They are necessary to implement `EguiStruct` (if type is Clone/PartialEq can be implemented through `impl_eclone!{ty}`/`impl_eeq!{ty}`/`impl_eeqclone!{ty}`).
- `EguiStruct` is mutable equivalent of `EguiStructImut`.

Macro `EguiStruct` can be used on structs&enums to derive all traits ( `EguiStructImut` & `EguiStruct` & `EguiStructEq` & `EguiStructClone`).
Macro supports attribute `eguis` on either enum/struct, field or variant level:

- enum/struct level:
  - `rename_all = "str"`- renames all fields/variants to selected case (recognized values: `"Upper"`, `"Lower"`, `"Title"`, `"Toggle"`, `"Camel"`, `"Pascal"`, `"UpperCamel"`, `"Snake"`, `"UpperSnake"`, `"ScreamingSnake"`, `"Kebab"`, `"Cobol"`, `"UpperKebab"`, `"Train"`, `"Flat"`, `"UpperFlat"`, `"Alternating"`, `"Sentence"`)
  - `prefix = "str"`- add this prefix when generating `rust-i18n` keys
  - `no_imut` - do not generate `EguiStructImut` implementation
  - `no_mut` - do not generate `EguiStruct` implementation
  - `no_eclone` - do not generate `EguiStructClone` implementation
  - `no_eeq` - do not generate `EguiStructEq` implementation
  - `start_collapsed = "Expr"` - sets `start_collapsed()` implementation (should return `bool`; can use `self`)
  - `resetable = "val"` OR `resetable(with_expr = Expr)` - all fields/variants will be resetable according to provieded value (val: `"not_resetable"`, `"field_default"`, `"struct_default"`, `"follow_arg"`(use value passed on runtime through reset2 arg))

- variant level:
  - `rename ="str"`- Name of the field to be displayed on UI labels or variantName in i18n key
  - `skip` - Don't generate code for the given variant
  - `hint ="str"` - add on hover hint
  - `imut` - variant will be shown as immutable
  - `i18n ="i18n_key"`- normally i18n keys are in format "prefix.enumName.variantName", override this with "i18n_key"
  - `resetable`- overides enum/struct level resetable

- field level
  - `rename`, `skip`, `hint`, `imut`, `i18n`- see variant level
  - `resetable`- overides enum/struct & variant level resetable
  - `on_change = "expr"`- Use function (`expr`: closure surounded by `()` OR function path) callback (when value has been changed; signature: `fn(&mut field_type)`)
  - `on_change_struct = "expr"`- Similar to `on_change` but takes whole struct: signature: `fn(&mut self)`
  - `imconfig`- pass format/config object to customise how field is displayed
  - `config`- same as imconfig but for mutable display
  - `start_collapsed = true/false` - field always starts collapsed/uncollapsed (overides fields `start_collapsed()` return)
  - `map_pre`- Expression (closure surounded by `()` OR function path) called to map field to another type before displaying
    - this allows displaying fields that does not implement EguiStruct or overiding how field is shown
    - function shall take `& field_type` or `&mut field_type` AND return either mutable reference or owned value of selected type
    - ! beware, becouse (if `map_pre_ref` is not set) this will make field work only with resetable values: {NonResetable, WithExpr, FieldDefault}
    - defaults to `map_pre_ref` (so if `&mut` is not needed for map, can be left unused)
  - `map_pre_ref`- similar to `map_pre`, but takes immutable reference (signature: `fn(&field_type)->mapped`),
    - used for EguiStructImut, converting default/reset2 and inside eguis_eq (if eeq not specified)
  - `map_post`- Expression (closure surounded by `()` OR function path) called to map mapped field back to field_type after displaying
    - only used if `map_pre` is set AND not for EguiStructImut
    - signature: `fn(&mut field_type, &mapped)` (with `mapped` type matching return from `map_pre`)
    - expresion should assign new value to `&mut field_type`
  - `eeq`- override `eguis_eq` function for field (signature fn(&field_type, &field_type))
    - if either `field_type : EguiStructEq` OR `map_pre_ref` is specified can be unused
  - `eclone`- override `eguis_eclone` function for field (signature fn(&mut field_type, &field_type))
    - if `field_type : EguiStructClone` can be unused

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
