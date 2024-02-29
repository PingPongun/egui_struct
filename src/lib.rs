use egui::{Button, Grid, Id, Response, ScrollArea, Ui, Widget, WidgetText};
pub use egui_struct_macros::*;
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

macro_rules! generate_show {
    ($top_name:ident, $collapsing_name:ident, $primitive_name:ident, $childs_name:ident, $typ:ty, $config:ident, $COLUMN_COUNT:ident, $SIMPLE:ident, $has_childs_imut:ident, $has_primitive:ident) => {
        type $config<'a>: Default;
        const $COLUMN_COUNT: usize = 2;
        const $SIMPLE: bool = true;
        fn $has_childs_imut(&self) -> bool {
            false
        }
        fn $has_primitive(&self) -> bool {
            !self.$has_childs_imut()
        }

        fn $top_name(
            self: $typ,
            ui: &mut Ui,
            label: impl Into<WidgetText> + Clone,
            reset2: Option<&Self>,
        ) -> Response
        where
            Self: 'static,
        {
            let label: WidgetText = label.into();
            let id =
                ui.make_persistent_id((label.text().to_string(), std::any::TypeId::of::<Self>()));
            ScrollArea::vertical()
                .show(ui, |ui| {
                    Grid::new(id)
                        .num_columns(Self::$COLUMN_COUNT)
                        .show(ui, |ui| {
                            self.$collapsing_name(ui, label, "", -1, Default::default(), reset2, id)
                        })
                        .inner
                })
                .inner
        }

        fn $collapsing_name(
            self: $typ,
            ui: &mut Ui,
            label: impl Into<WidgetText> + Clone,
            hint: impl Into<WidgetText> + Clone,
            indent_level: isize,
            config: Self::$config<'_>,
            _reset2: Option<&Self>,
            parent_id: Id,
        ) -> Response {
            let mut uncollapsed = true;
            let has_childs_imut = self.$has_childs_imut();
            let id = parent_id.with(label.clone().into().text());
            ui.horizontal(|ui| {
                if indent_level >= 0 {
                    for _ in 0..indent_level {
                        ui.separator();
                    }
                    if has_childs_imut {
                        let id = id.with("__EguiStruct_collapsing_state");
                        uncollapsed = ui.data_mut(|d| d.get_temp_mut_or(id, true).clone());
                        let icon = if uncollapsed { "⏷" } else { "⏵" };
                        if Button::new(icon).frame(false).small().ui(ui).clicked() {
                            ui.data_mut(|d| d.insert_temp(id, !uncollapsed));
                        }
                    }
                }
                let mut lab = ui.label(label.into());
                let hint = hint.into();
                if !hint.is_empty() {
                    lab = lab.on_hover_text(hint);
                }
                lab
            });

            let mut ret = ui
                .horizontal(|ui| {
                    let id = id.with("__EguiStruct_primitive");
                    #[allow(unused_mut)]
                    let mut ret = self.$primitive_name(ui, config, id);
                    macro_rules! reset {
                        (show_collapsing_imut) => {
                            ret
                        };
                        (show_collapsing) => {
                            if let Some(reset2) = _reset2 {
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
                    reset! {$collapsing_name}
                })
                .inner;
            ui.end_row();

            if has_childs_imut && uncollapsed {
                ret = self.$childs_name(ui, indent_level + 1, ret, _reset2, id);
            }
            ret
        }
        fn $primitive_name(
            self: $typ,
            ui: &mut Ui,
            _config: Self::$config<'_>,
            _id: impl Hash + Clone,
        ) -> Response {
            ui.label("")
        }
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
    };
}
pub trait EguiStructClone {
    fn eguis_clone(&mut self, source: &Self);
}
pub trait EguiStructEq {
    fn eguis_eq(&self, _rhs: &Self) -> bool {
        //default implementation can be used if reset button is not required
        true
    }
}
#[macro_export]
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
macro_rules! impl_eeqclone {
    ([$($generics:tt)*], $type:ty) => {
        impl_eeq!{[$($generics)*], $type}
        impl_eclone!{[$($generics)*], $type}
    };
    ($type:ty) => {impl_eeqclone!{[],$type}}
}

pub trait EguiStruct: EguiStructClone + EguiStructEq {
    generate_show! { show_top, show_collapsing, show_primitive, show_childs, &mut Self, ConfigType, COLUMN_COUNT, SIMPLE, has_childs, has_primitive }
}
pub trait EguiStructImut {
    generate_show! { show_top_imut, show_collapsing_imut, show_primitive_imut, show_childs_imut, &Self, ConfigTypeImut, COLUMN_COUNT_IMUT, SIMPLE_IMUT, has_childs_imut, has_primitive_imut }
}

///Config structure for mutable view of Numerics
#[derive(Default)]
pub enum ConfigNum<'a, T: 'a> {
    ///Default: DragValue (without limits)
    #[default]
    NumDefault,

    ///DragValue(min, max)
    DragValue(T, T),

    ///Slider(min, max)
    Slider(T, T),

    ///Slider(min, max, step)
    SliderStep(T, T, T),

    ///Combobox with available options specified by included iterator
    ComboBox(&'a mut dyn Iterator<Item = T>),
}
macro_rules! impl_num_primitives {
    ($($typ:ty)*) => {
        $(
            impl EguiStruct for $typ {
                type ConfigType<'a> = ConfigNum<'a, $typ>;
                fn show_primitive(&mut self, ui: &mut Ui, config: Self::ConfigType<'_>, id: impl Hash  + Clone) -> Response {
                    match config{
                        Self::ConfigType::NumDefault        =>  egui::DragValue::new(self).ui(ui),
                        Self::ConfigType::DragValue(min,max)=>  egui::DragValue::new(self).clamp_range(min..=max).ui(ui),
                        Self::ConfigType::Slider(min,max)   =>  egui::Slider::new(self, min..=max).ui(ui),
                        Self::ConfigType::SliderStep(min,max,step)   =>  egui::Slider::new(self, min..=max).step_by(step as f64).ui(ui),
                        Self::ConfigType::ComboBox(iter) => show_combobox(self, ui, Some(iter), id),
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

impl EguiStruct for bool {
    type ConfigType<'a> = ();
    fn show_primitive(
        &mut self,
        ui: &mut Ui,
        _config: Self::ConfigType<'_>,
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

impl EguiStruct for String {
    type ConfigType<'a> = ConfigStr<'a>;
    fn show_primitive(
        &mut self,
        ui: &mut Ui,
        config: Self::ConfigType<'_>,
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
impl<T: EguiStruct + Default> EguiStruct for Option<T> {
    const SIMPLE: bool = false;
    type ConfigType<'a> = ();
    fn has_childs(&self) -> bool {
        !T::SIMPLE && self.is_some()
    }
    fn has_primitive(&self) -> bool {
        true
    }
    fn show_primitive(
        &mut self,
        ui: &mut Ui,
        _config: Self::ConfigType<'_>,
        id: impl Hash + Clone,
    ) -> Response {
        ui.horizontal(|ui| {
            let mut checked = self.is_some();
            let mut ret = checked.show_primitive(ui, (), ());

            match (checked, T::SIMPLE, self.as_mut()) {
                (true, true, Some(value)) => {
                    ret |= value.show_primitive(ui, Default::default(), id)
                }
                (true, false, Some(_)) => (), //if inner is not simple, it will be shown below
                (true, _, None) => *self = Some(T::default()),
                (false, _, _) => *self = None,
            }
            ret
        })
        .inner
    }
    fn show_childs(
        &mut self,
        ui: &mut Ui,
        indent_level: isize,
        mut response: Response,
        reset2: Option<&Self>,
        id: Id,
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive() {
                response |= inner.show_collapsing(
                    ui,
                    "[0]",
                    "",
                    indent_level,
                    Default::default(),
                    reset2.unwrap_or(&None).as_ref(),
                    id,
                );
            } else {
                response |= inner.show_childs(
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
    ($Self:ty, $typ:ty, $iter:ident, $collapsing_name:ident, $childs_name:ident,$trait:ident, $SIMPLE:ident, $ConfigType:ident, $has_childs_imut:ident, $has_primitive:ident) => {
        impl<T: $trait> $trait for $typ{
            const $SIMPLE: bool = false;
            type $ConfigType<'a> = ();
            fn $has_childs_imut(&self) -> bool {
                !self.is_empty()
            }
            fn $has_primitive(&self) -> bool {
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
        }
    };
    (IMUT, $($typ:ty)*) => { $(impl_vec! {&Self, $typ, iter, show_collapsing_imut, show_childs_imut, EguiStructImut, SIMPLE_IMUT, ConfigTypeImut, has_childs_imut, has_primitive_imut})* };
    ($($typ:ty)*) => {
        $(
            impl_vec! {IMUT, $typ}
            impl_vec! {&mut Self, $typ, iter_mut, show_collapsing, show_childs, EguiStruct, SIMPLE, ConfigType, has_childs, has_primitive}

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
        )*
    };
}

impl_vec! {[T] Vec<T>}
impl_vec! {IMUT, std::collections::HashSet<T> }
#[cfg(feature = "indexmap")]
impl_vec! {IMUT, indexmap::IndexSet<T> }

/////////////////////////////////////////////////
macro_rules! impl_map {
    ($Self:ty, $typ:ty, [$( $Qbound:path),*], $iter:ident, $collapsing_name:ident, $childs_name:ident,$trait:ident, $SIMPLE:ident, $ConfigType:ident, $has_childs_imut:ident, $has_primitive:ident) => {
        impl<Q: ToString $(+ $Qbound)*, V: $trait> $trait for $typ{
            const $SIMPLE: bool = false;
            type $ConfigType<'a> = ();
            fn $has_childs_imut(&self) -> bool {
                !self.is_empty()
            }
            fn $has_primitive(&self) -> bool {
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
        }
    };
    ($typ:ty) => {
        impl_map! {&Self, $typ, [], iter, show_collapsing_imut, show_childs_imut, EguiStructImut, SIMPLE_IMUT, ConfigTypeImut, has_childs_imut, has_primitive_imut}
        impl_map! {&mut Self, $typ, [Eq, std::hash::Hash], iter_mut, show_collapsing, show_childs, EguiStruct, SIMPLE, ConfigType, has_childs, has_primitive}

        impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructClone> EguiStructClone for $typ {
            fn eguis_clone(&mut self, source: &Self) {
                //this is very simplified implementation, that asummes that lenghts & keys are the same
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
        impl EguiStruct for $t {
            type ConfigType<'a> = ();
            fn show_primitive(&mut self, ui: &mut Ui, _config: Self::ConfigType<'_>, _id: impl Hash + Clone)-> Response  {
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
impl<T: Clone + ToString + PartialEq + 'static> EguiStruct for Combobox<T> {
    type ConfigType<'a> = Option<&'a mut dyn Iterator<Item = T>>;

    fn show_primitive(
        self: &mut Self,
        ui: &mut Ui,
        config: Self::ConfigType<'_>,
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
