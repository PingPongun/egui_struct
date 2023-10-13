use egui::{Button, Grid, Response, ScrollArea, Ui, Widget, WidgetText};

pub use egui_struct_macros::*;
macro_rules! generate_show {
    ($top_name:ident, $collapsing_name:ident, $primitive_name:ident, $childs_name:ident, $typ:ty) => {
        fn $top_name(self: $typ, ui: &mut Ui, label: impl Into<WidgetText> + Clone) -> Response {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Grid::new(ui.next_auto_id())
                        .striped(true)
                        .num_columns(Self::COLUMN_COUNT)
                        .show(ui, |ui| self.$collapsing_name(ui, label, "", -1))
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
            let mut ret = self.$primitive_name(ui);
            ui.end_row();

            if has_childs && uncollapsed {
                ret = self.$childs_name(ui, indent_level + 1, ret);
            }
            ret
        }
        fn $primitive_name(self: $typ, ui: &mut Ui) -> Response {
            ui.label("")
        }
        fn $childs_name(
            self: $typ,
            _ui: &mut Ui,
            _indent_level: isize,
            _response: Response,
        ) -> Response {
            unreachable!()
        }
    };
}
pub trait EguiStruct: EguiStructImut {
    generate_show! {show_top_mut, show_collapsing_mut, show_primitive_mut, show_childs_mut, &mut Self}
}
pub trait EguiStructImut {
    const COLUMN_COUNT: usize = 2;
    const SIMPLE: bool = true;
    fn has_childs(&self) -> bool {
        false
    }
    fn has_primitive(&self) -> bool {
        !self.has_childs()
    }
    generate_show! {show_top, show_collapsing, show_primitive, show_childs, &Self}
}

macro_rules! impl_num_primitives {
    ($($typ:ty)*) => {
        $(
            impl EguiStruct for $typ {
                fn show_primitive_mut(&mut self, ui: &mut Ui) -> Response {
                    egui::DragValue::new(self).ui(ui)
                }
            }
            impl EguiStructImut for $typ {
                fn show_primitive(&self, ui: &mut Ui) -> Response {
                    ui.label(self.to_string())
                }
            }
        )*
    };
}

impl_num_primitives!(i8 i16 i32 i64 u8 u16 u32 u64 usize isize f32 f64);

impl EguiStruct for bool {
    fn show_primitive_mut(&mut self, ui: &mut Ui) -> Response {
        egui::Checkbox::without_text(self).ui(ui)
    }
}
impl EguiStructImut for bool {
    fn show_primitive(&self, ui: &mut Ui) -> Response {
        ui.add_enabled(false, egui::Checkbox::without_text(&mut self.clone()))
    }
}
/////////////////////////////////////////////////////////

impl EguiStruct for String {
    fn show_primitive_mut(&mut self, ui: &mut Ui) -> Response {
        ui.text_edit_singleline(self)
    }
}
impl EguiStructImut for String {
    fn show_primitive(&self, ui: &mut Ui) -> Response {
        ui.label(self)
    }
}

impl EguiStructImut for str {
    fn show_primitive(&self, ui: &mut Ui) -> Response {
        ui.label(self)
    }
}

/////////////////////////////////////////////////////////
impl<T: EguiStructImut + Default> EguiStructImut for Option<T> {
    const SIMPLE: bool = false;
    fn has_childs(&self) -> bool {
        !T::SIMPLE && self.is_some()
    }
    fn has_primitive(&self) -> bool {
        true
    }
    fn show_primitive(&self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let mut ret = self.is_some().show_primitive(ui);
            match (T::SIMPLE, self) {
                (true, Some(value)) => ret |= value.show_primitive(ui),
                (true, None) => (),
                (false, _) => (),
            }
            ret
        })
        .inner
    }
    fn show_childs(&self, ui: &mut Ui, indent_level: isize, mut response: Response) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive() {
                response |= inner.show_collapsing(ui, "[0]", "", indent_level);
            } else {
                response |= inner.show_childs(ui, indent_level, response.clone())
            }
        }
        response
    }
}
impl<T: EguiStruct + Default> EguiStruct for Option<T> {
    fn show_primitive_mut(&mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let mut checked = self.is_some();
            let mut ret = checked.show_primitive_mut(ui);

            match (checked, T::SIMPLE, self.as_mut()) {
                (true, true, Some(value)) => ret |= value.show_primitive_mut(ui),
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
    ) -> Response {
        if let Some(inner) = self {
            if inner.has_primitive() {
                response |= inner.show_collapsing_mut(ui, "[0]", "", indent_level);
            } else {
                response |= inner.show_childs_mut(ui, indent_level, response.clone())
            }
        }
        response
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
        ) -> Response {
            self.$iter().enumerate().for_each(|(idx, x)| {
                response |= x.$collapsing_name(ui, idx.to_string(), "", indent_level)
            });
            response
        }
    };
    (IMUT, $typ:ty) => {
        impl<T: EguiStructImut> EguiStructImut for $typ{
            const SIMPLE: bool = false;
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
                impl_vec! {&mut Self, iter_mut, show_collapsing_mut, show_childs_mut}
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
        ) -> Response {
            self.$iter().for_each(|(q, v)| {
                response |= v.$collapsing_name(ui, q.to_string(), "", indent_level)
            });
            response
        }
    };
    ($typ:ty) => {
        impl<Q: ToString, V: EguiStructImut> EguiStructImut for $typ {
            const SIMPLE: bool = false;
            fn has_childs(&self) -> bool {
                !self.is_empty()
            }
            fn has_primitive(&self) -> bool {
                false
            }
            impl_map! {&Self, iter, show_collapsing, show_childs}
        }
        impl<Q: ToString, V: EguiStruct> EguiStruct for $typ {
            impl_map! {&mut Self, iter_mut, show_collapsing_mut, show_childs_mut}
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
            fn show_primitive(&self, ui: &mut Ui) -> Response {
                ui.label(self.to_string())
            }
        }
        impl EguiStruct for $t {
            fn show_primitive_mut(&mut self, ui: &mut Ui)-> Response  {
                let mut text = self.to_string();
                let ret=ui.text_edit_singleline(&mut text);
                if let Ok(value) = text.parse() {
                    *self = value;
                }
                ret
            }
        }
    )*)
}
impl_large_numerics!(i128 u128);
///////////////////////////////////////////////////////////

#[derive(Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RangedVal<const MIN: isize, const MAX: isize>(pub isize);

impl<const MIN: isize, const MAX: isize> EguiStructImut for RangedVal<MIN, MAX> {
    fn show_primitive(&self, ui: &mut Ui) -> Response {
        self.0.show_primitive(ui)
    }
}
impl<const MIN: isize, const MAX: isize> EguiStruct for RangedVal<MIN, MAX> {
    fn show_primitive_mut(&mut self, ui: &mut Ui) -> Response {
        egui::DragValue::new(&mut self.0)
            .clamp_range(MIN..=MAX)
            .ui(ui)
    }
}
