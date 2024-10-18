use crate::config::config_coll_expandable::*;
use crate::config::*;
use crate::traits::*;
use crate::wrappers::*;

use crate::egui::{Rect, Response, Sense};
use exgrid::ExUi;

use std::any::Any;
use std::hash::Hash;

mod impl_sets_imut {
    use super::*;
    macro_rules! impl_set_imut {
        ( $typ:ty $(, $cons:ident)?) => {
            impl<T: EguiStructImut $(, const $cons:usize)*> EguiStructImut for $typ {
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
                    config: &Self::ConfigTypeImut<'_>,
                    _reset2: Option<&Self>,
                ) -> Response {
                    let mut response = ui.interact(
                        Rect::NOTHING,
                        "dummy".into(),
                        Sense {
                            click: false,
                            drag: false,
                            focusable: false,
                        },
                    );
                    self.iter().enumerate().for_each(|(idx, x)| {
                        let has_childs = x.has_childs_imut();
                        response |= ui
                            .maybe_collapsing_rows(has_childs, |ui: &mut ExUi| {
                                ui.extext(idx.to_string());
                                x.show_primitive_imut(ui, config)
                            })
                            .initial_state(|| x.start_collapsed_imut())
                            .body_simple(|ui| x.show_childs_imut(ui, config, None));
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
    impl_set_imut! {[T;N], N}
}
mod impl_maps_imut {
    use super::*;
    macro_rules! impl_map_imut {
        ( $typ:ty ) => {
            impl<K: EguiStructImut, V: EguiStructImut> EguiStructImut for $typ {
                const SIMPLE_IMUT: bool = false;
                type ConfigTypeImut<'a> = (K::ConfigTypeImut<'a>, V::ConfigTypeImut<'a>);
                fn has_childs_imut(&self) -> bool {
                    !self.is_empty()
                }
                fn has_primitive_imut(&self) -> bool {
                    false
                }
                fn show_childs_imut(
                    &self,
                    ui: &mut ExUi,
                    config: &Self::ConfigTypeImut<'_>,
                    _reset2: Option<&Self>,
                ) -> Response {
                    let mut response = ui.interact(
                        Rect::NOTHING,
                        "dummy".into(),
                        Sense {
                            click: false,
                            drag: false,
                            focusable: false,
                        },
                    );
                    self.iter().for_each(|(key, val)| {
                        let has_childs = val.has_childs_imut();
                        response |= ui
                            .maybe_collapsing_rows(has_childs, |ui: &mut ExUi| {
                                ui.keep_cell_start();
                                key.show_primitive_imut(ui, &config.0);
                                ui.keep_cell_stop();
                                val.show_primitive_imut(ui, &config.1)
                            })
                            .initial_state(|| val.start_collapsed_imut())
                            .body_simple(|ui| val.show_childs_imut(ui, &config.1, None));
                    });
                    response
                }
                fn start_collapsed_imut(&self) -> bool {
                    self.len() > 16
                }
            }
        };
    }
    impl_map_imut! {std::collections::HashMap<K,V>}
    #[cfg(feature = "indexmap")]
    impl_map_imut! {indexmap::IndexMap<K,V> }
}

mod impl_from_wrapper {
    #[cfg(feature = "indexmap")]
    use indexmap::{IndexMap, IndexSet};
    use std::collections::{HashMap, HashSet};

    use super::*;
    macro_rules! impl_from_wrapper {
        ($typ:ident, [$($bound:ident),*]) => {
            impl<T: EguiStructMut + EguiStructImut + Default + Send + Any $(+$bound)*> EguiStructMut
                for $typ<T>
            {
                type ConfigTypeMut<'a> = ConfigCollMut<'a, T, (),bool,()>;

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
                    config: &Self::ConfigTypeMut<'_>,
                    reset2: Option<&Self>,
                ) -> Response {
                    SetWrapperFull::new_mut(self).show_childs_mut(
                        ui,
                        config,
                        reset2.map(|x| SetWrapperFull::new_ref(x)).as_ref(),
                    )
                }

                fn start_collapsed_mut(&self) -> bool {
                    SetWrapperFull::new_ref(self).start_collapsed_mut()
                }

                // fn preview_str_mut<'b>(&'b self) -> &'b str {
                //     "TODO"
                // }
            }
            impl<T: EguiStructMut + EguiStructImut + Default + Send + Any $(+$bound)*> EguiStructClone
                for $typ<T>
            {
                fn eguis_clone(&mut self, source: &Self) {
                    SetWrapperFull::new_mut(self).eguis_clone(&SetWrapperFull::new_ref(source))
                }

                fn eguis_clone_full(&self) -> Option<Self> {
                    SetWrapperFull::new_ref(self)
                        .eguis_clone_full()
                        .map(|x| x.0.owned())
                }
            }
            impl<T: EguiStructMut + EguiStructImut + Default + Send + Any $(+$bound)*> EguiStructEq
                for $typ<T>
            {
                fn eguis_eq(&self, rhs: &Self) -> bool {
                    SetWrapperFull::new_ref(self).eguis_eq(&SetWrapperFull::new_ref(rhs))
                }
            }
        };
    }
    impl_from_wrapper! {Vec,[]}
    impl_from_wrapper! {HashSet, [Hash, Eq]}
    #[cfg(feature = "indexmap")]
    impl_from_wrapper! {IndexSet, [Hash, Eq]}

    macro_rules! impl_from_map_wrapper {
        ($typ:ident) => {
            impl<
                    K: EguiStructMut + EguiStructImut + Default + Send + Any + Hash + Eq,
                    V: EguiStructMut + EguiStructImut + Default + Send + Any,
                > EguiStructMut for $typ<K, V>
            {
                type ConfigTypeMut<'a> = ConfigCollMut<'a, K, V, bool, bool>;

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
                    config: &Self::ConfigTypeMut<'_>,
                    reset2: Option<&Self>,
                ) -> Response {
                    CollWrapperFull::new_mut(self).show_childs_mut(
                        ui,
                        config,
                        reset2.map(|x| CollWrapperFull::new_ref(x)).as_ref(),
                    )
                }

                fn start_collapsed_mut(&self) -> bool {
                    CollWrapperFull::new_ref(self).start_collapsed_mut()
                }

                // fn preview_str_mut<'b>(&'b self) -> &'b str {
                //     "TODO"
                // }
            }
            impl<
                    K: EguiStructMut + EguiStructImut + Default + Send + Any + Hash + Eq,
                    V: EguiStructMut + EguiStructImut + Default + Send + Any,
                > EguiStructClone for $typ<K, V>
            {
                fn eguis_clone(&mut self, source: &Self) {
                    CollWrapperFull::new_mut(self).eguis_clone(&CollWrapperFull::new_ref(source))
                }

                fn eguis_clone_full(&self) -> Option<Self> {
                    CollWrapperFull::new_ref(self)
                        .eguis_clone_full()
                        .map(|x| x.0.owned())
                }
            }
            impl<
                    K: EguiStructMut + EguiStructImut + Default + Send + Any + Hash + Eq,
                    V: EguiStructMut + EguiStructImut + Default + Send + Any,
                > EguiStructEq for $typ<K, V>
            {
                fn eguis_eq(&self, rhs: &Self) -> bool {
                    CollWrapperFull::new_ref(self).eguis_eq(&CollWrapperFull::new_ref(rhs))
                }
            }
        };
    }
    impl_from_map_wrapper! {HashMap}
    #[cfg(feature = "indexmap")]
    impl_from_map_wrapper! {IndexMap}
}
mod coll_wrapper {
    use super::*;
    impl<
            'b,
            K: EguiStructMut,
            V: EguiStructMut,
            D: CollWrapperT<K, V>,
            EK: ConfigCollExpandableT<K>,
            EV: ConfigCollExpandableT<V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        > EguiStructMut for CollWrapper<'b, K, V, D, EK, EV, IK, IV>
    where
        Self: ConfigCollT<K, V, EK, EV>,
    {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = ConfigCollMut<'a, K, V, EK, EV>;
        fn has_childs_mut(&self) -> bool {
            true
        }
        fn has_primitive_mut(&self) -> bool {
            false
        }
        fn show_childs_mut(
            &mut self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeMut<'_>,
            reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.dummy_response();
            let mut idx2swap = None;
            let len = self.e_len();
            self.e_map(|idx, (mut key, mut val)| {
                let mut remove = true;
                let reset = reset2.map(|x| x.e_get(idx)).flatten();
                let has_childs = if typeid::of::<V>() == typeid::of::<()>() {
                    key.has_childs_mut()
                } else {
                    val.has_childs_mut()
                };
                let header = |ui: &mut ExUi| {
                    ui.keep_cell_start();
                    let mut response = ui.dummy_response();
                    if typeid::of::<V>() == typeid::of::<()>() {
                        ui.extext(idx.to_string());
                    } else {
                        response |= D::key_show_primitive::<IK>(
                            &mut key,
                            config.mutable_key,
                            ui,
                            &config.inner_config.0,
                            reset.map(|i| i.0),
                        )
                    }
                    if config.shrinkable {
                        let bresp = ui.button("-");
                        response |= bresp.clone();
                        if bresp.clicked() {
                            remove = false;
                        }
                    }
                    if config.reorder && D::REORDERABLE {
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
                    if typeid::of::<V>() == typeid::of::<()>() {
                        D::key_show_primitive::<IK>(
                            &mut key,
                            config.mutable_value,
                            ui,
                            &config.inner_config.0,
                            reset.map(|i| i.0),
                        )
                    } else {
                        IV::show_primitive(
                            &mut val,
                            config.mutable_value,
                            ui,
                            &config.inner_config.1,
                            reset.map(|i| i.1),
                        )
                    }
                };
                response |=
                    ui.maybe_collapsing_rows(has_childs, header)
                        .body_simple(|ui: &mut ExUi| {
                            if typeid::of::<V>() == typeid::of::<()>() {
                                D::key_show_childs::<IK>(
                                    &mut key,
                                    config.mutable_value,
                                    ui,
                                    &config.inner_config.0,
                                    reset.map(|i| i.0),
                                )
                            } else {
                                IV::show_childs(
                                    &mut val,
                                    config.mutable_value,
                                    ui,
                                    &config.inner_config.1,
                                    reset.map(|i| i.1),
                                )
                            }
                        });
                remove
            });
            if let Some(idx) = idx2swap {
                self.e_swap(idx);
            }
            self._add_elements(ui, config);
            response
        }
        fn start_collapsed_mut(&self) -> bool {
            self.e_len() > 16
        }
    }
    impl<
            K: EguiStructMut,
            V: EguiStructMut,
            D: CollWrapperT<K, V>,
            EK: ConfigCollExpandableT<K>,
            EV: ConfigCollExpandableT<V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        > EguiStructEq for CollWrapper<'_, K, V, D, EK, EV, IK, IV>
    {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            let mut ret = self.e_len() == rhs.e_len();
            self.e_iter()
                .zip(rhs.e_iter())
                .for_each(|(s, r)| ret &= s.0.eguis_eq(r.0) && s.1.eguis_eq(r.1));
            ret
        }
    }

    impl<
            K: EguiStructMut,
            V: EguiStructMut,
            D: CollWrapperT<K, V>,
            EK: ConfigCollExpandableT<K>,
            EV: ConfigCollExpandableT<V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        > EguiStructClone for CollWrapper<'_, K, V, D, EK, EV, IK, IV>
    {
        fn eguis_clone(&mut self, source: &Self) {
            let src: Vec<_> = source.e_iter().collect();
            **self = D::e_from_iter(self.e_drain().zip(src.iter()).map(|(mut s, r)| {
                s.0.eguis_clone(r.0);
                s.1.eguis_clone(r.1);
                s
            }));
            for i in self.e_len()..source.e_len() {
                let s = src[i];
                if let Some(val) = s.0.eguis_clone_full().zip(s.1.eguis_clone_full()) {
                    self.e_push(val);
                }
            }
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            if self.e_len() == 0 {
                return Some(CollWrapper::new(D::e_new()));
            }
            let mut cloned: Vec<_> = self
                .e_iter()
                .map(|s| s.0.eguis_clone_full().zip(s.1.eguis_clone_full()))
                .collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(CollWrapper::new(D::e_from_iter(
                    cloned.into_iter().map(|x| x.unwrap()),
                )))
            }
        }
    }
}
//##### SLICE #####
mod slice {
    use super::*;
    macro_rules! gen_impl {
        ($typ:ty $(, $cons:ident)?) => {
            impl<T: EguiStructMut $(,const $cons: usize)*> EguiStructMut for $typ {
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
                    config: &Self::ConfigTypeMut<'_>,
                    reset2: Option<&Self>,
                ) -> Response {
                    let mut response = ui.dummy_response();
                    let mut idx2swap = None;
                    let len = self.len();
                    self.iter_mut().enumerate().for_each(|(idx, x)| {
                        let has_childs = x.has_childs_mut();
                        let header = |ui: &mut ExUi| {
                            let mut hresp = ui.dummy_response();
                            ui.keep_cell_start();
                            crate::trait_implementor_set::primitive_label(ui, idx.to_string(), "");
                            if idx != 0 {
                                let bresp = ui.button("⬆");
                                hresp |= bresp.clone();
                                if bresp.clicked() {
                                    idx2swap = Some((idx - 1, idx));
                                }
                            }
                            if idx != len - 1 {
                                let bresp = ui.button("⬇");
                                hresp |= bresp.clone();
                                if bresp.clicked() {
                                    idx2swap = Some((idx, idx + 1));
                                }
                            }
                            ui.keep_cell_stop();
                            hresp | crate::trait_implementor_set::primitive_w_reset(x, ui, config, reset2.map(|x| x.get(idx)).flatten())
                        };
                        response |= ui.maybe_collapsing_rows(has_childs, header)
                            .initial_state(|| x.start_collapsed_mut())
                            .body_simple(|ui| x.show_childs_mut(ui, config, reset2.map(|x| x.get(idx)).flatten()));
                    });
                    if let Some(idx) = idx2swap {
                        self.swap(idx.0, idx.1);
                    }
                    response
                }
                fn start_collapsed_mut(&self) -> bool {
                    self.len() > 16
                }
            }
            impl<T: EguiStructEq $(,const $cons: usize)*> EguiStructEq for $typ {
                fn eguis_eq(&self, rhs: &Self) -> bool {
                    let mut ret = self.len() == rhs.len();
                    self.iter()
                        .zip(rhs.iter())
                        .for_each(|(s, r)| ret &= s.eguis_eq(r));
                    ret
                }
            }
            impl<T: EguiStructClone $(,const $cons: usize)*> EguiStructClone for $typ {
                fn eguis_clone(&mut self, source: &Self) {
                    self.iter_mut()
                        .zip(source.iter())
                        .for_each(|(s, r)| s.eguis_clone(r))
                }
                fn eguis_clone_full(&self) -> Option<Self> {
                    // if self.len() == 0 {
                    //     Some($empty)
                    // } else {
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
                    // }
                }
            }
        };
    }
    gen_impl! {&mut [T]}
    gen_impl! {[T;N], N}
}
