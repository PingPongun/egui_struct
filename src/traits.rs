#[cfg(doc)]
use crate::config::*;
use crate::egui;
#[cfg(doc)]
use crate::*;
use egui::{Response, RichText, ScrollArea, Ui};
use exgrid::*;
use std::ops::Deref;

macro_rules! generate_show {
    ($show_collapsing:ident, $show_primitive:ident, $show_childs:ident, $start_collapsed:ident,
         $typ:ty, $ConfigType:ident, $SIMPLE:ident, $has_childs:ident, $has_primitive:ident, $preview_str:ident) => {
        /// Type that will pass some data to customize how data is shown, in most cases this will be () (eg. for numerics this is [ConfigNum])
        type $ConfigType<'a>: Default;

        /// Flag that indicates that data can be shown in the same line as parent (set to true if data is shown as single&simple widget)
        const $SIMPLE: bool = true;

        /// Indicates if data has childs section at the moment
        fn $has_childs(&self) -> bool {
            false
        }

        /// Indicates if data has primitive section at the moment
        fn $has_primitive(&self) -> bool {
            !self.$has_childs()
        }
        /// Use it when implementing [.show_childs_mut()](EguiStructMut::show_childs_mut) to display single nested element
        ///
        /// ⚠ WARNING: This function is not intended for manual implementation ⚠
        fn $show_collapsing(
            self: $typ,
            ui: &mut ExUi,
            label: impl Into<RichText> + Clone,
            hint: impl Into<RichText> + Clone,
            config: &mut Self::$ConfigType<'_>, //TODO use &
            reset2: Option<&Self>,
            start_collapsed: Option<bool>,
        ) -> Response {
            let has_childs = self.$has_childs();
            let header = |ui: &mut ExUi| {
                crate::trait_implementor_set::primitive_label(ui, label, hint);
                macro_rules! primitive {
                    (show_primitive_imut) => {
                        self.show_primitive_imut(ui, config)
                    };
                    (show_primitive_mut) => {
                        crate::trait_implementor_set::primitive_w_reset(self, ui, config, reset2)
                    };
                }
                primitive!($show_primitive)
            };
            ui.maybe_collapsing_rows(has_childs, header)
                .initial_state(|| start_collapsed.unwrap_or(self.$start_collapsed()))
                .body_simple(|ui| self.$show_childs(ui, config, reset2))
        }
        /// UI elements shown in the same line as label
        ///
        /// If data element view is fully contained in childs section(does not have primitive section), leave this & [.has_primitive()](EguiStructMut::has_primitive_mut) with default impl
        fn $show_primitive(
            self: $typ,
            ui: &mut ExUi,
            _config: &mut Self::$ConfigType<'_>,
        ) -> Response {
            ui.dummy_response()
        }

        /// UI elements related to nested data, that is show inside collapsible rows
        ///
        /// If data element view is simple & can fully be contained in primitive section, leave this & [.has_childs()](EguiStructMut::has_childs_mut) with default impl
        fn $show_childs(
            self: $typ,
            ui: &mut ExUi,
            _config: &mut Self::$ConfigType<'_>,
            _reset2: Option<&Self>,
        ) -> Response {
            ui.dummy_response()
        }

        /// Controls if struct is initially collapsed/uncollapsed (if "show_childs_mut" is shown by default)
        ///
        /// eg. Collections (vecs, slices, hashmaps, ..) are initially collapsed if they have more than 16 elements
        fn $start_collapsed(&self) -> bool {
            false
        }

        // /// String that may be used by parent structs to hint its content
        // ///
        // /// eg. `Vec<int>` may display preview of its data as `[1,2,3,..]#100`
        // /// (impl of preview_str() for int returns its value as str)
        // fn $preview_str<'b>(&'b self) -> &'b str {
        //     ""
        // }
    };
}
/// Similar to std [`Clone`] trait, but they respect `#[eguis(skip)]`.
///
/// Necessary to implement [`EguiStructMut`]. Used to provide reset functionality.
///
/// If type is [`Clone`] can be implemented with [`impl_eclone!`]/[`impl_eeqclone!`].
pub trait EguiStructClone
where
    Self: Sized,
{
    /// Similar to std [`Clone::clone_from`], but respects `#[eguis(skip)]` (may not clone all data)
    fn eguis_clone(&mut self, source: &Self);

    /// If type is [`Clone`] same as [`Clone::clone`], otherwise it should return instance of
    /// `Self` that matches `self` on significant fields(not marked with `#[eguis(skip)]`),
    /// Can return eg. `Self::default().eguis_clone(self)`
    /// If new instance of `Self` can not be created should return `None`
    fn eguis_clone_full(&self) -> Option<Self> {
        None
    }
}

/// Similar to std [`PartialEq`] trait, but they respect `#[eguis(skip)]`.
///
/// Necessary to implement [`EguiStructMut`]. Used to provide reset functionality (if reset is not used, may be blank impl).
///
/// If type is [`PartialEq`] can be implemented with [`impl_eeq!`]/[`impl_eeqclone!`].
pub trait EguiStructEq {
    fn eguis_eq(&self, _rhs: &Self) -> bool {
        //default implementation can be used if reset button is not required
        true
    }
}
// pub trait EguiStructResettable {
//     type Reset2;
//     fn reset_possible(&self, _rhs: &Self::Reset2) -> bool {
//         //default implementation can be used if reset button is not required
//         false
//     }
//     fn reset2(&mut self, source: &Self::Reset2);
// }

// #[macro_export]
// macro_rules! impl_egui_struct_resettable {
//     ($($type:ty)*) => {
//         $(
//             impl EguiStructResettable for $type {
//                 type Reset2 = $type;

//                 fn reset2(&mut self, source: &Self::Reset2) {
//                     *self = source.clone()
//                 }

//                 fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
//                     //default implementation can be used if reset button is not required
//                     self == rhs
//                 }
//             }
//         )*
//     };
// }
// impl_egui_struct_resettable! {i8 i16 i32 i64 u8 u16 u32 u64 isize f32 f64 bool u128 i128 String}
// impl EguiStructResettable for usize {
//     type Reset2 = EguiStructConfig<ConfigNum<'static, usize>, usize>;
//     fn reset2(&mut self, source: &Self::Reset2) {
//         *self = source.reset.clone().unwrap()
//     }
//     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
//         if let Some(r) = rhs.reset {
//             *self == r
//         } else {
//             false
//         }
//         // *self == rhs.0
//     }
// }
// struct Test {
//     a: usize,
//     b: usize,
// }
// pub struct EguiStructConfig<C, R> {
//     reset_btn: bool,
//     config: Option<C>,
//     reset: Option<R>,
//     label: String,
//     hint: String,
// }

// impl<C, R> Default for EguiStructConfig<C, R> {
//     fn default() -> Self {
//         Self {
//             reset_btn: true,
//             config: None,
//             reset: None,
//             label: "".into(),
//             hint: "".into(),
//         }
//     }
// }

// #[allow(nonstandard_style)]
// struct _Test___EguiStructResettable {
//     eguis: EguiStructConfig<ConfigStr<'static>, ()>,
//     a: <usize as EguiStructResettable>::Reset2,
//     b: <usize as EguiStructResettable>::Reset2,
// }
// impl Default for _Test___EguiStructResettable {
//     fn default() -> Self {
//         Self {
//             a: EguiStructConfig {
//                 config: Some(ConfigNum::Slider(1, 10)),
//                 reset: Some(5),
//                 label: "A field".into(),
//                 hint: "This is A field".into(),
//                 ..Default::default()
//             },
//             b: EguiStructConfig {
//                 label: "B field".into(),
//                 hint: "This is B field".into(),
//                 ..Default::default()
//             },
//             eguis: EguiStructConfig {
//                 reset_btn: true,
//                 label: "".into(),
//                 hint: "".into(),
//                 config: None,
//                 reset: None,
//             },
//         }
//     }
// }
// impl EguiStructResettable for Test {
//     type Reset2 = _Test___EguiStructResettable;

//     fn reset2(&mut self, source: &Self::Reset2) {
//         self.a.reset2(&source.a);
//         self.b.reset2(&source.b)
//     }

//     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
//         self.a.reset_possible(&rhs.a) || self.b.reset_possible(&rhs.b)
//     }
// }

/// Trait, that allows generating mutable view of data (takes `&mut data`)
pub trait EguiStructMut: EguiStructClone + EguiStructEq {
    generate_show! { show_collapsing_mut, show_primitive_mut, show_childs_mut, start_collapsed_mut,
    &mut Self, ConfigTypeMut, SIMPLE_MUT, has_childs_mut, has_primitive_mut, preview_str_mut }
}
/// Trait, that allows generating immutable view of data (takes `&data`)
pub trait EguiStructImut {
    generate_show! { show_collapsing_imut, show_primitive_imut, show_childs_imut, start_collapsed_imut,
    &Self, ConfigTypeImut, SIMPLE_IMUT, has_childs_imut, has_primitive_imut, preview_str_imut }
}
macro_rules! generate_IntoEguiStruct {
    ($typ:ty, $cfg_name:ident, $trait:ident) => {
        fn $cfg_name(self: $typ) -> EguiStructWrapper<$typ>
        where
            Self: $trait,
        {
            EguiStructWrapper {
                data: self,
                label: Default::default(),
                reset2: None,
                scroll_area_auto_shrink: [true; 2],
                scroll_bar_visibility: Default::default(),
                striped: None,
                view_mode: Default::default(),
            }
        }
    };
}

pub trait EguiStruct {
    generate_IntoEguiStruct! {&mut Self, eguis_mut, EguiStructMut}
    generate_IntoEguiStruct! {&Self, eguis_imut, EguiStructImut}
}
impl<T> EguiStruct for T {}
#[non_exhaustive]
pub struct EguiStructWrapper<'a, T: Deref> {
    pub data: T,
    pub label: RichText,
    pub reset2: Option<&'a T::Target>,
    pub scroll_area_auto_shrink: [bool; 2],
    pub scroll_bar_visibility: egui::scroll_area::ScrollBarVisibility,
    pub striped: Option<bool>,
    pub view_mode: exgrid::GridMode,
}

macro_rules! generate_EguiStruct_show {
    ($show_collapsing:ident, $generic:ident, $typ:ty, $bound:ident) => {
        impl<'a, $generic: $bound + ?Sized> EguiStructWrapper<'a, $typ> {
            pub fn show(self, ui: &mut Ui) -> Response {
                let id = ui.make_persistent_id((
                    self.label.text().to_string(),
                    std::any::type_name::<$generic>(),
                ));
                ScrollArea::vertical()
                    .id_source(id)
                    .auto_shrink(self.scroll_area_auto_shrink)
                    .show(ui, |ui| {
                        let mut grid = ExGrid::new(id);
                        if let Some(s) = self.striped {
                            grid = grid.striped(s);
                        }
                        grid.mode(self.view_mode)
                            .show(ui, |ui| {
                                self.data.$show_collapsing(
                                    ui,
                                    self.label,
                                    "",
                                    &mut Default::default(),
                                    self.reset2,
                                    None,
                                )
                            })
                            .inner
                    })
                    .inner
            }
        }
    };
}
generate_EguiStruct_show! {show_collapsing_mut, T, &mut T, EguiStructMut}
generate_EguiStruct_show! {show_collapsing_imut, T, &T, EguiStructImut}

impl<'a, T: Deref> EguiStructWrapper<'a, T> {
    pub fn auto_shrink(mut self, val: [bool; 2]) -> Self {
        self.scroll_area_auto_shrink = val;
        self
    }

    pub fn label(mut self, label: impl Into<RichText> + Clone) -> Self {
        self.label = label.into();
        self
    }

    pub fn reset2(mut self, reset2: &'a T::Target) -> Self {
        self.reset2 = Some(reset2);
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = Some(striped);
        self
    }

    pub fn view_mode(mut self, view_mode: exgrid::GridMode) -> Self {
        self.view_mode = view_mode;
        self
    }
}
