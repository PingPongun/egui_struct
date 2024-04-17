use crate::traits::*;
use crate::types::combobox::show_combobox;
use crate::types::*;
use crate::*;
use egui::{Id, Response, Ui, Widget};
use std::hash::Hash;

pub mod macros {
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
}

mod impl_numerics {
    use super::*;
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
}
/////////////////////////////////////////////////////////
mod impl_str {
    use super::*;
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
}
/////////////////////////////////////////////////////////
mod impl_option {
    use super::*;
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
                    (true, Some(value)) => {
                        ret |= value.show_primitive_imut(ui, Default::default(), id)
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
    // impl<T: EguiStructResetable + Default> EguiStructResetable for Option<T>
    // where
    //     <T as EguiStructResetable>::Reset2: Sized,
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
}
///////////////////////////////////////////////////
mod impl_sets {
    use super::*;
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

        // impl<T: EguiStructResetable> EguiStructResetable for $typ
        // where
        //     <T as EguiStructResetable>::Reset2: Sized,
        // {
        //     type Reset2= $restyp;
        //     fn reset2(&mut self, source: &Self::Reset2) {
        //         //TODO update this if vector length can change
        //         self.iter_mut().zip(source.iter()).for_each(|(s,r)|s.reset2(r))
        //     }
        //     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
        //         let mut ret = self.len()==rhs.len();
        //         self.iter().zip(rhs.iter()).for_each(|(s,r)|ret &= s.reset_possible(r));
        //         ret
        //     }
        // }
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

    // impl_vec! {[T], Box<[T::Reset2]>}
    // impl_vec! {Vec<T>, Vec<T::Reset2>}
    impl_vec! {[T]}
    impl_vec! {Vec<T>}
    impl_vec! {IMUT, std::collections::HashSet<T> }
    #[cfg(feature = "indexmap")]
    impl_vec! {IMUT, indexmap::IndexSet<T> }
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

            // impl<Q: ToString + Eq + std::hash::Hash, V: EguiStructResetable> EguiStructResetable for $typ
            // where
            //     <V as EguiStructResetable>::Reset2: Sized,
            // {
            //     type Reset2= $restyp;
            //     fn reset2(&mut self, source: &Self::Reset2) {
            //         //this is very simplified implementation, that assumes that lenghts & keys are the same
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

    // impl_map! { std::collections::HashMap<Q,V>, std::collections::HashMap<Q,V::Reset2> }
    impl_map! { std::collections::HashMap<Q,V> }
    #[cfg(feature = "indexmap")]
    impl_map! { indexmap::IndexMap<Q,V>> }
    // impl_map! { indexmap::IndexMap<Q,V>, indexmap::IndexMap<Q,V::Reset2> }
}
