#![feature(prelude_import)]
#![warn(clippy::all, rust_2018_idioms)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    #![allow(dead_code)]
    use egui::mutex::RwLock;
    use egui::RichText;
    use egui_struct::exgrid::GridMode;
    use egui_struct::prelude::*;
    use rust_i18n::set_locale;
    use std::collections::{HashMap, HashSet};
    use ConfigNum::*;
    enum Language {
        #[default]
        English,
        Polish,
    }
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for Language {
        const SIMPLE_MUT: ::std::primitive::bool = true;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            match self {
                _ => false,
            }
        }
        fn has_primitive_mut(&self) -> ::std::primitive::bool {
            true
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            #![allow(unused)]
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            match self {
                _ => {}
            }
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            #![allow(unused)]
            fn to_text(s: &Language) -> ::std::string::String {
                match s {
                    Language::English => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///English
                            "Language.English",
                        )
                    }
                    Language::Polish => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Polish
                            "Language.Polish",
                        )
                    }
                    _ => "".to_string(),
                }
            }
            let id = ui.id();
            ui.keep_cell_start();
            let mut inner_response = ui.dummy_response();
            let mut response = ::egui::ComboBox::from_id_source((
                    id.clone(),
                    "__EguiStruct_enum_combobox",
                ))
                .wrap(false)
                .selected_text(to_text(self))
                .show_ui(
                    ui,
                    |ui| {
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::English => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///English
                                    "Language.English",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::English;
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Polish => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Polish
                                    "Language.Polish",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Polish;
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                    },
                )
                .response;
            match self {
                Self::English => {}
                Self::Polish => {}
                _ => {}
            }
            match self {
                _ => {}
            }
            response | inner_response
        }
        fn start_collapsed_mut(&self) -> bool {
            false
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for Language {
        fn eguis_clone(&mut self, source: &Self) {
            match source {
                Self::English => {
                    *self = Self::English;
                }
                Self::Polish => {
                    *self = Self::Polish;
                }
                _ => {}
            }
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for Language {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            match self {
                _ => {}
            }
            ret
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Language {
        #[inline]
        fn default() -> Language {
            Self::English
        }
    }
    impl Language {
        fn set_locale(&self) {
            match self {
                Language::English => set_locale("en"),
                Language::Polish => set_locale("pl"),
            }
        }
    }
    #[eguis(start_collapsed = "if let Self::NamedCustom{..} = self {true} else {false}")]
    pub enum Color {
        #[default]
        Red,
        Green,
        Named(String),
        Named2 { name: String },
        #[eguis(resetable(with_expr = ||Color::Custom(255, 13, 17)))]
        Custom(
            #[eguis(
                resetable = "not_resetable",
                map_pre = (|field:&mut u8|field.to_string()),
                map_post = (
                    |field:&mut
                    u8,
                    mapped:String|{use
                    std::str::FromStr;if
                    let
                    Ok(new_val)= u8::from_str(mapped.as_str()){*field = new_val;}}
                )
            )]
            u8,
            u8,
            #[eguis(config = "DragValue(1,111)")]
            u8,
        ),
        #[eguis(skip, rename = "Skipped Custom")]
        SkippedCustom(u8, u8, u8),
        #[eguis(skip)]
        SkippedNamedCustom { red: u8, blue: u8, green: u8, metadata: Metadata },
        #[eguis(rename = "Renamed Custom", hint = "This is named custom")]
        NamedCustom { red: u8, blue: u8, green: u8, metadata: Metadata },
    }
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for Color {
        const SIMPLE_MUT: ::std::primitive::bool = false;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            match self {
                Self::Named(..) => !String::SIMPLE_MUT,
                Self::Named2 { .. } => true,
                Self::Custom(..) => true,
                Self::NamedCustom { .. } => true,
                _ => false,
            }
        }
        fn has_primitive_mut(&self) -> ::std::primitive::bool {
            true
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            #![allow(unused)]
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            #[allow(nonstandard_style)]
            static VARIANT_Custom_DEFAULT_EXPR: ::std::sync::OnceLock<Color> = ::std::sync::OnceLock::new();
            _ = VARIANT_Custom_DEFAULT_EXPR.get_or_init(|| Color::Custom(255, 13, 17));
            match self {
                Self::Named(_field_0) => {
                    response
                        |= _field_0
                            .show_collapsing_mut(
                                ui,
                                "[0]",
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::Named(_field_0) = f {
                                            ::std::option::Option::Some(_field_0)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                }
                Self::Named2 { name } => {
                    response
                        |= name
                            .show_collapsing_mut(
                                ui,
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///name
                                    "Color.Named2.name",
                                ),
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::Named2 { name } = f {
                                            ::std::option::Option::Some(name)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                }
                Self::Custom(_field_0, _field_1, _field_2) => {
                    #[allow(unused_mut)]
                    let mut mapped = (|field: &mut u8| field.to_string())(_field_0);
                    let r = mapped
                        .show_collapsing_mut(
                            ui,
                            "[0]",
                            "",
                            ::std::default::Default::default(),
                            ::std::option::Option::None.map(|x| (x)).as_ref(),
                            None,
                        );
                    response |= r.clone();
                    if r.changed() {
                        (|field: &mut u8, mapped: String| {
                            use std::str::FromStr;
                            if let Ok(new_val) = u8::from_str(mapped.as_str()) {
                                *field = new_val;
                            }
                        })(_field_0, mapped);
                    }
                    {};
                    response
                        |= _field_1
                            .show_collapsing_mut(
                                ui,
                                "[1]",
                                "",
                                ::std::default::Default::default(),
                                if let Self::Custom(_field_0, _field_1, _field_2) = &VARIANT_Custom_DEFAULT_EXPR
                                    .get()
                                    .unwrap()
                                {
                                    ::std::option::Option::Some(_field_1)
                                } else {
                                    ::std::option::Option::None
                                },
                                None,
                            );
                    {};
                    response
                        |= _field_2
                            .show_collapsing_mut(
                                ui,
                                "[2]",
                                "",
                                DragValue(1, 111),
                                if let Self::Custom(_field_0, _field_1, _field_2) = &VARIANT_Custom_DEFAULT_EXPR
                                    .get()
                                    .unwrap()
                                {
                                    ::std::option::Option::Some(_field_2)
                                } else {
                                    ::std::option::Option::None
                                },
                                None,
                            );
                    {};
                }
                Self::NamedCustom { red, blue, green, metadata } => {
                    response
                        |= red
                            .show_collapsing_mut(
                                ui,
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///red
                                    "Color.NamedCustom.red",
                                ),
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::NamedCustom { red, blue, green, metadata } = f {
                                            ::std::option::Option::Some(red)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                    response
                        |= blue
                            .show_collapsing_mut(
                                ui,
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///blue
                                    "Color.NamedCustom.blue",
                                ),
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::NamedCustom { red, blue, green, metadata } = f {
                                            ::std::option::Option::Some(blue)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                    response
                        |= green
                            .show_collapsing_mut(
                                ui,
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///green
                                    "Color.NamedCustom.green",
                                ),
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::NamedCustom { red, blue, green, metadata } = f {
                                            ::std::option::Option::Some(green)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                    response
                        |= metadata
                            .show_collapsing_mut(
                                ui,
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///metadata
                                    "Color.NamedCustom.metadata",
                                ),
                                "",
                                ::std::default::Default::default(),
                                reset2
                                    .and_then(|f| {
                                        if let Self::NamedCustom { red, blue, green, metadata } = f {
                                            ::std::option::Option::Some(metadata)
                                        } else {
                                            ::std::option::Option::None
                                        }
                                    }),
                                None,
                            );
                    {};
                }
                _ => {}
            }
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            #![allow(unused)]
            fn to_text(s: &Color) -> ::std::string::String {
                match s {
                    Color::Red => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Red
                            "Color.Red",
                        )
                    }
                    Color::Green => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Green
                            "Color.Green",
                        )
                    }
                    Color::Named(..) => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Named
                            "Color.Named",
                        )
                    }
                    Color::Named2 { .. } => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Named2
                            "Color.Named2",
                        )
                    }
                    Color::Custom(..) => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Custom
                            "Color.Custom",
                        )
                    }
                    Color::NamedCustom { .. } => {
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Renamed Custom
                            "Color.NamedCustom",
                        )
                    }
                    _ => "".to_string(),
                }
            }
            let id = ui.id();
            ui.keep_cell_start();
            let mut inner_response = ui.dummy_response();
            let mut response = ::egui::ComboBox::from_id_source((
                    id.clone(),
                    "__EguiStruct_enum_combobox",
                ))
                .wrap(false)
                .selected_text(to_text(self))
                .show_ui(
                    ui,
                    |ui| {
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Red => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Red
                                    "Color.Red",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Red;
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Green => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Green
                                    "Color.Green",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Green;
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Named(..) => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Named
                                    "Color.Named",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Named(String::default());
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Named2 { .. } => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Named2
                                    "Color.Named2",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Named2 {
                                name: String::default(),
                            };
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::Custom(..) => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Custom
                                    "Color.Custom",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::Custom(
                                u8::default(),
                                u8::default(),
                                u8::default(),
                            );
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                        let mut tresp = ui
                            .selectable_label(
                                match self {
                                    Self::NamedCustom { .. } => true,
                                    _ => false,
                                },
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///Renamed Custom
                                    "Color.NamedCustom",
                                ),
                            )
                            .on_hover_text(
                                crate::_rust_i18n_translate(
                                    rust_i18n::locale().as_str(),
                                    #[allow(unused_doc_comments)]
                                    #[allow(unused_doc_comments)]
                                    ///This is named custom
                                    "Color.NamedCustom.__hint",
                                ),
                            );
                        if tresp.clicked() {
                            *self = Self::NamedCustom {
                                red: u8::default(),
                                blue: u8::default(),
                                green: u8::default(),
                                metadata: Metadata::default(),
                            };
                            tresp.mark_changed()
                        }
                        inner_response |= tresp;
                    },
                )
                .response;
            match self {
                Self::Red => {}
                Self::Green => {}
                Self::Named(..) => {}
                Self::Named2 { .. } => {}
                Self::Custom(..) => {}
                Self::NamedCustom { .. } => {
                    response = response
                        .on_hover_text(
                            crate::_rust_i18n_translate(
                                rust_i18n::locale().as_str(),
                                #[allow(unused_doc_comments)]
                                #[allow(unused_doc_comments)]
                                ///This is named custom
                                "Color.NamedCustom.__hint",
                            ),
                        );
                }
                _ => {}
            }
            match self {
                Self::Named(_field_0) => {
                    let mut mapped = (_field_0);
                    let r = mapped
                        .show_primitive_mut(ui, ::std::default::Default::default());
                    {};
                    response |= r;
                }
                _ => {}
            }
            response | inner_response
        }
        fn start_collapsed_mut(&self) -> bool {
            if let Self::NamedCustom { .. } = self { true } else { false }
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for Color {
        fn eguis_clone(&mut self, source: &Self) {
            match source {
                Self::Red => {
                    *self = Self::Red;
                }
                Self::Green => {
                    *self = Self::Green;
                }
                Self::Named(_field_0) => {
                    if let Self::Named(_2_field_0) = self {
                        _2_field_0.eguis_clone(_field_0);
                    } else {
                        *self = Self::Named(String::default());
                        if let Self::Named(_2_field_0) = self {
                            _2_field_0.eguis_clone(_field_0);
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                Self::Named2 { name } => {
                    if let Self::Named2 { name: _2_name } = self {
                        _2_name.eguis_clone(name);
                    } else {
                        *self = Self::Named2 {
                            name: String::default(),
                        };
                        if let Self::Named2 { name: _2_name } = self {
                            _2_name.eguis_clone(name);
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                Self::Custom(_field_0, _field_1, _field_2) => {
                    if let Self::Custom(_2_field_0, _2_field_1, _2_field_2) = self {
                        _2_field_0.eguis_clone(_field_0);
                        _2_field_1.eguis_clone(_field_1);
                        _2_field_2.eguis_clone(_field_2);
                    } else {
                        *self = Self::Custom(
                            u8::default(),
                            u8::default(),
                            u8::default(),
                        );
                        if let Self::Custom(_2_field_0, _2_field_1, _2_field_2) = self {
                            _2_field_0.eguis_clone(_field_0);
                            _2_field_1.eguis_clone(_field_1);
                            _2_field_2.eguis_clone(_field_2);
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                Self::NamedCustom { red, blue, green, metadata } => {
                    if let Self::NamedCustom {
                        red: _2_red,
                        blue: _2_blue,
                        green: _2_green,
                        metadata: _2_metadata,
                    } = self {
                        _2_red.eguis_clone(red);
                        _2_blue.eguis_clone(blue);
                        _2_green.eguis_clone(green);
                        _2_metadata.eguis_clone(metadata);
                    } else {
                        *self = Self::NamedCustom {
                            red: u8::default(),
                            blue: u8::default(),
                            green: u8::default(),
                            metadata: Metadata::default(),
                        };
                        if let Self::NamedCustom {
                            red: _2_red,
                            blue: _2_blue,
                            green: _2_green,
                            metadata: _2_metadata,
                        } = self {
                            _2_red.eguis_clone(red);
                            _2_blue.eguis_clone(blue);
                            _2_green.eguis_clone(green);
                            _2_metadata.eguis_clone(metadata);
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => {}
            }
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for Color {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            match self {
                Self::Named(_field_0) => {
                    if let Self::Named(_2_field_0) = rhs {
                        ret &= _field_0.eguis_eq(_2_field_0);
                    } else {
                        ret = false;
                    }
                }
                Self::Named2 { name } => {
                    if let Self::Named2 { name: _2_name } = rhs {
                        ret &= name.eguis_eq(_2_name);
                    } else {
                        ret = false;
                    }
                }
                Self::Custom(_field_0, _field_1, _field_2) => {
                    if let Self::Custom(_2_field_0, _2_field_1, _2_field_2) = rhs {
                        ret &= _field_0.eguis_eq(_2_field_0);
                        ret &= _field_1.eguis_eq(_2_field_1);
                        ret &= _field_2.eguis_eq(_2_field_2);
                    } else {
                        ret = false;
                    }
                }
                Self::NamedCustom { red, blue, green, metadata } => {
                    if let Self::NamedCustom {
                        red: _2_red,
                        blue: _2_blue,
                        green: _2_green,
                        metadata: _2_metadata,
                    } = rhs {
                        ret &= red.eguis_eq(_2_red);
                        ret &= blue.eguis_eq(_2_blue);
                        ret &= green.eguis_eq(_2_green);
                        ret &= metadata.eguis_eq(_2_metadata);
                    } else {
                        ret = false;
                    }
                }
                _ => {}
            }
            ret
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Color {
        #[inline]
        fn default() -> Color {
            Self::Red
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct STATIC_COMBOBOX {
        __private_field: (),
    }
    #[doc(hidden)]
    pub static STATIC_COMBOBOX: STATIC_COMBOBOX = STATIC_COMBOBOX {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for STATIC_COMBOBOX {
        type Target = RwLock<HashSet<String>>;
        fn deref(&self) -> &RwLock<HashSet<String>> {
            #[inline(always)]
            fn __static_ref_initialize() -> RwLock<HashSet<String>> {
                RwLock::new(
                    HashSet::from([
                        "Einar".to_string(),
                        "Olaf".to_string(),
                        "Harald".to_string(),
                    ]),
                )
            }
            #[inline(always)]
            fn __stability() -> &'static RwLock<HashSet<String>> {
                static LAZY: ::lazy_static::lazy::Lazy<RwLock<HashSet<String>>> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for STATIC_COMBOBOX {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    #[eguis(rename_all = "Sentence", resetable = "struct_default")]
    pub struct Data {
        #[eguis(skip)]
        skipped_data: u32,
        #[eguis(resetable(with_expr = "Resetable with expr".to_string()))]
        string: String,
        #[eguis(resetable = "not_resetable")]
        not_resetable_string: String,
        #[eguis(resetable = "field_default")]
        i8: i8,
        #[eguisM(resetable = "field_default")]
        i16: i16,
        i32: i32,
        i64: i64,
        i128: i128,
        isize: isize,
        #[eguis(imut, hint = "This is isize but immutable")]
        isize_imut: isize,
        #[eguis(
            hint = "This is also isize but limited to range <5,11>",
            config = "Slider(5,11)"
        )]
        limited_isize: isize,
        #[eguis(config = "SliderStep(5,110,5)")]
        stepped_isize: isize,
        bool: bool,
        u8: u8,
        u16: u16,
        #[eguis(
            map_pre_ref = u32::to_string,
            map_post = (
                |field:&mut
                u32,
                mapped:String|{use
                std::str::FromStr;if
                let
                Ok(new_val)= u32::from_str(mapped.as_str()){*field = new_val;}}
            )
        )]
        u32: u32,
        #[eguis(
            resetable = "not_resetable",
            map_pre = RwLock::write,
            eeq = (
                |field:&RwLock<u32>,
                rhs:&RwLock<u32>|field.read().eguis_eq(&*rhs.read())
            ),
            eclone = (
                |field:&mut
                RwLock<u32>,
                rhs:&RwLock<u32>|field.write().eguis_clone(&*rhs.read())
            )
        )]
        u32_rwlock: RwLock<u32>,
        f32: f32,
        f64: f64,
        u128: u128,
        usize: usize,
        #[eguis(
            hint = "fields in derived struct needs to implement EguiStructMut or deref to type that implements it"
        )]
        usize_boxed: Box<usize>,
        #[eguis(config = "Some(&mut [2,3,5,7,11,13,17,19].into_iter())")]
        u8_combobox_wrapper: Combobox<u8>,
        #[eguis(config = "ComboBox(&mut [2,3,5,7,11,13,17,19].into_iter())")]
        u8_combobox_config: u8,
        #[eguis(config = "Some(&mut STATIC_COMBOBOX.read().clone().into_iter())")]
        static_combobox: Combobox<String>,
        primary_color: Color,
        secondary_color: Color,
    }
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for Data {
        const SIMPLE_MUT: ::std::primitive::bool = false;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            !Self::SIMPLE_MUT
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            static STRUCT_DEFAULT: ::std::sync::OnceLock<Data> = ::std::sync::OnceLock::new();
            _ = STRUCT_DEFAULT.get_or_init(Data::default);
            response
                |= self
                    .string
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///String
                            "Data.string",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&"Resetable with expr".to_string()),
                        None,
                    );
            {};
            response
                |= self
                    .not_resetable_string
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Not resetable string
                            "Data.not_resetable_string",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::None,
                        None,
                    );
            {};
            response
                |= self
                    .i8
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///I 8
                            "Data.i8",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&::std::default::Default::default()),
                        None,
                    );
            {};
            response
                |= self
                    .i16
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///I 16
                            "Data.i16",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&::std::default::Default::default()),
                        None,
                    );
            {};
            response
                |= self
                    .i32
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///I 32
                            "Data.i32",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().i32),
                        None,
                    );
            {};
            response
                |= self
                    .i64
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///I 64
                            "Data.i64",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().i64),
                        None,
                    );
            {};
            response
                |= self
                    .i128
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///I 128
                            "Data.i128",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().i128),
                        None,
                    );
            {};
            response
                |= self
                    .isize
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Isize
                            "Data.isize",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().isize,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .isize_imut
                    .show_collapsing_imut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Isize imut
                            "Data.isize_imut",
                        ),
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///This is isize but immutable
                            "Data.isize_imut.__hint.",
                        ),
                        ::std::default::Default::default(),
                        ::std::option::Option::None,
                        None,
                    );
            response
                |= self
                    .limited_isize
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Limited isize
                            "Data.limited_isize",
                        ),
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///This is also isize but limited to range <5,11>
                            "Data.limited_isize.__hint.",
                        ),
                        Slider(5, 11),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().limited_isize,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .stepped_isize
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Stepped isize
                            "Data.stepped_isize",
                        ),
                        "",
                        SliderStep(5, 110, 5),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().stepped_isize,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .bool
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Bool
                            "Data.bool",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().bool),
                        None,
                    );
            {};
            response
                |= self
                    .u8
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///U 8
                            "Data.u8",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().u8),
                        None,
                    );
            {};
            response
                |= self
                    .u16
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///U 16
                            "Data.u16",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().u16),
                        None,
                    );
            {};
            #[allow(unused_mut)]
            let mut mapped = u32::to_string(&mut self.u32);
            let r = mapped
                .show_collapsing_mut(
                    ui,
                    crate::_rust_i18n_translate(
                        rust_i18n::locale().as_str(),
                        #[allow(unused_doc_comments)]
                        #[allow(unused_doc_comments)]
                        ///U 32
                        "Data.u32",
                    ),
                    "",
                    ::std::default::Default::default(),
                    ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().u32)
                        .map(|x| u32::to_string(x))
                        .as_ref(),
                    None,
                );
            response |= r.clone();
            if r.changed() {
                (|field: &mut u32, mapped: String| {
                    use std::str::FromStr;
                    if let Ok(new_val) = u32::from_str(mapped.as_str()) {
                        *field = new_val;
                    }
                })(&mut self.u32, mapped);
            }
            {};
            #[allow(unused_mut)]
            let mut mapped = RwLock::write(&mut self.u32_rwlock);
            let r = mapped
                .show_collapsing_mut(
                    ui,
                    crate::_rust_i18n_translate(
                        rust_i18n::locale().as_str(),
                        #[allow(unused_doc_comments)]
                        #[allow(unused_doc_comments)]
                        ///U 32 rwlock
                        "Data.u32_rwlock",
                    ),
                    "",
                    ::std::default::Default::default(),
                    ::std::option::Option::None.map(|x| (x)).as_ref(),
                    None,
                );
            response |= r.clone();
            {};
            response
                |= self
                    .f32
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///F 32
                            "Data.f32",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().f32),
                        None,
                    );
            {};
            response
                |= self
                    .f64
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///F 64
                            "Data.f64",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().f64),
                        None,
                    );
            {};
            response
                |= self
                    .u128
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///U 128
                            "Data.u128",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().u128),
                        None,
                    );
            {};
            response
                |= self
                    .usize
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Usize
                            "Data.usize",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().usize,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .usize_boxed
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Usize boxed
                            "Data.usize_boxed",
                        ),
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///fields in derived struct needs to implement EguiStructMut or deref to type that implements it
                            "Data.usize_boxed.__hint.",
                        ),
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().usize_boxed,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .u8_combobox_wrapper
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///U 8 combobox wrapper
                            "Data.u8_combobox_wrapper",
                        ),
                        "",
                        Some(&mut [2, 3, 5, 7, 11, 13, 17, 19].into_iter()),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().u8_combobox_wrapper,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .u8_combobox_config
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///U 8 combobox config
                            "Data.u8_combobox_config",
                        ),
                        "",
                        ComboBox(&mut [2, 3, 5, 7, 11, 13, 17, 19].into_iter()),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().u8_combobox_config,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .static_combobox
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Static combobox
                            "Data.static_combobox",
                        ),
                        "",
                        Some(&mut STATIC_COMBOBOX.read().clone().into_iter()),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().static_combobox,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .primary_color
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Primary color
                            "Data.primary_color",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().primary_color,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .secondary_color
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///Secondary color
                            "Data.secondary_color",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().secondary_color,
                        ),
                        None,
                    );
            {};
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            ui.dummy_response()
        }
        fn start_collapsed_mut(&self) -> bool {
            false
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for Data {
        fn eguis_clone(&mut self, rhs: &Self) {
            self.string.eguis_clone(&rhs.string);
            self.not_resetable_string.eguis_clone(&rhs.not_resetable_string);
            self.i8.eguis_clone(&rhs.i8);
            self.i16.eguis_clone(&rhs.i16);
            self.i32.eguis_clone(&rhs.i32);
            self.i64.eguis_clone(&rhs.i64);
            self.i128.eguis_clone(&rhs.i128);
            self.isize.eguis_clone(&rhs.isize);
            self.isize_imut.eguis_clone(&rhs.isize_imut);
            self.limited_isize.eguis_clone(&rhs.limited_isize);
            self.stepped_isize.eguis_clone(&rhs.stepped_isize);
            self.bool.eguis_clone(&rhs.bool);
            self.u8.eguis_clone(&rhs.u8);
            self.u16.eguis_clone(&rhs.u16);
            self.u32.eguis_clone(&rhs.u32);
            (|field: &mut RwLock<u32>, rhs: &RwLock<u32>| {
                field.write().eguis_clone(&*rhs.read())
            })(&mut self.u32_rwlock, &rhs.u32_rwlock);
            self.f32.eguis_clone(&rhs.f32);
            self.f64.eguis_clone(&rhs.f64);
            self.u128.eguis_clone(&rhs.u128);
            self.usize.eguis_clone(&rhs.usize);
            self.usize_boxed.eguis_clone(&rhs.usize_boxed);
            self.u8_combobox_wrapper.eguis_clone(&rhs.u8_combobox_wrapper);
            self.u8_combobox_config.eguis_clone(&rhs.u8_combobox_config);
            self.static_combobox.eguis_clone(&rhs.static_combobox);
            self.primary_color.eguis_clone(&rhs.primary_color);
            self.secondary_color.eguis_clone(&rhs.secondary_color);
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for Data {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            ret &= self.string.eguis_eq(&rhs.string);
            ret &= self.not_resetable_string.eguis_eq(&rhs.not_resetable_string);
            ret &= self.i8.eguis_eq(&rhs.i8);
            ret &= self.i16.eguis_eq(&rhs.i16);
            ret &= self.i32.eguis_eq(&rhs.i32);
            ret &= self.i64.eguis_eq(&rhs.i64);
            ret &= self.i128.eguis_eq(&rhs.i128);
            ret &= self.isize.eguis_eq(&rhs.isize);
            ret &= self.isize_imut.eguis_eq(&rhs.isize_imut);
            ret &= self.limited_isize.eguis_eq(&rhs.limited_isize);
            ret &= self.stepped_isize.eguis_eq(&rhs.stepped_isize);
            ret &= self.bool.eguis_eq(&rhs.bool);
            ret &= self.u8.eguis_eq(&rhs.u8);
            ret &= self.u16.eguis_eq(&rhs.u16);
            ret &= u32::to_string(&self.u32).eguis_eq(&u32::to_string(&self.u32));
            ret
                &= (|field: &RwLock<u32>, rhs: &RwLock<u32>| {
                    field.read().eguis_eq(&*rhs.read())
                })(&self.u32_rwlock, &rhs.u32_rwlock);
            ret &= self.f32.eguis_eq(&rhs.f32);
            ret &= self.f64.eguis_eq(&rhs.f64);
            ret &= self.u128.eguis_eq(&rhs.u128);
            ret &= self.usize.eguis_eq(&rhs.usize);
            ret &= self.usize_boxed.eguis_eq(&rhs.usize_boxed);
            ret &= self.u8_combobox_wrapper.eguis_eq(&rhs.u8_combobox_wrapper);
            ret &= self.u8_combobox_config.eguis_eq(&rhs.u8_combobox_config);
            ret &= self.static_combobox.eguis_eq(&rhs.static_combobox);
            ret &= self.primary_color.eguis_eq(&rhs.primary_color);
            ret &= self.secondary_color.eguis_eq(&rhs.secondary_color);
            ret
        }
    }
    impl Default for Data {
        fn default() -> Self {
            Self {
                skipped_data: 0,
                string: "Hello!".to_string(),
                not_resetable_string: "Hello!".to_string(),
                i8: 42,
                i16: 1555,
                i32: -242522,
                i64: 23425259,
                i128: i128::MAX,
                isize: -14,
                isize_imut: -333,
                limited_isize: 6,
                stepped_isize: 50,
                bool: true,
                u8: 94,
                u16: 14029,
                u32: 3025844,
                u32_rwlock: RwLock::new(9999),
                f32: std::f32::consts::PI,
                f64: std::f64::consts::PI,
                u128: u128::MAX,
                usize: usize::MAX,
                usize_boxed: Box::new(usize::MAX),
                u8_combobox_wrapper: Combobox(3),
                u8_combobox_config: 3,
                static_combobox: Combobox("default name".to_string()),
                primary_color: Color::default(),
                secondary_color: Color::default(),
            }
        }
    }
    pub struct TupleStruct(
        #[eguis(resetable = "struct_default")]
        u8,
        #[eguis(
            on_change_struct = (|s:&mut TupleStruct|s.2 = format!("Wololo!: {}", s.1))
        )]
        u32,
        String,
        SubData,
    );
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for TupleStruct {
        const SIMPLE_MUT: ::std::primitive::bool = false;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            !Self::SIMPLE_MUT
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            static STRUCT_DEFAULT: ::std::sync::OnceLock<TupleStruct> = ::std::sync::OnceLock::new();
            _ = STRUCT_DEFAULT.get_or_init(TupleStruct::default);
            response
                |= self
                    .0
                    .show_collapsing_mut(
                        ui,
                        "[0]",
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(&STRUCT_DEFAULT.get().unwrap().0),
                        None,
                    );
            {};
            response
                |= self
                    .1
                    .show_collapsing_mut(
                        ui,
                        "[1]",
                        "",
                        ::std::default::Default::default(),
                        reset2.map(|f| &f.1),
                        None,
                    );
            {
                if response.changed() {
                    (|s: &mut TupleStruct| {
                        s
                            .2 = {
                            let res = ::alloc::fmt::format(
                                format_args!("Wololo!: {0}", s.1),
                            );
                            res
                        };
                    })(self)
                }
            };
            response
                |= self
                    .2
                    .show_collapsing_mut(
                        ui,
                        "[2]",
                        "",
                        ::std::default::Default::default(),
                        reset2.map(|f| &f.2),
                        None,
                    );
            {};
            response
                |= self
                    .3
                    .show_collapsing_mut(
                        ui,
                        "[3]",
                        "",
                        ::std::default::Default::default(),
                        reset2.map(|f| &f.3),
                        None,
                    );
            {};
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            ui.dummy_response()
        }
        fn start_collapsed_mut(&self) -> bool {
            false
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for TupleStruct {
        fn eguis_clone(&mut self, rhs: &Self) {
            self.0.eguis_clone(&rhs.0);
            self.1.eguis_clone(&rhs.1);
            self.2.eguis_clone(&rhs.2);
            self.3.eguis_clone(&rhs.3);
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for TupleStruct {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            ret &= self.0.eguis_eq(&rhs.0);
            ret &= self.1.eguis_eq(&rhs.1);
            ret &= self.2.eguis_eq(&rhs.2);
            ret &= self.3.eguis_eq(&rhs.3);
            ret
        }
    }
    impl Default for TupleStruct {
        fn default() -> Self {
            Self(3, 24, "Hello!".to_string(), SubData::default())
        }
    }
    pub struct Metadata {
        message: String,
    }
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for Metadata {
        const SIMPLE_MUT: ::std::primitive::bool = false;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            !Self::SIMPLE_MUT
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            response
                |= self
                    .message
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///message
                            "Metadata.message",
                        ),
                        "",
                        ::std::default::Default::default(),
                        reset2.map(|f| &f.message),
                        None,
                    );
            {};
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            ui.dummy_response()
        }
        fn start_collapsed_mut(&self) -> bool {
            false
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for Metadata {
        fn eguis_clone(&mut self, rhs: &Self) {
            self.message.eguis_clone(&rhs.message);
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for Metadata {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            ret &= self.message.eguis_eq(&rhs.message);
            ret
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Metadata {
        #[inline]
        fn default() -> Metadata {
            Metadata {
                message: ::core::default::Default::default(),
            }
        }
    }
    #[eguis(resetable = "struct_default")]
    pub struct SubData {
        value: String,
        number: u32,
    }
    impl ::egui_struct::trait_implementor_set::EguiStructSplitMut for SubData {
        const SIMPLE_MUT: ::std::primitive::bool = false;
        type ConfigTypeSplitMut<'a> = ();
        fn has_childs_mut(&self) -> ::std::primitive::bool {
            !Self::SIMPLE_MUT
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            reset2: ::std::option::Option<&Self>,
        ) -> ::egui::Response {
            use ::egui_struct::trait_implementor_set::EguiStructMut;
            use ::egui_struct::trait_implementor_set::EguiStructImut;
            let mut response = ui
                .interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );
            static STRUCT_DEFAULT: ::std::sync::OnceLock<SubData> = ::std::sync::OnceLock::new();
            _ = STRUCT_DEFAULT.get_or_init(SubData::default);
            response
                |= self
                    .value
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///value
                            "SubData.value",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().value,
                        ),
                        None,
                    );
            {};
            response
                |= self
                    .number
                    .show_collapsing_mut(
                        ui,
                        crate::_rust_i18n_translate(
                            rust_i18n::locale().as_str(),
                            #[allow(unused_doc_comments)]
                            #[allow(unused_doc_comments)]
                            ///number
                            "SubData.number",
                        ),
                        "",
                        ::std::default::Default::default(),
                        ::std::option::Option::Some(
                            &STRUCT_DEFAULT.get().unwrap().number,
                        ),
                        None,
                    );
            {};
            response
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ::egui_struct::exgrid::ExUi,
            _config: Self::ConfigTypeSplitMut<'_>,
        ) -> ::egui::Response {
            ui.dummy_response()
        }
        fn start_collapsed_mut(&self) -> bool {
            false
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructClone for SubData {
        fn eguis_clone(&mut self, rhs: &Self) {
            self.value.eguis_clone(&rhs.value);
            self.number.eguis_clone(&rhs.number);
        }
    }
    impl ::egui_struct::trait_implementor_set::EguiStructEq for SubData {
        fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
            let mut ret = true;
            ret &= self.value.eguis_eq(&rhs.value);
            ret &= self.number.eguis_eq(&rhs.number);
            ret
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SubData {
        #[inline]
        fn default() -> SubData {
            SubData {
                value: ::core::default::Default::default(),
                number: ::core::default::Default::default(),
            }
        }
    }
    pub struct DemoApp {
        data: Data,
    }
    #[automatically_derived]
    impl ::core::default::Default for DemoApp {
        #[inline]
        fn default() -> DemoApp {
            DemoApp {
                data: ::core::default::Default::default(),
            }
        }
    }
    impl eframe::App for DemoApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            let Self { data } = self;
            egui::CentralPanel::default()
                .show(
                    ctx,
                    |ui| {
                        ui.style_mut().visuals.striped = true;
                        data.eguis_mut().label(RichText::new("Data").heading()).show(ui);
                    },
                );
        }
    }
}
use app::DemoApp;
use rust_i18n::BackendExt;
/// I18n backend instance
///
/// [PUBLIC] This is a public API, and as an example in examples/
#[allow(missing_docs)]
static _RUST_I18N_BACKEND: rust_i18n::once_cell::sync::Lazy<
    Box<dyn rust_i18n::Backend>,
> = rust_i18n::once_cell::sync::Lazy::new(|| {
    let mut backend = rust_i18n::SimpleBackend::new();
    let trs = [
        ("Color.Custom", "Custom"),
        ("Color.Green", "Green"),
        ("Color.Named", "Named"),
        ("Color.Named2", "Named2"),
        ("Color.Named2.name", "name"),
        ("Color.NamedCustom", "Renamed Custom"),
        ("Color.NamedCustom.__hint", "This is named custom"),
        ("Color.NamedCustom.blue", "blue"),
        ("Color.NamedCustom.green", "green"),
        ("Color.NamedCustom.metadata", "metadata"),
        ("Color.NamedCustom.red", "red"),
        ("Color.Red", "Red"),
        ("Data.app_language", "App language"),
        ("Data.bool", "Bool"),
        ("Data.f32", "F 32"),
        ("Data.f64", "F 64"),
        ("Data.hashmap", "Hashmap"),
        ("Data.i128", "I 128"),
        ("Data.i16", "I 16"),
        ("Data.i32", "I 32"),
        ("Data.i64", "I 64"),
        ("Data.i8", "I 8"),
        ("Data.isize", "Isize"),
        ("Data.isize_imut", "Isize imut"),
        ("Data.isize_imut.__hint.", "This is isize but immutable"),
        ("Data.limited_isize", "Limited isize"),
        ("Data.limited_isize.__hint.", "This is also isize but limited to range <5,11>"),
        ("Data.list", "List"),
        ("Data.nested_struct", "Nested struct"),
        ("Data.not_resetable_string", "Not resetable string"),
        ("Data.optional", "Optional"),
        ("Data.optional.__hint.", "This is Option<_>"),
        ("Data.optional_optional", "Optional Optional"),
        ("Data.optional_string", "Optional string"),
        (
            "Data.optional_string.__hint.",
            "This is also Option, but as inner value is simple it is presented inline",
        ),
        ("Data.primary_color", "Primary color"),
        ("Data.secondary_color", "Secondary color"),
        ("Data.static_combobox", "Static combobox"),
        ("Data.stepped_isize", "Stepped isize"),
        ("Data.string", "String"),
        ("Data.u128", "U 128"),
        ("Data.u16", "U 16"),
        ("Data.u32", "U 32"),
        ("Data.u32_rwlock", "U 32 rwlock"),
        ("Data.u8", "U 8"),
        ("Data.u8_combobox_config", "U 8 combobox config"),
        ("Data.u8_combobox_wrapper", "U 8 combobox wrapper"),
        ("Data.unnamed_struct", "Unnamed struct"),
        ("Data.usize", "Usize"),
        ("Data.usize_boxed", "Usize boxed"),
        (
            "Data.usize_boxed.__hint.",
            "fields in derived struct needs to implement EguiStructMut or deref to type that implements it",
        ),
        ("Language.English", "English"),
        ("Language.Polish", "Polish"),
        ("Metadata.message", "message"),
        ("SubData.number", "number"),
        ("SubData.value", "value"),
    ];
    backend.add_translations("en", &trs.into_iter().collect());
    let trs = [
        ("Color.Custom", "Niestandardowy"),
        ("Color.Green", "Zielony"),
        ("Color.Named", "Nazwany"),
        ("Color.Named2", "Nazwany2"),
        ("Color.Named2.name", "nazwa"),
        ("Color.NamedCustom", "Renamed Custom"),
        ("Color.NamedCustom.__hint", "To jest niestandardowy kolor z nazwanymi polami"),
        ("Color.NamedCustom.blue", "niebieski"),
        ("Color.NamedCustom.green", "zielony"),
        ("Color.NamedCustom.metadata", "metadane"),
        ("Color.NamedCustom.red", "czerwony"),
        ("Color.Red", "Czerwony"),
        ("Data.app_language", "J─Özyk Aplikacji"),
        ("Data.bool", "Bool"),
        ("Data.f32", "F 32"),
        ("Data.f64", "F 64"),
        ("Data.hashmap", "Hashmap"),
        ("Data.i128", "I 128"),
        ("Data.i16", "I 16"),
        ("Data.i32", "I 32"),
        ("Data.i64", "I 64"),
        ("Data.i8", "I 8"),
        ("Data.isize", "Isize"),
        ("Data.isize_imut", "Isize imut"),
        ("Data.isize_imut.__hint.", "This is isize but immutable"),
        ("Data.limited_isize", "Limited isize"),
        ("Data.limited_isize.__hint.", "This is also isize but limited to range <5,11>"),
        ("Data.list", "Lista"),
        ("Data.nested_struct", "Zagnie┼╝d┼╝ona struktura"),
        ("Data.not_resetable_string", "Not resetable string"),
        ("Data.optional", "Pole opcjonalne"),
        ("Data.optional.__hint.", "This is Option<_>"),
        ("Data.optional_string", "Optional string"),
        (
            "Data.optional_string.__hint.",
            "This is also Option, but as inner value is simple it is presented inline",
        ),
        ("Data.primary_color", "Primary color"),
        ("Data.secondary_color", "Secondary color"),
        ("Data.static_combobox", "Static combobox"),
        ("Data.stepped_isize", "Stepped isize"),
        ("Data.string", "String"),
        ("Data.u128", "U 128"),
        ("Data.u16", "U 16"),
        ("Data.u32", "U 32"),
        ("Data.u32_rwlock", "U 32 rwlock"),
        ("Data.u8", "U 8"),
        ("Data.u8_combobox_config", "U 8 combobox config"),
        ("Data.u8_combobox_wrapper", "U 8 combobox wrapper"),
        ("Data.unnamed_struct", "Unnamed struct"),
        ("Data.usize", "Usize"),
        ("Data.usize_boxed", "Usize boxed"),
        (
            "Data.usize_boxed.__hint.",
            "fields in derived struct needs to implement EguiStructMut or deref to type that implements it",
        ),
        ("Language.English", "English"),
        ("Language.Polish", "Polski"),
        ("Metadata.message", "message"),
        ("SubData.number", "number"),
        ("SubData.value", "value"),
    ];
    backend.add_translations("pl", &trs.into_iter().collect());
    Box::new(backend)
});
static _RUST_I18N_FALLBACK_LOCALE: Option<&'static str> = Some("en");
/// Get I18n text by locale and key
#[inline]
#[allow(missing_docs)]
pub fn _rust_i18n_translate(locale: &str, key: &str) -> String {
    if let Some(value) = _RUST_I18N_BACKEND.translate(locale, key) {
        return value.to_string();
    }
    if let Some(fallback) = _RUST_I18N_FALLBACK_LOCALE {
        if let Some(value) = _RUST_I18N_BACKEND.translate(fallback, key) {
            return value.to_string();
        }
    }
    if locale.is_empty() {
        return key.to_string();
    }
    return {
        let res = ::alloc::fmt::format(format_args!("{0}.{1}", locale, key));
        res
    };
}
#[allow(missing_docs)]
pub fn _rust_i18n_available_locales() -> Vec<&'static str> {
    let mut locales = _RUST_I18N_BACKEND.available_locales();
    locales.sort();
    locales
}
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_struct demo",
        native_options,
        Box::new(|_creation_context| Box::<DemoApp>::default()),
    )
}
