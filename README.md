# EguiStruct

EguiStruct is a rust derive macro that creates egui UI's from arbitrary structs and enums.
This is useful for generating data bindings that can be modified and displayed in an [egui](https://github.com/emilk/egui) ui.

Crate idea is similar to crates [enum2egui](https://github.com/matthewjberger/enum2egui), [egui_inspect](https://github.com/Meisterlama/egui_inspect) and  [egui-controls](https://github.com/aalekhpatel07/egui-controls), but there are some important differences:

## EguiStruct vs similar crates

### EguiStruct vs either(enum2egui or egui_inspect)

- Everything is put inside scroll&grid layout (with collapsable rows)
  - Gui is less chaotic,
  - all values are aligned,
- integrated/with i18n in mind (with [rust-i18n](https://github.com/longbridgeapp/rust-i18n) crate (or if using extractor [modified rust-i18n](https://github.com/PingPongun/rust-i18n.git)) )
- supports on hover hints
- supports renaming&converting case for fields/variants
- supports callback on-change for fields
- (optionaly) adds button to reset value to some specified value

### EguiStruct vs enum2egui

- supports some configuration (i.e. currently numerics can be displayed as unbounded DragValue, DragValue.clamp(min,max) or Slider)
- numerously nested structs are compact in width
- currently for maps & vec is only possible to edit values, not insert/remove/move elements

### EguiStruct vs egui_inspect

- depends on egui v0.23 (egui_inspect on v0.20)
- currently only supports configuration of numerics
- supports also enums
- does not support overriding trait function through attribute

### EguiStruct vs egui-controls

- egui-controls supports only types that can be passed to Slider or TextEdit, and does not support neither nested types nor enums
- egui-controls does not use traits (it implements single function)
- egui-controls, unless field is marked, generates immutable view
- egui-controls parses doc comments to display field description (next to value), EguiStruct offers similar feature through hint attribute (but visible only on hover)
- EguiStruct supports i18n/ renaming&converting case/ callback on-change/ reset button

## Usage

### Basic description

Add this to your `Cargo.toml`:

```toml
egui_struct = { git = "https://github.com/PingPongun/egui_struct.git", branch = "master" }
```

Add derive macro `EguiStruct` to struct you want to show (and all nested types):

```Rust
#[derive(EguiStruct)]
pub struct TupleStruct(u8, u32, String, SubData);
```

then to show data, you only need to call show_top_mut(..) on top level struct:

```Rust
egui::CentralPanel::default().show(ctx, |ui| {
  data.show_top_mut(ui, RichText::new("Data").heading(), None);
});
```

### Detailed description

Crate consists of 4 traits (EguiStructImut -> EguiStructEq+EguiStructClone -> EguiStruct) and one derive macro (EguiStruct).

- EguiStructImut:
  - for end user ofers one function show_top(..), which displays struct inside scroll area.
  - when implementing (most of bellow has some default impl):
    - show_primitive(..) - ui elements shown in the same line as label
    - show_childs(..) - ui elements related to nested data, that is show inside collapsible rows
    - has_childs(..) && has_primitive(..) - indicates if data has at the moment childs/primitive section
    - const SIMPLE - flag that indicates that data can be shown in the same line as parent (set to true if data is shown as single&simple widget)
    - type ConfigTypeImut - type that will pass some data to cutomise how data is shown, in most cases this will be ()
    - show_collapsing(..) - do not overide this method, use it when implementing show_childs(..) to display single nested element
- EguiStructEq/EguiStructClone are similar to std PartialEq/Clone traits, but they respect eguis(skip). They are necessary to implement EguiStruct.
- EguiStruct is mutable equivalent of EguiStructImut.

Macro EguiStruct can be used on structs&enums to derive all traits ( EguiStructImut & EguiStruct & EguiStructEq & EguiStructClone).
Macro supports attribute "eguis" on either enum/struct, field or variant level:

- enum/struct level:
  - rename_all = "str"- renames all fields/variants to selected case (recognized values: "Upper", "Lower", "Title", "Toggle", "Camel", "Pascal", "UpperCamel", "Snake", "UpperSnake", "ScreamingSnake", "Kebab", "Cobol", "UpperKebab", "Train", "Flat", "UpperFlat", "Alternating", "Sentence")
  - prefix = "str"- add this prefix when generating rust-i18n keys
  - no_imut - do not generate EguiStructImut implementation
  - no_mut - do not generate EguiStruct implementation
  - no_eclone - do not generate EguiStructClone implementation
  - no_eeq - do not generate EguiStructEq implementation
  - resetable = "val" OR resetable(with_expr = Expr) - all fields/variants will be resetable according to provieded value (val: "not_resetable", "field_default", "struct_default", "follow_arg"(use value passed on runtime through reset2 arg))

- variant level:
  - rename ="str"- Name of the field to be displayed on UI labels or variantName in i18n key
  - skip - Don't generate code for the given variant
  - hint ="str" - add on hover hint
  - imut - variant will be shown as immutable
  - i18n ="i18n_key"- normally i18n keys are in format "prefix.enumName.variantName", override this with "i18n_key"
  - resetable- overides enum/struct level resetable

- field level
  - rename, skip, hint, imut, i18n- see variant level
  - resetable- overides enum/struct & variant level resetable
  - on_change- Use function callback (when value has been changed; signature: fn(&field_type) )
  - imconfig- pass format/config object to customise how field is displayed
  - config- same as imconfig but for mutable display

### Example

See ./demo

![obraz](https://github.com/PingPongun/egui_struct/assets/46752179/5c7281f7-4fba-4fc5-8a4d-de36000155f6)
