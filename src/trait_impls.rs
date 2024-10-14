use crate::config::*;
use crate::traits::*;
use crate::wrappers::combobox::show_combobox;
use crate::*;
use egui::Response;
use egui::Widget;
use exgrid::ExUi;

pub mod macros {
    #[cfg(doc)]
    pub use super::*;

    #[macro_export]
    /// Generate [EguiStructClone] implementation based on [Clone]
    macro_rules! impl_eclone {
    ([$($generics:tt)*], $type:ty) => {
        impl<$($generics)*> EguiStructClone for $type {
            fn eguis_clone(&mut self, source: &Self) {
                self.clone_from(source);
            }
            fn eguis_clone_full(&self)->Option<Self> {
                Some(self.clone())
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
}

mod impl_numerics {
    use super::*;
    macro_rules! impl_num_primitives {
    ($($typ:ty)*) => {
        $(
            impl EguiStructMut for $typ {
                type ConfigTypeMut<'a> = ConfigNum<'a, $typ>;
                fn show_primitive_mut(&mut self, ui: &mut ExUi, config: &Self::ConfigTypeMut<'_>) -> Response {
                    match config{
                        Self::ConfigTypeMut::NumDefault        =>  egui::DragValue::new(self).ui(ui),
                        #[cfg(any(feature = "egui28", feature = "egui29"))]
                        Self::ConfigTypeMut::DragValue(min,max)=>  egui::DragValue::new(self).range(*min..=*max).ui(ui),
                        #[cfg(not(any(feature = "egui28", feature = "egui29")))]
                        Self::ConfigTypeMut::DragValue(min,max)=>  egui::DragValue::new(self).clamp_range(*min..=*max).ui(ui),
                        Self::ConfigTypeMut::Slider(min,max)   =>  egui::Slider::new(self, *min..=*max).ui(ui),
                        Self::ConfigTypeMut::SliderStep(min,max,step)   =>  egui::Slider::new(self, *min..=*max).step_by(*step as f64).ui(ui),
                        Self::ConfigTypeMut::ComboBox(iter) => show_combobox(self, ui, &Some(*iter)),
                    }
                }
            }
            impl EguiStructImut for $typ {
                type ConfigTypeImut<'a> = ConfigStrImut;
                fn show_primitive_imut(&self, ui: &mut ExUi, config: &Self::ConfigTypeImut<'_>) -> Response {
                    self.to_string().as_str().show_primitive_imut(ui, config)
                }
            }
            impl_eeqclone!{$typ}
        )*
    };
}

    impl_num_primitives!(i8 i16 i32 i64 u8 u16 u32 u64 usize isize f32 f64);

    macro_rules! impl_large_numerics {
    ($($t:ty)*) => ($(
        impl EguiStructImut for $t {
            type ConfigTypeImut<'a> = ();
            fn show_primitive_imut(&self, ui: &mut ExUi, _config: &Self::ConfigTypeImut<'_>) -> Response {
                ui.label(self.to_string())
            }
        }
        impl EguiStructMut for $t {
            type ConfigTypeMut<'a> = ();
            fn show_primitive_mut(&mut self, ui: &mut ExUi, _config: &Self::ConfigTypeMut<'_>)-> Response  {
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

    impl EguiStructMut for bool {
        type ConfigTypeMut<'a> = ();
        fn show_primitive_mut(
            &mut self,
            ui: &mut ExUi,
            _config: &Self::ConfigTypeMut<'_>,
        ) -> Response {
            egui::Checkbox::without_text(self).ui(ui)
        }
    }
    impl EguiStructImut for bool {
        type ConfigTypeImut<'a> = ();
        fn show_primitive_imut(
            &self,
            ui: &mut ExUi,
            _config: &Self::ConfigTypeImut<'_>,
        ) -> Response {
            ui.add_enabled(false, egui::Checkbox::without_text(&mut self.clone()))
        }
    }
    impl_eeqclone! {bool}
}
/////////////////////////////////////////////////////////
mod impl_str {
    use super::*;
    impl EguiStructMut for String {
        type ConfigTypeMut<'a> = ConfigStr<'a>;
        fn show_primitive_mut(
            &mut self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeMut<'_>,
        ) -> Response {
            match config {
                ConfigStr::SingleLine => ui.text_edit_singleline(self),
                ConfigStr::MultiLine => ui.text_edit_multiline(self),
                ConfigStr::ComboBox(iter) => show_combobox(self, ui, &Some(*iter)),
            }
        }
    }
    impl EguiStructImut for String {
        type ConfigTypeImut<'a> = ConfigStrImut;
        fn show_primitive_imut(
            &self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeImut<'_>,
        ) -> Response {
            self.as_str().show_primitive_imut(ui, config)
        }
    }
    impl_eeqclone! {String}

    impl EguiStructImut for str {
        type ConfigTypeImut<'a> = ConfigStrImut;
        fn show_primitive_imut(
            mut self: &Self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeImut<'_>,
        ) -> Response {
            match config {
                ConfigStrImut::NonSelectable => ui.label(self),
                ConfigStrImut::Selectable => ui.text_edit_singleline(&mut self),
            }
        }
    }
}
/////////////////////////////////////////////////////////
mod impl_option {
    use super::*;
    impl<T: EguiStructImut + Default> EguiStructImut for Option<T> {
        const SIMPLE_IMUT: bool = false;
        type ConfigTypeImut<'a> = T::ConfigTypeImut<'a>;
        fn has_childs_imut(&self) -> bool {
            !T::SIMPLE_IMUT && self.is_some()
        }
        fn has_primitive_imut(&self) -> bool {
            true
        }
        fn show_primitive_imut(
            &self,
            ui: &mut ExUi,
            _config: &Self::ConfigTypeImut<'_>,
        ) -> Response {
            ui.horizontal(|ui| {
                let mut ret = self.is_some().show_primitive_imut(&mut ui.into(), &mut ());
                match (T::SIMPLE_IMUT, self) {
                    (true, Some(value)) => {
                        ret |= value.show_primitive_imut(&mut ui.into(), &mut Default::default())
                    }
                    (true, None) => (),
                    (false, _) => (),
                }
                ret
            })
            .inner
        }
        fn show_childs_imut(
            &self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeImut<'_>,
            _reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.interact(
                egui::Rect::NOTHING,
                "dummy".into(),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            if let Some(inner) = self {
                if inner.has_primitive_imut() {
                    response |= inner.show_collapsing_imut(ui, "[0]", "", config, None, None);
                } else {
                    response |= inner.show_childs_imut(ui, config, None)
                }
            }
            response
        }
    }
    impl<T: EguiStructMut + Default> EguiStructMut for Option<T> {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = T::ConfigTypeMut<'a>;
        fn has_childs_mut(&self) -> bool {
            !T::SIMPLE_MUT && self.is_some()
        }
        fn has_primitive_mut(&self) -> bool {
            true
        }
        fn show_primitive_mut(
            &mut self,
            ui: &mut ExUi,
            _config: &Self::ConfigTypeMut<'_>,
        ) -> Response {
            ui.horizontal(|ui| {
                let mut checked = self.is_some();
                let mut ret = checked.show_primitive_mut(&mut ui.into(), &mut ());

                match (checked, T::SIMPLE_MUT, self.as_mut()) {
                    (true, true, Some(value)) => {
                        ret |= value.show_primitive_mut(&mut ui.into(), &mut Default::default())
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
            ui: &mut ExUi,
            config: &Self::ConfigTypeMut<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.interact(
                egui::Rect::NOTHING,
                "dummy".into(),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            if let Some(inner) = self {
                if inner.has_primitive_mut() {
                    response |= inner.show_collapsing_mut(
                        ui,
                        "[0]",
                        "",
                        config,
                        reset2.unwrap_or(&None).as_ref(),
                        None,
                    );
                } else {
                    response |= inner.show_childs_mut(ui, config, reset2.unwrap_or(&None).as_ref())
                }
            }
            response
        }
    }
    // impl<T: EguiStructResettable + Default> EguiStructResettable for Option<T>
    // where
    //     <T as EguiStructResettable>::Reset2: Sized,
    // {
    //     type Reset2 = Option<T::Reset2>;

    //     fn reset2(&mut self, source: &Self::Reset2) {
    //         if let Some(source) = source {
    //             if let Some(s) = self {
    //                 s.reset2(source);
    //             } else {
    //                 let mut v: T = Default::default();
    //                 v.reset2(source);
    //                 *self = Some(v);
    //             }
    //         } else {
    //             *self = None;
    //         }
    //     }

    //     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
    //         if let Some(s) = self {
    //             if let Some(r) = rhs {
    //                 s.reset_possible(r)
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         }
    //     }
    // }
    impl<T: EguiStructClone + Default> EguiStructClone for Option<T> {
        fn eguis_clone(&mut self, source: &Self) {
            if let Some(source) = source {
                if let Some(s) = self {
                    s.eguis_clone(source);
                } else {
                    //TODO ? use eguis_clone_full here?
                    let mut v: T = Default::default();
                    v.eguis_clone(source);
                    *self = Some(v);
                }
            } else {
                *self = None;
            }
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            if let Some(s) = self {
                s.eguis_clone_full().map(|x| Some(x))
            } else {
                Some(None)
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
}
/////////////////////////////////////////////////
mod impl_maps {
    use super::*;
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
                ui: &mut ExUi,
                config: &Self::$ConfigTypeMut<'_>,
                _reset2: Option<&Self>,
            ) -> Response {
                let mut response = ui.interact(
                    egui::Rect::NOTHING,
                    "dummy".into(),
                    egui::Sense {
                        click: false,
                        drag: false,
                        focusable: false,
                    },
                );

                self.$iter().for_each(|(q, v)| {
                    response |= v.$collapsing_name(
                        ui,
                        q.to_string(),
                        "",
                        &mut Default::default(),
                        None, None,
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

            // impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructResettable> EguiStructResettable for $typ
            // where
            //     <V as EguiStructResettable>::Reset2: Sized,
            // {
            //     type Reset2= $restyp;
            //     fn reset2(&mut self, source: &Self::Reset2) {
            //         //this is very simplified implementation, that assumes that lengths & keys are the same
            //         self.iter_mut().for_each(|(q, v)| {
            //             if let Some(r) = source.get(q) {
            //                 v.reset2(r)
            //             }
            //         })
            //     }
            //     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
            //         let mut ret = self.len() == rhs.len();
            //         self.iter().for_each(|(q, v)| {
            //             if let Some(r) = rhs.get(q) {
            //                 ret &= v.reset_possible(r)
            //             } else {
            //                 ret = false
            //             }
            //         });
            //         ret
            //     }
            // }
        impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructClone> EguiStructClone for $typ {
            fn eguis_clone(&mut self, source: &Self) {
                //this is very simplified implementation, that assumes that lengths & keys are the same
                self.iter_mut().for_each(|(q, v)| {
                    if let Some(r) = source.get(q) {
                        v.eguis_clone(r)
                    }
                })
            }
            //TODO
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

    // impl_map! { std::collections::HashMap<Q,V>, std::collections::HashMap<Q,V::Reset2> }
    impl_map! { std::collections::HashMap<Q,V> }
}

impl EguiStructMut for exgrid::GridMode {
    type ConfigTypeMut<'a> = ();

    fn show_primitive_mut(
        self: &mut Self,
        ui: &mut ExUi,
        _config: &Self::ConfigTypeMut<'_>,
    ) -> Response {
        let isgrid = *self == Self::Traditional;
        ui.keep_cell_start();
        let grs = ui.selectable_label(isgrid, "Grid");
        let crs = ui.selectable_label(!isgrid, "Compact");
        ui.keep_cell_stop();
        if grs.clicked() {
            *self = Self::Traditional
        }
        if crs.clicked() {
            *self = Self::CompactWidth
        }
        grs | crs
    }
    // fn preview_str_mut<'b>(&'b self) -> &'b str {
    //     ""
    // }
}
impl_eeqclone! {exgrid::GridMode}

impl EguiStructMut for () {
    type ConfigTypeMut<'a> = ();
}
impl EguiStructImut for () {
    type ConfigTypeImut<'a> = ();
}
impl_eeqclone! {()}
