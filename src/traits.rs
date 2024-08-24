use crate::egui;
use egui::{Response, RichText, ScrollArea, Ui};
use exgrid::*;
use std::ops::Deref;

macro_rules! generate_show_collapsing {
    ($show_collapsing_inner:ident, $primitive_name:ident, $childs_name:ident, $start_collapsed:ident,
         $typ:ty, $config:ident,$has_childs:ident) => {
        #[doc(hidden)]
        fn $show_collapsing_inner(
            self: $typ,
            ui: &mut ExUi,
            label: impl Into<RichText> + Clone,
            hint: impl Into<RichText> + Clone,
            indent_level: isize,
            config: Self::$config<'_>,
            reset2: Option<&Self>,
            start_collapsed: Option<bool>,
        ) -> Response {
            let has_childs = self.$has_childs();

            let header = |ui: &mut ExUi| {
                let lab = ui.extext(label);
                let hint = hint.into();
                if !hint.is_empty() {
                    lab.on_hover_text(hint);
                }
                #[allow(unused_mut)]
                let mut ret = self.$primitive_name(ui, config);
                macro_rules! reset {
                    (show_collapsing_inner_imut) => {
                        ret
                    };
                    (show_collapsing_inner_mut) => {
                        if let Some(reset2) = reset2 {
                            if !reset2.eguis_eq(self) {
                                let mut r = ui.button("⟲");
                                if r.clicked() {
                                    self.eguis_clone(reset2);
                                    r.mark_changed();
                                }
                                ret |= r;
                            }
                        }
                        ret
                    };
                }
                reset! {$show_collapsing_inner}
            };
            ui.maybe_collapsing_rows(has_childs, header)
                .initial_state(|| start_collapsed.unwrap_or(self.$start_collapsed()))
                .body_simple(|ui| self.$childs_name(ui, indent_level + 1, reset2))
        }
    };
}

pub trait EguiStructMutInner: EguiStructMut {
    generate_show_collapsing! { show_collapsing_inner_mut, show_primitive_mut, show_childs_mut, start_collapsed_mut,
    &mut Self, ConfigTypeMut, has_childs_mut }
}
/// Trait, that allows generating immutable view of data (takes `&data`)
pub trait EguiStructImutInner: EguiStructImut {
    generate_show_collapsing! { show_collapsing_inner_imut, show_primitive_imut, show_childs_imut, start_collapsed_imut,
    &Self, ConfigTypeImut, has_childs_imut }
}
impl<T: EguiStructMut + ?Sized> EguiStructMutInner for T {}
impl<T: EguiStructImut + ?Sized> EguiStructImutInner for T {}

macro_rules! generate_show {
    ($top_name:ident, $collapsing_name:ident, $show_collapsing_inner_mut:ident, $primitive_name:ident, $childs_name:ident, $start_collapsed:ident,
         $typ:ty, $config:ident, $SIMPLE:ident, $has_childs:ident, $has_primitive:ident) => {
        /// Type that will pass some data to customise how data is shown, in most cases this will be () (eg. for numerics this is [ConfigNum])
        type $config<'a>: Default;

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
        fn $collapsing_name(
            self: $typ,
            ui: &mut ExUi,
            label: impl Into<RichText> + Clone,
            hint: impl Into<RichText> + Clone,
            indent_level: isize,
            config: Self::$config<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            self.$show_collapsing_inner_mut(ui, label, hint, indent_level, config, reset2, None)
        }

        /// UI elements shown in the same line as label
        ///
        /// If data element view is fully contained in childs section(does not have primitive section), leave this & [.has_primitive()](EguiStructMut::has_primitive) with default impl
        fn $primitive_name(self: $typ, ui: &mut ExUi, _config: Self::$config<'_>) -> Response {
            ui.label("")
        }

        /// UI elements related to nested data, that is show inside collapsible rows
        ///
        /// If data element view is simple & can fully be contained in primitive section, leave this & [.has_childs()](EguiStructMut::has_childs) with default impl
        fn $childs_name(
            self: $typ,
            _ui: &mut ExUi,
            _indent_level: isize,
            _reset2: Option<&Self>,
        ) -> Response {
            unreachable!()
        }

        /// Controls if struct is initally collapsed/uncollapsed (if "show_childs_mut" is shown by default)
        ///
        /// eg. Collections (vecs, slices, hashmaps, ..) are initially collapsed if they have more than 16 elements
        fn $start_collapsed(&self) -> bool {
            false
        }
    };
}
/// Similar to std [`Clone`] trait, but they respect `#[eguis(skip)]`.
///
/// Necessary to implement [`EguiStructMut`]. Used to provide reset functionality.
///
/// If type is [`Clone`] can be implemented with [`impl_eclone!`]/[`impl_eeqclone!`].
pub trait EguiStructClone {
    fn eguis_clone(&mut self, source: &Self);
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
// pub trait EguiStructResetable {
//     type Reset2;
//     fn reset_possible(&self, _rhs: &Self::Reset2) -> bool {
//         //default implementation can be used if reset button is not required
//         false
//     }
//     fn reset2(&mut self, source: &Self::Reset2);
// }

// #[macro_export]
// macro_rules! impl_egui_struct_resetable {
//     ($($type:ty)*) => {
//         $(
//             impl EguiStructResetable for $type {
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
// impl_egui_struct_resetable! {i8 i16 i32 i64 u8 u16 u32 u64 isize f32 f64 bool u128 i128 String}
// impl EguiStructResetable for usize {
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
// struct _Test___EguiStructResetable {
//     eguis: EguiStructConfig<ConfigStr<'static>, ()>,
//     a: <usize as EguiStructResetable>::Reset2,
//     b: <usize as EguiStructResetable>::Reset2,
// }
// impl Default for _Test___EguiStructResetable {
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
// impl EguiStructResetable for Test {
//     type Reset2 = _Test___EguiStructResetable;

//     fn reset2(&mut self, source: &Self::Reset2) {
//         self.a.reset2(&source.a);
//         self.b.reset2(&source.b)
//     }

//     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
//         self.a.reset_possible(&rhs.a) || self.b.reset_possible(&rhs.b)
//     }
// }

/// Trait, that allows generating mutable view of data (takes `&mut data`)
///
///  For end user (if you implement trait with macro & not manualy) ofers one function [`.show_top_mut()`](Self::show_top_mut), which displays struct inside scroll area.
pub trait EguiStructMut: EguiStructClone + EguiStructEq {
    generate_show! { show_top_mut, show_collapsing_mut, show_collapsing_inner_mut, show_primitive_mut, show_childs_mut, start_collapsed_mut,
    &mut Self, ConfigTypeMut, SIMPLE_MUT, has_childs_mut, has_primitive_mut }
}
/// Trait, that allows generating immutable view of data (takes `&data`)
pub trait EguiStructImut {
    generate_show! { show_top_imut, show_collapsing_imut, show_collapsing_inner_imut, show_primitive_imut, show_childs_imut, start_collapsed_imut,
    &Self, ConfigTypeImut, SIMPLE_IMUT, has_childs_imut, has_primitive_imut }
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
                #[cfg(not(feature = "egui21"))]
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
    #[cfg(not(feature = "egui21"))]
    pub scroll_bar_visibility: egui::scroll_area::ScrollBarVisibility,
    pub striped: Option<bool>,
    pub view_mode: exgrid::GridMode,
}

macro_rules! generate_EguiStruct_show {
    ($collapsing_name:ident, $generic:ident, $typ:ty, $bound:ident) => {
        impl<'a, $generic: $bound + ?Sized> EguiStructWrapper<'a, $typ> {
            pub fn show(self, ui: &mut Ui) -> Response
            where
                $generic: 'static,
            {
                let id = ui.make_persistent_id((
                    self.label.text().to_string(),
                    std::any::TypeId::of::<$generic>(),
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
                                self.data.$collapsing_name(
                                    ui,
                                    self.label,
                                    "",
                                    -1,
                                    Default::default(),
                                    self.reset2,
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
