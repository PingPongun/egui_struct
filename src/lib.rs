use egui::{Button, Grid, Response, ScrollArea, Ui, Widget, WidgetText};
pub use egui_struct_macros::*;

macro_rules! generate_show {
    ($top_name:ident, $collapsing_name:ident, $primitive_name:ident, $childs_name:ident, $typ:ty, $config:ident) => {
        fn $top_name(
            self: $typ,
            ui: &mut Ui,
            label: impl Into<WidgetText> + Clone,
            reset2: Option<&Self>,
        ) -> Response {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Grid::new(ui.next_auto_id())
                        .striped(true)
                        .num_columns(Self::COLUMN_COUNT)
                        .show(ui, |ui| {
                            self.$collapsing_name(ui, label, "", -1, Default::default(), reset2)
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
            config: Self::$config,
            _reset2: Option<&Self>,
        ) -> Response {
            let mut uncollapsed = true;
            let has_childs = self.has_childs();
            ui.horizontal(|ui| {
                if indent_level >= 0 {
                    for _ in 0..indent_level {
                        ui.separator();
                    }
                    if has_childs {
                        let id =
                            ui.make_persistent_id(self as *const Self as *const usize as usize);
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
                    #[allow(unused_mut)]
                    let mut ret = self.$primitive_name(ui, config);
                    macro_rules! reset {
                        (show_collapsing) => {
                            ret
                        };
                        (show_collapsing_mut) => {
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

            if has_childs && uncollapsed {
                ret = self.$childs_name(ui, indent_level + 1, ret, _reset2);
            }
            ret
        }
        fn $primitive_name(self: $typ, ui: &mut Ui, _config: Self::$config) -> Response {
            ui.label("")
        }
        fn $childs_name(
            self: $typ,
            _ui: &mut Ui,
            _indent_level: isize,
            _response: Response,
            _reset2: Option<&Self>,
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
macro_rules! impl_eclone {
    ([$($generics:tt)*], $type:ty) => {
        impl<$($generics)*> EguiStructClone for $type {
            fn eguis_clone(&mut self, source: &Self) {
                self.clone_from(source);
            }
        }
    };
}
macro_rules! impl_eeq {
    ([$($generics:tt)*], $type:ty) => {
        impl<$($generics)*> EguiStructEq for $type {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                self == rhs
            }
        }
    };
}
macro_rules! eimpl {
    ([$($generics:tt)*], $type:ty) => {
        impl_eeq!{[$($generics)*], $type}
        impl_eclone!{[$($generics)*], $type}
    };
}

pub trait EguiStruct: EguiStructImut + EguiStructClone + EguiStructEq {
    type ConfigType: Default;
    generate_show! { show_top_mut, show_collapsing_mut, show_primitive_mut, show_childs_mut, &mut Self, ConfigType}
}
pub trait EguiStructImut {
    type ConfigTypeImut: Default;
    const COLUMN_COUNT: usize = 2;
    const SIMPLE: bool = true;
    fn has_childs(&self) -> bool {
        false
    }
    fn has_primitive(&self) -> bool {
        !self.has_childs()
    }
    generate_show! { show_top, show_collapsing, show_primitive, show_childs, &Self, ConfigTypeImut }
}
#[derive(Default)]
pub enum ConfigNum<T> {
    ///Default: DragValue (without limits)
    #[default]
    NumDefault,
    ///DragValue(min, max)
    DragValue(T, T),
    ///Slider(min, max)
    Slider(T, T),
}
macro_rules! impl_num_primitives {
    ($($typ:ty)*) => {
        $(
            impl EguiStruct for $typ {
                type ConfigType = ConfigNum<$typ>;
                fn show_primitive_mut(&mut self, ui: &mut Ui, config: Self::ConfigType) -> Response {
                    match config{
                        Self::ConfigType::NumDefault        =>  egui::DragValue::new(self).ui(ui),
                        Self::ConfigType::DragValue(min,max)=>  egui::DragValue::new(self).clamp_range(min..=max).ui(ui),
                        Self::ConfigType::Slider(min,max)   =>  egui::Slider::new(self, min..=max).ui(ui),
                    }
                }
            }
            impl EguiStructImut for $typ {
                type ConfigTypeImut = ();
                fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
                    ui.label(self.to_string())
                }
            }
            eimpl!{[],$typ}
        )*
    };
}

impl_num_primitives!(i8 i16 i32 i64 u8 u16 u32 u64 usize isize f32 f64);

impl EguiStruct for bool {
    type ConfigType = ();
    fn show_primitive_mut(&mut self, ui: &mut Ui, _config: Self::ConfigType) -> Response {
        egui::Checkbox::without_text(self).ui(ui)
    }
}
impl EguiStructImut for bool {
    type ConfigTypeImut = ();
    fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
        ui.add_enabled(false, egui::Checkbox::without_text(&mut self.clone()))
    }
}
eimpl! {[],bool}
/////////////////////////////////////////////////////////

impl EguiStruct for String {
    type ConfigType = ();
    fn show_primitive_mut(&mut self, ui: &mut Ui, _config: Self::ConfigType) -> Response {
        ui.text_edit_singleline(self)
    }
}
impl EguiStructImut for String {
    type ConfigTypeImut = ();
    fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
        ui.label(self)
    }
}
eimpl! {[],String}

impl EguiStructImut for str {
    type ConfigTypeImut = ();
    fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
        ui.label(self)
    }
}

/////////////////////////////////////////////////////////
impl<T: EguiStructImut + Default> EguiStructImut for Option<T> {
    const SIMPLE: bool = false;
    type ConfigTypeImut = ();
    fn has_childs(&self) -> bool {
        !T::SIMPLE && self.is_some()
    }
    fn has_primitive(&self) -> bool {
        true
    }
    fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
        ui.horizontal(|ui| {
            let mut ret = self.is_some().show_primitive(ui, ());
            match (T::SIMPLE, self) {
                (true, Some(value)) => ret |= value.show_primitive(ui, Default::default()),
                (true, None) => (),
                (false, _) => (),
            }
            ret
        })
        .inner
    }
    fn show_childs(
        &self,
        ui: &mut Ui,
        indent_level: isize,
        mut response: Response,
        _reset2: Option<&Self>,
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive() {
                response |=
                    inner.show_collapsing(ui, "[0]", "", indent_level, Default::default(), None);
            } else {
                response |= inner.show_childs(ui, indent_level, response.clone(), None)
            }
        }
        response
    }
}
impl<T: EguiStruct + Default> EguiStruct for Option<T> {
    type ConfigType = ();
    fn show_primitive_mut(&mut self, ui: &mut Ui, _config: Self::ConfigType) -> Response {
        ui.horizontal(|ui| {
            let mut checked = self.is_some();
            let mut ret = checked.show_primitive_mut(ui, ());

            match (checked, T::SIMPLE, self.as_mut()) {
                (true, true, Some(value)) => {
                    ret |= value.show_primitive_mut(ui, Default::default())
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
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive() {
                response |= inner.show_collapsing_mut(
                    ui,
                    "[0]",
                    "",
                    indent_level,
                    Default::default(),
                    reset2.unwrap_or(&None).as_ref(),
                );
            } else {
                response |= inner.show_childs_mut(
                    ui,
                    indent_level,
                    response.clone(),
                    reset2.unwrap_or(&None).as_ref(),
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
    ($typ:ty,$iter:ident, $collapsing_name:ident, $childs_name:ident) => {
        fn $childs_name(
            self: $typ,
            ui: &mut Ui,
            indent_level: isize,
            mut response: Response,
            _reset2: Option<&Self>,
        ) -> Response {
            self.$iter().enumerate().for_each(|(idx, x)| {
                response |= x.$collapsing_name(ui, idx.to_string(), "", indent_level, Default::default(), None)
            });
            response
        }
    };
    (IMUT, $typ:ty) => {
        impl<T: EguiStructImut> EguiStructImut for $typ{
            const SIMPLE: bool = false;
            type ConfigTypeImut = ();
            fn has_childs(&self) -> bool {
                !self.is_empty()
            }
            fn has_primitive(&self) -> bool {
                false
            }
            impl_vec! {&Self, iter, show_collapsing, show_childs}
        }
    };
    ($($typ:ty)*) => {
        $(
            impl_vec!{IMUT, $typ}
            impl<T: EguiStruct> EguiStruct for $typ {
                type ConfigType = ();
                impl_vec! {&mut Self, iter_mut, show_collapsing_mut, show_childs_mut}
            }
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
    ($typ:ty,$iter:ident, $collapsing_name:ident, $childs_name:ident) => {
        fn $childs_name(
            self: $typ,
            ui: &mut Ui,
            indent_level: isize,
            mut response: Response,
            _reset2: Option<&Self>,
        ) -> Response {
            self.$iter().for_each(|(q, v)| {
                response |= v.$collapsing_name(
                    ui,
                    q.to_string(),
                    "",
                    indent_level,
                    Default::default(),
                    None,
                )
            });
            response
        }
    };
    ($typ:ty) => {
        impl<Q: ToString, V: EguiStructImut> EguiStructImut for $typ {
            const SIMPLE: bool = false;
            type ConfigTypeImut = ();
            fn has_childs(&self) -> bool {
                !self.is_empty()
            }
            fn has_primitive(&self) -> bool {
                false
            }
            impl_map! {&Self, iter, show_collapsing, show_childs}
        }
        impl<
                Q: ToString + Eq + std::hash::Hash,
                V: EguiStruct + EguiStructClone + EguiStructEq,
            > EguiStruct for $typ
        {
            type ConfigType = ();
            impl_map! {&mut Self, iter_mut, show_collapsing_mut, show_childs_mut}
        }
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
            type ConfigTypeImut = ();
            fn show_primitive(&self, ui: &mut Ui, _config: Self::ConfigTypeImut) -> Response {
                ui.label(self.to_string())
            }
        }
        impl EguiStruct for $t {
            type ConfigType = ();
            fn show_primitive_mut(&mut self, ui: &mut Ui, _config: Self::ConfigType)-> Response  {
                let mut text = self.to_string();
                let ret=ui.text_edit_singleline(&mut text);
                if let Ok(value) = text.parse() {
                    *self = value;
                }
                ret
            }
        }
        eimpl!{[],$t}
    )*)
}
impl_large_numerics!(i128 u128);
