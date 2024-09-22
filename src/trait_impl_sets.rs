use crate::traits::*;
use crate::types::*;
use crate::*;
use egui::Response;
use exgrid::ExUi;

use set::*;
use std::any::Any;

mod impl_sets_imut {
    use super::*;
    macro_rules! impl_set_imut {
        ( $typ:ty ) => {
            impl<T: EguiStructImut> EguiStructImut for $typ {
                const SIMPLE_IMUT: bool = false;
                type ConfigTypeImut<'a> = T::ConfigTypeImut<'a>;
                fn has_childs_imut(&self) -> bool {
                    !self.is_empty()
                }
                fn has_primitive_imut(&self) -> bool {
                    false
                }
                fn show_childs_imut(
                    &self,
                    ui: &mut ExUi,
                    config: &mut Self::ConfigTypeImut<'_>,
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
                    self.iter().enumerate().for_each(|(idx, x)| {
                        response |=
                            x.show_collapsing_imut(ui, idx.to_string(), "", config, None, None)
                    });
                    response
                }
                fn start_collapsed_imut(&self) -> bool {
                    self.len() > 16
                }
            }
        };
    }
    impl_set_imut! {Vec<T> }
    impl_set_imut! {std::collections::HashSet<T>}
    #[cfg(feature = "indexmap")]
    impl_set_imut! {indexmap::IndexSet<T> }
    impl_set_imut! {[T]}
}
mod hashset {
    use super::*;
    macro_rules! impl_set {
    ($typ:ty, $impl:ident, $ConfigType:ty, [$( $bound:path),*]) => {

        impl<T: 'static+EguiStructMut $(+ $bound)*> EguiStructMut for $typ{
            const SIMPLE_MUT: bool = false;
            type ConfigTypeMut<'a> = ConfigSetMut<'a, T, bool>;
            fn has_childs_mut(&self) -> bool {
                !self.is_empty()
            }
            fn has_primitive_mut(&self) -> bool {
                false
            }
            fn show_childs_mut(
                &mut self,
                ui: &mut ExUi,
                config: &mut Self::ConfigTypeMut<'_>,
                reset2: Option<&Self>,
            ) -> Response {
                let mut response = ui.dummy_response();
                macro_rules! show{
                    (HASHSET)=>{
                        self.iter().enumerate().for_each(|(idx, x)| {
                            response |= x.show_collapsing_imut(ui, idx.to_string(), "", &mut Default::default(), None, None)
                        });
                    };
                    (INDEXSET)=>{
                        // Below allows to mutate set elements, but:
                        // - HashSet: at each frame element order is changed, which makes it unusable
                        // - IndexSet: Set may deduplicate during editing
                        *self = self
                            .drain(..)
                            .enumerate()
                            .map(|(idx, mut x)| {
                                response |= x.show_collapsing_mut(ui, idx.to_string(), "", &mut config.inner_config, None, None);
                                x
                            })
                            .collect()
                    };
                    (VEC)=>{
                        self.iter_mut().enumerate().for_each(|(idx, x)| {
                            response |= x.show_collapsing_mut(ui, idx.to_string(), "", &mut config.inner_config, None, None)
                        });
                    };
                }
                show!($impl);

                // if let Some(add)=config.expandable{
                //     let mut new_val=(add.default)();
                //     let has_childs = new_val.has_childs_mut();
                //     let header = |ui: &mut ExUi| {
                //         let bresp=ui.button("+");
                //         response|=bresp;
                //         if bresp.clicked(){
                //             self.insert(new_val);
                //         }
                //         crate::trait_implementor_set::primitive_w_reset(&mut new_val, ui, &mut config.inner_config, todo)
                //     };
                //     response|=ui.maybe_collapsing_rows(has_childs, header)
                //         .body_simple(|ui| new_val.show_childs_mut(ui, &mut config.inner_config, todo));
                // }

                response
            }
            fn start_collapsed_mut(&self) -> bool {
                self.len() > 16
            }
        }

        impl<T: EguiStructEq> EguiStructEq for $typ  {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                //TODO allow mismatched order for std::HashMap
                let mut ret = self.len()==rhs.len();
                self.iter().zip(rhs.iter()).for_each(|(s,r)|ret &= s.eguis_eq(r));
                ret
            }
        }
    };
}
    impl_set! {std::collections::HashSet<T>, HASHSET, ConfigSetMut<T>,[Eq, std::hash::Hash, EguiStructImut] }
    #[cfg(feature = "indexmap")]
    impl_set! {indexmap::IndexSet<T>, INDEXSET, ConfigSetMut<T>,[Eq, std::hash::Hash]}
    impl<T: EguiStructClone + Eq + std::hash::Hash> EguiStructClone for std::collections::HashSet<T> {
        fn eguis_clone(&mut self, source: &Self) {
            let src: Vec<_> = source.iter().collect();
            *self = self
                .drain()
                .zip(src.iter())
                .map(|(mut s, r)| {
                    s.eguis_clone(r);
                    s
                })
                .collect();
            for i in self.len()..source.len() {
                if let Some(val) = src[i - 1].eguis_clone_full() {
                    self.insert(val);
                }
            }
        }
        fn eguis_clone_full(&self) -> Option<Self> {
            if self.len() == 0 {
                return Some(Self::new());
            }
            let mut cloned: Vec<_> = self.iter().map(|s| s.eguis_clone_full()).collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(cloned.into_iter().map(|x| x.unwrap()).collect())
            }
        }
    }
    #[cfg(feature = "indexmap")]
    impl<T: EguiStructClone + Eq + std::hash::Hash> EguiStructClone for indexmap::IndexSet<T> {
        fn eguis_clone(&mut self, source: &Self) {
            *self = self
                .drain(..)
                .zip(source.iter())
                .map(|(mut s, r)| {
                    s.eguis_clone(r);
                    s
                })
                .collect();
            for i in self.len()..source.len() {
                if let Some(val) = source[i - 1].eguis_clone_full() {
                    self.insert(val);
                }
            }
        }
        fn eguis_clone_full(&self) -> Option<Self> {
            if self.len() == 0 {
                return Some(Self::new());
            }
            let mut cloned: Vec<_> = self.iter().map(|s| s.eguis_clone_full()).collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(cloned.into_iter().map(|x| x.unwrap()).collect())
            }
        }
    }
}
mod vec {
    use super::*;
    impl<T: EguiStructMut + EguiStructImut + Default + Send + Any> EguiStructMut for Vec<T> {
        type ConfigTypeMut<'a> = ConfigSetMut<'a, T, bool>;

        const SIMPLE_MUT: bool = false;

        fn has_childs_mut(&self) -> bool {
            true
        }

        fn has_primitive_mut(&self) -> bool {
            false
        }

        fn show_childs_mut(
            self: &mut Self,
            ui: &mut ExUi,
            config: &mut Self::ConfigTypeMut<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            VecWrapperFull::new_mut(self).show_childs_mut(
                ui,
                config,
                reset2.map(|x| VecWrapper::new_ref(x)).as_ref(),
            )
        }

        fn start_collapsed_mut(&self) -> bool {
            VecWrapperFull::new_ref(self).start_collapsed_mut()
        }

        fn preview_str_mut<'b>(&'b self) -> &'b str {
            "TODO"
        }
    }
    impl<T: EguiStructMut + EguiStructImut + Default + Send + Any> EguiStructClone for Vec<T> {
        fn eguis_clone(&mut self, source: &Self) {
            VecWrapperFull::new_mut(self).eguis_clone(&VecWrapper::new_ref(source))
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            VecWrapperFull::new_ref(self)
                .eguis_clone_full()
                .map(|x| x.0.owned())
        }
    }
    impl<T: EguiStructMut + EguiStructImut + Default + Send + Any> EguiStructEq for Vec<T> {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            VecWrapperFull::new_ref(self).eguis_eq(&VecWrapper::new_ref(rhs))
        }
    }
}
mod vec_wrapper {
    use super::*;
    impl<'b, T: EguiStructMut, E: ConfigSetExpandableT<T>, I: ConfigSetImutT<T>> EguiStructMut
        for VecWrapper<'b, T, E, I>
    where
        Self: ConfigSetT<T, E>,
    {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = ConfigSetMut<'a, T, E>;
        fn has_childs_mut(&self) -> bool {
            true
        }
        fn has_primitive_mut(&self) -> bool {
            false
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ExUi,
            config: &mut Self::ConfigTypeMut<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.dummy_response();
            let mut idx2remove = None;
            let mut idx2swap = None;
            let len = self.len();
            self.iter_mut().enumerate().for_each(|(idx, x)| {
                let reset = reset2.map(|x| x.get(idx)).flatten();
                let has_childs = x.has_childs_mut();
                let header = |ui: &mut ExUi| {
                    ui.keep_cell_start();
                    ui.extext(idx.to_string());
                    let mut response = ui.dummy_response();
                    if config.shrinkable {
                        let bresp = ui.button("-");
                        response |= bresp.clone();
                        if bresp.clicked() {
                            idx2remove = Some(idx);
                        }
                    }
                    if config.reorder {
                        if idx != 0 {
                            let bresp = ui.button("⬆");
                            response |= bresp.clone();
                            if bresp.clicked() {
                                idx2swap = Some((idx - 1, idx));
                            }
                        }
                        if idx != len - 1 {
                            let bresp = ui.button("⬇");
                            response |= bresp.clone();
                            if bresp.clicked() {
                                idx2swap = Some((idx, idx + 1));
                            }
                        }
                    }
                    ui.keep_cell_stop();
                    if config.mutable_value {
                        crate::trait_implementor_set::primitive_w_reset(
                            x,
                            ui,
                            &mut config.inner_config,
                            reset,
                        )
                    } else {
                        I::_show_primitive_imut(x, ui)
                    }
                };
                response |=
                    ui.maybe_collapsing_rows(has_childs, header)
                        .body_simple(|ui: &mut ExUi| {
                            if config.mutable_value {
                                x.show_childs_mut(ui, &mut config.inner_config, reset)
                            } else {
                                I::_show_childs_imut(x, ui)
                            }
                        });
            });
            if let Some(idx) = idx2remove {
                self.remove(idx);
            }
            if let Some(idx) = idx2swap {
                self.swap(idx.0, idx.1);
            }
            self._add_elements(ui, config);
            response
        }
        fn start_collapsed_mut(&self) -> bool {
            self.len() > 16
        }
    }
    impl<T: EguiStructMut, E: ConfigSetExpandableT<T>, I: ConfigSetImutT<T>> EguiStructEq
        for VecWrapper<'_, T, E, I>
    {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            let mut ret = self.len() == rhs.len();
            self.iter()
                .zip(rhs.iter())
                .for_each(|(s, r)| ret &= s.eguis_eq(r));
            ret
        }
    }

    impl<T: EguiStructMut, E: ConfigSetExpandableT<T>, I: ConfigSetImutT<T>> EguiStructClone
        for VecWrapper<'_, T, E, I>
    {
        fn eguis_clone(&mut self, source: &Self) {
            self.truncate(source.len());
            self.iter_mut()
                .zip(source.iter())
                .for_each(|(s, r)| s.eguis_clone(r));
            for i in self.len()..source.len() {
                if let Some(val) = source[i].eguis_clone_full() {
                    self.push(val)
                }
            }
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            if self.len() == 0 {
                return Some(VecWrapper::new(Vec::new()));
            }
            let mut cloned: Vec<_> = self.iter().map(|s| s.eguis_clone_full()).collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(VecWrapper::new(
                    cloned.into_iter().map(|x| x.unwrap()).collect(),
                ))
            }
        }
    }
}
//##### SLICE #####
mod slice {
    use super::*;
    //TODO add impl for [T;N]

    impl<T: EguiStructMut> EguiStructMut for &mut [T] {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = T::ConfigTypeMut<'a>;
        fn has_childs_mut(&self) -> bool {
            !self.is_empty()
        }
        fn has_primitive_mut(&self) -> bool {
            false
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ExUi,
            config: &mut Self::ConfigTypeMut<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.dummy_response();
            self.iter_mut().enumerate().for_each(|(idx, x)| {
                response |= x.show_collapsing_mut(
                    ui,
                    idx.to_string(),
                    "",
                    config,
                    reset2.map(|x| x.get(idx)).flatten(),
                    None,
                )
            });
            response
        }
        fn start_collapsed_mut(&self) -> bool {
            self.len() > 16
        }
    }
    impl<T: EguiStructEq> EguiStructEq for &mut [T] {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            let mut ret = self.len() == rhs.len();
            self.iter()
                .zip(rhs.iter())
                .for_each(|(s, r)| ret &= s.eguis_eq(r));
            ret
        }
    }
    impl<T: EguiStructClone> EguiStructClone for &mut [T] {
        fn eguis_clone(&mut self, source: &Self) {
            self.iter_mut()
                .zip(source.iter())
                .for_each(|(s, r)| s.eguis_clone(r))
        }
        fn eguis_clone_full(&self) -> Option<Self> {
            if self.len() == 0 {
                Some(&mut [])
            } else {
                // let s: Vec<_> = self
                //     .iter()
                //     .map(|s| s.eguis_clone_full())
                //     .filter(|x| x.is_some())
                //     .collect();
                // if s.len() == 0 {
                //     None
                // } else {
                //     Some(&mut s.into_iter().map(|x| x.unwrap()).collect::<Box<[T]>>())
                // }
                //TODO ? better implementation possible?
                None
            }
        }
    }
}
