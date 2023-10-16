# EguiStruct
EguiStruct is a rust derive macro that creates egui UI's from arbitrary structs and enums.
This is useful for generating data bindings that can be modified and displayed in an [egui](https://github.com/emilk/egui) ui. 

Crate idea is similar to crates [enum2egui](https://github.com/matthewjberger/enum2egui) and [egui_inspect](https://github.com/Meisterlama/egui_inspect), but there are some important differences:
### EguiStruct vs either(enum2egui or egui_inspect)
- Everything is put inside scroll&grid layout (with collapsable rows)
  - Gui is less chaotic,
  - all values are aligned,
- integrated/with i18n in mind (with [rust-i18n](https://github.com/longbridgeapp/rust-i18n) crate (or if using extractor [modified rust-i18n](https://github.com/PingPongun/rust-i18n.git)) )
- supports on hover hints
- supports renaming&converting case for fields/variants
- supports callback on-change for fields
### EguiStruct vs enum2egui
- supports some configuration (i.e. currently numerics can be displayed as unbounded DragValue, DragValue.clamp(min,max) or Slider)
- numerously nested structs are compact in width
### EguiStruct vs egui_inspect
- depends on egui v0.23 (egui_inspect on v0.20)
- currently only supports configuration of numerics
- supports also enums
- does not support overriding trait function through attribute 

## Usage

Add this to your `Cargo.toml`:

```toml
egui_struct = { git = "https://github.com/PingPongun/egui_struct.git", branch = "master" }
```

### Example
See ./demo

![obraz](https://github.com/PingPongun/egui_struct/assets/46752179/d095771a-abbe-49bb-92c2-36c20c48a0b8)
