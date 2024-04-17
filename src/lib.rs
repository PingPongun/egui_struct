//! Crate consists of 4 traits ([`EguiStructImut`] & [`EguiStructMut`]: [`EguiStructEq`]+[`EguiStructClone`]) and two derive macros ([`macro@EguiStructImut`] to derive [`EguiStructImut`] & [`macro@EguiStructMut`] to derive the other three).
//!
//! See [demo](https://github.com/PingPongun/egui_struct/tree/master/demo)

use egui::{Button, Grid, Id, Response, ScrollArea, Ui, Widget, WidgetText};
pub mod prelude {
    pub use crate::EguiStruct;
    pub use egui_struct_macros::*;
}
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "egui21")]
use egui21 as egui;
#[cfg(feature = "egui22")]
use egui22 as egui;
#[cfg(feature = "egui23")]
use egui23 as egui;
#[cfg(feature = "egui24")]
use egui24 as egui;
#[cfg(feature = "egui25")]
use egui25 as egui;
#[cfg(feature = "egui26")]
use egui26 as egui;
#[cfg(feature = "egui27")]
use egui27 as egui;

macro_rules! generate_show_collapsing {
    ($show_collapsing_inner:ident, $primitive_name:ident, $childs_name:ident, $start_collapsed:ident,
         $typ:ty, $config:ident,$has_childs:ident) => {
        #[doc(hidden)]
        fn $show_collapsing_inner(
            self: $typ,
            ui: &mut Ui,
            label: impl Into<WidgetText> + Clone,
            hint: impl Into<WidgetText> + Clone,
            indent_level: isize,
            config: Self::$config<'_>,
            reset2: Option<&Self>,
            parent_id: Id,
            start_collapsed: Option<bool>,
        ) -> Response {
            let mut collapsed = false;
            let has_childs = self.$has_childs();
            let id = parent_id.with(label.clone().into().text());
            let label = label.into();
            let mut ret = ui.interact(
                egui::Rect::NOTHING,
                "dummy".into(),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );
            if !label.is_empty() || indent_level != -1 {
                ui.horizontal(|ui| {
                    if indent_level >= 0 {
                        for _ in 0..indent_level {
                            ui.separator();
                        }
                        if has_childs {
                            let id = id.with("__EguiStruct_collapsing_state");
                            collapsed = ui.data_mut(|d| {
                                d.get_temp_mut_or_insert_with(id, || {
                                    start_collapsed.unwrap_or(self.$start_collapsed())
                                })
                                .clone()
                            });
                            let icon = if collapsed { "⏵" } else { "⏷" };
                            if Button::new(icon).frame(false).small().ui(ui).clicked() {
                                ui.data_mut(|d| d.insert_temp(id, !collapsed));
                            }
                        }
                    }
                    let mut lab = ui.label(label);
                    let hint = hint.into();
                    if !hint.is_empty() {
                        lab = lab.on_hover_text(hint);
                    }
                    lab
                });

                ret = ui
                    .horizontal(|ui| {
                        let id = id.with("__EguiStruct_primitive");
                        #[allow(unused_mut)]
                        let mut ret = self.$primitive_name(ui, config, id);
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
                    })
                    .inner;
                ui.end_row();
            }

            if has_childs && !collapsed {
                ret = self.$childs_name(ui, indent_level + 1, ret, reset2, id);
            }
            ret
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
            ui: &mut Ui,
            label: impl Into<WidgetText> + Clone,
            hint: impl Into<WidgetText> + Clone,
            indent_level: isize,
            config: Self::$config<'_>,
            reset2: Option<&Self>,
            parent_id: Id,
        ) -> Response {
            self.$show_collapsing_inner_mut(
                ui,
                label,
                hint,
                indent_level,
                config,
                reset2,
                parent_id,
                None,
            )
        }

        /// UI elements shown in the same line as label
        ///
        /// If data element view is fully contained in childs section(does not have primitive section), leave this & [.has_primitive()](EguiStructMut::has_primitive) with default impl
        fn $primitive_name(
            self: $typ,
            ui: &mut Ui,
            _config: Self::$config<'_>,
            _id: impl Hash + Clone,
        ) -> Response {
            ui.label("")
        }

        /// UI elements related to nested data, that is show inside collapsible rows
        ///
        /// If data element view is simple & can fully be contained in primitive section, leave this & [.has_childs()](EguiStructMut::has_childs) with default impl
        fn $childs_name(
            self: $typ,
            _ui: &mut Ui,
            _indent_level: isize,
            _response: Response,
            _reset2: Option<&Self>,
            _parent_id: Id,
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

#[macro_export]
/// Generate [EguiStructClone] implementation based on [Clone]
macro_rules! impl_eclone {
    ([$($generics:tt)*], $type:ty) => {
        impl<$($generics)*> EguiStructClone for $type {
            fn eguis_clone(&mut self, source: &Self) {
                self.clone_from(source);
            }
        }
    };
}

#[macro_export]
/// Generate [EguiStructEq] implementation based on [PartialEq]
macro_rules! impl_eeq {
    ([$($generics:tt)*], $type:ty) => {
        impl<$($generics)*> EguiStructEq for $type {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                self == rhs
            }
        }
    };
}

#[macro_export]
/// Wrapper for both [impl_eeq!] & [impl_eclone!]
///
/// Generate [EguiStructClone] & [EguiStructEq] implementation based on [Clone] & [PartialEq]
///
/// Usage:
/// ```
/// impl_eeqclone!(MyType)
/// impl_eeqclone!([T], MyType2) //for MyType2<T>
/// ```
macro_rules! impl_eeqclone {
    ([$($generics:tt)*], $type:ty) => {
        impl_eeq!{[$($generics)*], $type}
        impl_eclone!{[$($generics)*], $type}
    };
    ($type:ty) => {impl_eeqclone!{[],$type}}
}

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
    pub label: WidgetText,
    pub reset2: Option<&'a T::Target>,
    pub scroll_area_auto_shrink: [bool; 2],
    #[cfg(not(feature = "egui21"))]
    pub scroll_bar_visibility: egui::scroll_area::ScrollBarVisibility,
    pub striped: Option<bool>,
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
                        let mut grid = Grid::new(id);
                        if let Some(s) = self.striped {
                            grid = grid.striped(s);
                        }
                        grid.show(ui, |ui| {
                            self.data.$collapsing_name(
                                ui,
                                self.label,
                                "",
                                -1,
                                Default::default(),
                                self.reset2,
                                id,
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
    pub fn label(mut self, label: impl Into<WidgetText> + Clone) -> Self {
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
}
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Config structure for mutable view of Numerics
#[derive(Default)]
pub enum ConfigNum<'a, T: 'a> {
    /// Default: DragValue (without limits)
    #[default]
    NumDefault,

    /// DragValue(min, max)
    DragValue(T, T),

    /// Slider(min, max)
    Slider(T, T),

    /// Slider(min, max, step)
    SliderStep(T, T, T),

    /// Combobox with available options specified by included iterator
    ComboBox(&'a mut dyn Iterator<Item = T>),
}
macro_rules! impl_num_primitives {
    ($($typ:ty)*) => {
        $(
            impl EguiStructMut for $typ {
                type ConfigTypeMut<'a> = ConfigNum<'a, $typ>;
                fn show_primitive_mut(&mut self, ui: &mut Ui, config: Self::ConfigTypeMut<'_>, id: impl Hash  + Clone) -> Response {
                    match config{
                        Self::ConfigTypeMut::NumDefault        =>  egui::DragValue::new(self).ui(ui),
                        Self::ConfigTypeMut::DragValue(min,max)=>  egui::DragValue::new(self).clamp_range(min..=max).ui(ui),
                        Self::ConfigTypeMut::Slider(min,max)   =>  egui::Slider::new(self, min..=max).ui(ui),
                        Self::ConfigTypeMut::SliderStep(min,max,step)   =>  egui::Slider::new(self, min..=max).step_by(step as f64).ui(ui),
                        Self::ConfigTypeMut::ComboBox(iter) => show_combobox(self, ui, Some(iter), id),
                    }
                }
            }
            impl EguiStructImut for $typ {
                type ConfigTypeImut<'a> = ConfigStrImut;
                fn show_primitive_imut(&self, ui: &mut Ui, config: Self::ConfigTypeImut<'_>, _id: impl Hash  + Clone) -> Response {
                    self.to_string().as_str().show_primitive_imut(ui, config, ())
                }
            }
            impl_eeqclone!{$typ}
        )*
    };
}

impl_num_primitives!(i8 i16 i32 i64 u8 u16 u32 u64 usize isize f32 f64);

impl EguiStructMut for bool {
    type ConfigTypeMut<'a> = ();
    fn show_primitive_mut(
        &mut self,
        ui: &mut Ui,
        _config: Self::ConfigTypeMut<'_>,
        _id: impl Hash + Clone,
    ) -> Response {
        egui::Checkbox::without_text(self).ui(ui)
    }
}
impl EguiStructImut for bool {
    type ConfigTypeImut<'a> = ();
    fn show_primitive_imut(
        &self,
        ui: &mut Ui,
        _config: Self::ConfigTypeImut<'_>,
        _id: impl Hash + Clone,
    ) -> Response {
        ui.add_enabled(false, egui::Checkbox::without_text(&mut self.clone()))
    }
}
impl_eeqclone! {bool}
/////////////////////////////////////////////////////////
///Config structure for mutable view of String
#[derive(Default)]
pub enum ConfigStr<'a> {
    ///Default: single line `egui::TextEdit`
    #[default]
    SingleLine,

    ///multi line `egui::TextEdit`
    MultiLine,

    ///Combobox with available options specified by included iterator
    ComboBox(&'a mut dyn Iterator<Item = String>),
}

///Config structure for immutable view of many simple types like str, String & numerics
#[derive(Default)]
pub enum ConfigStrImut {
    ///`egui::Label`
    NonSelectable,

    ///Default: imutable `egui::TextEdit`
    #[default]
    Selectable,
}

impl EguiStructMut for String {
    type ConfigTypeMut<'a> = ConfigStr<'a>;
    fn show_primitive_mut(
        &mut self,
        ui: &mut Ui,
        config: Self::ConfigTypeMut<'_>,
        id: impl Hash + Clone,
    ) -> Response {
        match config {
            ConfigStr::SingleLine => ui.text_edit_singleline(self),
            ConfigStr::MultiLine => ui.text_edit_multiline(self),
            ConfigStr::ComboBox(iter) => show_combobox(self, ui, Some(iter), id),
        }
    }
}
impl EguiStructImut for String {
    type ConfigTypeImut<'a> = ConfigStrImut;
    fn show_primitive_imut(
        &self,
        ui: &mut Ui,
        config: Self::ConfigTypeImut<'_>,
        _id: impl Hash + Clone,
    ) -> Response {
        self.as_str().show_primitive_imut(ui, config, ())
    }
}
impl_eeqclone! {String}

impl EguiStructImut for str {
    type ConfigTypeImut<'a> = ConfigStrImut;
    fn show_primitive_imut(
        mut self: &Self,
        ui: &mut Ui,
        config: Self::ConfigTypeImut<'_>,
        _id: impl Hash + Clone,
    ) -> Response {
        match config {
            ConfigStrImut::NonSelectable => ui.label(self),
            ConfigStrImut::Selectable => ui.text_edit_singleline(&mut self),
        }
    }
}

/////////////////////////////////////////////////////////
impl<T: EguiStructImut + Default> EguiStructImut for Option<T> {
    const SIMPLE_IMUT: bool = false;
    type ConfigTypeImut<'a> = ();
    fn has_childs_imut(&self) -> bool {
        !T::SIMPLE_IMUT && self.is_some()
    }
    fn has_primitive_imut(&self) -> bool {
        true
    }
    fn show_primitive_imut(
        &self,
        ui: &mut Ui,
        _config: Self::ConfigTypeImut<'_>,
        id: impl Hash + Clone,
    ) -> Response {
        ui.horizontal(|ui| {
            let mut ret = self.is_some().show_primitive_imut(ui, (), ());
            match (T::SIMPLE_IMUT, self) {
                (true, Some(value)) => ret |= value.show_primitive_imut(ui, Default::default(), id),
                (true, None) => (),
                (false, _) => (),
            }
            ret
        })
        .inner
    }
    fn show_childs_imut(
        &self,
        ui: &mut Ui,
        indent_level: isize,
        mut response: Response,
        _reset2: Option<&Self>,
        id: Id,
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive_imut() {
                response |= inner.show_collapsing_imut(
                    ui,
                    "[0]",
                    "",
                    indent_level,
                    Default::default(),
                    None,
                    id,
                );
            } else {
                response |= inner.show_childs_imut(ui, indent_level, response.clone(), None, id)
            }
        }
        response
    }
}
impl<T: EguiStructMut + Default> EguiStructMut for Option<T> {
    const SIMPLE_MUT: bool = false;
    type ConfigTypeMut<'a> = ();
    fn has_childs_mut(&self) -> bool {
        !T::SIMPLE_MUT && self.is_some()
    }
    fn has_primitive_mut(&self) -> bool {
        true
    }
    fn show_primitive_mut(
        &mut self,
        ui: &mut Ui,
        _config: Self::ConfigTypeMut<'_>,
        id: impl Hash + Clone,
    ) -> Response {
        ui.horizontal(|ui| {
            let mut checked = self.is_some();
            let mut ret = checked.show_primitive_mut(ui, (), ());

            match (checked, T::SIMPLE_MUT, self.as_mut()) {
                (true, true, Some(value)) => {
                    ret |= value.show_primitive_mut(ui, Default::default(), id)
                }
                (true, false, Some(_)) => (), //if inner is not simple, it will be shown below
                (true, _, None) => *self = Some(T::default()),
                (false, _, _) => *self = None,
            }
            ret
        })
        .inner
    }
    fn show_childs_mut(
        &mut self,
        ui: &mut Ui,
        indent_level: isize,
        mut response: Response,
        reset2: Option<&Self>,
        id: Id,
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive_mut() {
                response |= inner.show_collapsing_mut(
                    ui,
                    "[0]",
                    "",
                    indent_level,
                    Default::default(),
                    reset2.unwrap_or(&None).as_ref(),
                    id,
                );
            } else {
                response |= inner.show_childs_mut(
                    ui,
                    indent_level,
                    response.clone(),
                    reset2.unwrap_or(&None).as_ref(),
                    id,
                )
            }
        }
        response
    }
}
impl<T: EguiStructClone + Default> EguiStructClone for Option<T> {
    fn eguis_clone(&mut self, source: &Self) {
        if let Some(source) = source {
            if let Some(s) = self {
                s.eguis_clone(source);
            } else {
                let mut v: T = Default::default();
                v.eguis_clone(source);
                *self = Some(v);
            }
        } else {
            *self = None;
        }
    }
}
impl<T: EguiStructEq> EguiStructEq for Option<T> {
    fn eguis_eq(&self, rhs: &Self) -> bool {
        if let Some(s) = self {
            if let Some(r) = rhs {
                s.eguis_eq(r)
            } else {
                false
            }
        } else {
            false
        }
    }
}
///////////////////////////////////////////////////
macro_rules! impl_vec {
    ($Self:ty, $typ:ty, $iter:ident, $collapsing_name:ident, $childs_name:ident, $start_collapsed_mut:ident,
        $trait:ident, $SIMPLE:ident, $ConfigTypeMut:ident, $has_childs_imut:ident, $has_primitive_mut:ident) => {

        impl<T: $trait> $trait for $typ{
            const $SIMPLE: bool = false;
            type $ConfigTypeMut<'a> = ();
            fn $has_childs_imut(&self) -> bool {
                !self.is_empty()
            }
            fn $has_primitive_mut(&self) -> bool {
                false
            }
            fn $childs_name(
                self: $Self,
                ui: &mut Ui,
                indent_level: isize,
                mut response: Response,
                _reset2: Option<&Self>,
                id: Id
            ) -> Response {
                self.$iter().enumerate().for_each(|(idx, x)| {
                    response |= x.$collapsing_name(ui, idx.to_string(), "", indent_level, Default::default(), None, id)
                });
                response
            }
            fn $start_collapsed_mut(&self) -> bool {
                self.len() > 16
            }
        }
    };
    (IMUT, $($typ:ty)*) => { $(impl_vec! {&Self, $typ, iter, show_collapsing_imut, show_childs_imut, start_collapsed_imut,
        EguiStructImut, SIMPLE_IMUT, ConfigTypeImut, has_childs_imut, has_primitive_imut})* };
    ($typ:ty) => {
        impl_vec! {IMUT, $typ}
        impl_vec! {&mut Self, $typ, iter_mut, show_collapsing_mut, show_childs_mut, start_collapsed_mut,
            EguiStructMut, SIMPLE_MUT, ConfigTypeMut, has_childs_mut, has_primitive_mut}

        impl<T: EguiStructClone> EguiStructClone for $typ {
            fn eguis_clone(&mut self, source: &Self) {
                //TODO update this if vector length can change
                self.iter_mut().zip(source.iter()).for_each(|(s,r)|s.eguis_clone(r))
            }
        }
        impl<T: EguiStructEq> EguiStructEq for $typ  {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                let mut ret = self.len()==rhs.len();
                self.iter().zip(rhs.iter()).for_each(|(s,r)|ret &= s.eguis_eq(r));
                ret
            }
        }
    };
}

impl_vec! {[T]}
impl_vec! {Vec<T>}
impl_vec! {IMUT, std::collections::HashSet<T> }
#[cfg(feature = "indexmap")]
impl_vec! {IMUT, indexmap::IndexSet<T> }

/////////////////////////////////////////////////
macro_rules! impl_map {
    ($Self:ty, $typ:ty, [$( $Qbound:path),*], $iter:ident, $collapsing_name:ident, $childs_name:ident, $start_collapsed_mut:ident,
        $trait:ident, $SIMPLE_MUT:ident, $ConfigTypeMut:ident, $has_childs_imut:ident, $has_primitive_mut:ident) => {

        impl<Q: ToString $(+ $Qbound)*, V: $trait> $trait for $typ{
            const $SIMPLE_MUT: bool = false;
            type $ConfigTypeMut<'a> = ();
            fn $has_childs_imut(&self) -> bool {
                !self.is_empty()
            }
            fn $has_primitive_mut(&self) -> bool {
                false
            }
            fn $childs_name(
                self: $Self,
                ui: &mut Ui,
                indent_level: isize,
                mut response: Response,
                _reset2: Option<&Self>,id:Id
            ) -> Response {
                self.$iter().for_each(|(q, v)| {
                    response |= v.$collapsing_name(
                        ui,
                        q.to_string(),
                        "",
                        indent_level,
                        Default::default(),
                        None,
                        id
                    )
                });
                response
            }
            fn $start_collapsed_mut(&self) -> bool {
                self.len() > 16
            }
        }
    };
    ($typ:ty) => {
        impl_map! {&Self, $typ, [], iter, show_collapsing_imut, show_childs_imut, start_collapsed_imut,
            EguiStructImut, SIMPLE_IMUT, ConfigTypeImut, has_childs_imut, has_primitive_imut}
        impl_map! {&mut Self, $typ, [Eq, std::hash::Hash], iter_mut, show_collapsing_mut, show_childs_mut, start_collapsed_mut,
            EguiStructMut, SIMPLE_MUT, ConfigTypeMut, has_childs_mut, has_primitive_mut}

        impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructClone> EguiStructClone for $typ {
            fn eguis_clone(&mut self, source: &Self) {
                //this is very simplified implementation, that assumes that lenghts & keys are the same
                self.iter_mut().for_each(|(q, v)| {
                    if let Some(r) = source.get(q) {
                        v.eguis_clone(r)
                    }
                })
            }
        }
        impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructEq> EguiStructEq for $typ {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                let mut ret = self.len() == rhs.len();
                self.iter().for_each(|(q, v)| {
                    if let Some(r) = rhs.get(q) {
                        ret &= v.eguis_eq(r)
                    } else {
                        ret = false
                    }
                });
                ret
            }
        }
    };
}

impl_map! { std::collections::HashMap<Q,V> }
#[cfg(feature = "indexmap")]
impl_map! { indexmap::IndexMap<Q,V> }
///////////////////////////////////////////////////////
macro_rules! impl_large_numerics {
    ($($t:ty)*) => ($(
        impl EguiStructImut for $t {
            type ConfigTypeImut<'a> = ();
            fn show_primitive_imut(&self, ui: &mut Ui, _config: Self::ConfigTypeImut<'_>, _id: impl Hash + Clone) -> Response {
                ui.label(self.to_string())
            }
        }
        impl EguiStructMut for $t {
            type ConfigTypeMut<'a> = ();
            fn show_primitive_mut(&mut self, ui: &mut Ui, _config: Self::ConfigTypeMut<'_>, _id: impl Hash + Clone)-> Response  {
                let mut text = self.to_string();
                let ret=ui.text_edit_singleline(&mut text);
                if let Ok(value) = text.parse() {
                    *self = value;
                }
                ret
            }
        }
        impl_eeqclone!{$t}
    )*)
}
impl_large_numerics!(i128 u128);

////////////////////////////////////////////////////////////

pub struct Combobox<T>(pub T);

impl<T: ToString> EguiStructImut for Combobox<T> {
    type ConfigTypeImut<'a> = ConfigStrImut;

    fn show_primitive_imut(
        self: &Self,
        ui: &mut Ui,
        config: Self::ConfigTypeImut<'_>,
        _id: impl Hash + Clone,
    ) -> Response {
        self.0.to_string().show_primitive_imut(ui, config, ())
    }
}

impl<T: Clone> EguiStructClone for Combobox<T> {
    fn eguis_clone(&mut self, source: &Self) {
        self.0.clone_from(&source.0)
    }
}
impl<T: PartialEq> EguiStructEq for Combobox<T> {
    fn eguis_eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}
impl<T: Clone + ToString + PartialEq + 'static> EguiStructMut for Combobox<T> {
    type ConfigTypeMut<'a> = Option<&'a mut dyn Iterator<Item = T>>;

    fn show_primitive_mut(
        self: &mut Self,
        ui: &mut Ui,
        config: Self::ConfigTypeMut<'_>,
        id: impl Hash + Clone,
    ) -> Response {
        show_combobox(&mut self.0, ui, config, id)
    }
}

fn show_combobox<'a, T: Clone + ToString + PartialEq>(
    sel: &mut T,
    ui: &mut Ui,
    config: Option<&'a mut dyn Iterator<Item = T>>,
    id: impl Hash + Clone,
) -> Response {
    let defspacing = ui.spacing().item_spacing.clone();
    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
    let mut inner_response = ui.allocate_response(egui::vec2(0.0, 0.0), egui::Sense::hover());
    let ret = egui::ComboBox::from_id_source((id, "__EguiStruct_combobox"))
        .selected_text(sel.to_string())
        .show_ui(ui, |ui| {
            ui.spacing_mut().item_spacing = defspacing;
            if let Some(config) = config {
                for i in config {
                    let s = i.to_string();
                    inner_response |= ui.selectable_value(sel, i, s);
                }
            }
        })
        .response;
    ui.spacing_mut().item_spacing = defspacing;
    ret | inner_response
}
impl<T> Deref for Combobox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Combobox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Default> Default for Combobox<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: Clone> Clone for Combobox<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Copy> Copy for Combobox<T> {}
impl<T: Eq> Eq for Combobox<T> {}
impl<T: Ord> Ord for Combobox<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T: PartialEq> PartialEq for Combobox<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: PartialOrd> PartialOrd for Combobox<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
