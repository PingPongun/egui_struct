use crate::config::config_set_expandable::*;
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
}
mod hashset {
    use super::*;

    impl<T: EguiStructMut + Eq + Hash + EguiStructImut + Default + Send + Any> EguiStructMut
        for std::collections::HashSet<T>
    {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = ConfigSetMut<'a, T, (), (), ()>;
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
            _reset2: Option<&Self>,
        ) -> Response {
            let mut response = ui.dummy_response();

            let mut idx = 0;
            self.retain(|x| {
                let has_childs = x.has_childs_imut();
                let header = |ui: &mut ExUi| {
                    ui.keep_cell_start();
                    ui.extext(idx.to_string());
                    let mut response = ui.dummy_response();
                    if config.shrinkable {
                        let bresp = ui.button("-");
                        response |= bresp.clone();
                    }
                    ui.keep_cell_stop();
                    response | x.show_primitive_imut(ui, &mut Default::default())
                };
                let loc_response =
                    ui.maybe_collapsing_rows(has_childs, header)
                        .body_simple(|ui: &mut ExUi| {
                            x.show_childs_imut(ui, &mut Default::default(), None)
                        });
                idx += 1;
                response |= loc_response.clone();
                !loc_response.clicked()
            });

            if let Some(add) = &config.expandable {
                if config.max_len.is_none() || self.len() < config.max_len.unwrap() {
                    let id = ui.id();
                    let mut val: Box<T> = ui
                        .data_remove(id)
                        .unwrap_or_else(|| Box::new(add.0.default_value()));
                    let mut add_elem = false;
                    response |= ui
                        .maybe_collapsing_rows(val.has_childs_mut(), |ui| {
                            let bresp = ui.button("+");
                            let presp = val.show_primitive_mut(ui, &config.inner_config.0);
                            add_elem = bresp.clicked();
                            bresp | presp
                        })
                        .body_simple(|ui| val.show_childs_mut(ui, &config.inner_config.0, None));
                    if add_elem {
                        self.insert(*val);
                    } else {
                        ui.data_store(id, val);
                    }
                }
            }
            response
        }

        fn start_collapsed_mut(&self) -> bool {
            self.len() > 16
        }
    }

    impl<T: EguiStructEq> EguiStructEq for std::collections::HashSet<T> {
        //TODO allow mismatched order for std::HashMap
        fn eguis_eq(&self, rhs: &Self) -> bool {
            let mut ret = self.len() == rhs.len();
            self.iter()
                .zip(rhs.iter())
                .for_each(|(s, r)| ret &= s.eguis_eq(r));
            ret
        }
    }

    impl<T: EguiStructClone + Eq + Hash> EguiStructClone for std::collections::HashSet<T> {
        fn eguis_clone(&mut self, source: &Self) {
            let src: Vec<_> = source.iter().collect();
            *self = self
                .drain()
                .zip(src.e_iter())
                .map(|(mut s, r)| {
                    s.eguis_clone(r.0);
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
}
mod impl_from_wrapper {
    use indexmap::IndexMap;
    #[cfg(feature = "indexmap")]
    use indexmap::IndexSet;

    use super::*;
    macro_rules! impl_from_wrapper {
        ($typ:ident, [$($bound:ident),*]) => {
            impl<T: EguiStructMut + EguiStructImut + Default + Send + Any $(+$bound)*> EguiStructMut
                for $typ<T>
            {
                type ConfigTypeMut<'a> = ConfigSetMut<'a, T, (),bool,()>;

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
    #[cfg(feature = "indexmap")]
    impl_from_wrapper! {IndexSet, [Hash, Eq]}

    macro_rules! impl_from_map_wrapper {
        ($typ:ident) => {
            impl<
                    K: EguiStructMut + EguiStructImut + Default + Send + Any + Hash + Eq,
                    V: EguiStructMut + EguiStructImut + Default + Send + Any,
                > EguiStructMut for $typ<K, V>
            {
                type ConfigTypeMut<'a> = ConfigSetMut<'a, K, V, bool, bool>;

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
                    MapWrapperFull::new_mut(self).show_childs_mut(
                        ui,
                        config,
                        reset2.map(|x| MapWrapperFull::new_ref(x)).as_ref(),
                    )
                }

                fn start_collapsed_mut(&self) -> bool {
                    MapWrapperFull::new_ref(self).start_collapsed_mut()
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
                    MapWrapperFull::new_mut(self).eguis_clone(&MapWrapperFull::new_ref(source))
                }

                fn eguis_clone_full(&self) -> Option<Self> {
                    MapWrapperFull::new_ref(self)
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
                    MapWrapperFull::new_ref(self).eguis_eq(&MapWrapperFull::new_ref(rhs))
                }
            }
        };
    }
    // impl_from_map_wrapper! {HashMap}
    #[cfg(feature = "indexmap")]
    impl_from_map_wrapper! {IndexMap}
}
mod vec_wrapper {
    use std::ops::DerefMut;

    use super::*;
    impl<
            'b,
            K: EguiStructMut,
            V: EguiStructMut,
            D: SetWrapperT<K, V>,
            EK: ConfigSetExpandableT<K>,
            EV: ConfigSetExpandableT<V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        > EguiStructMut for SetWrapper<'b, K, V, D, EK, EV, IK, IV>
    where
        Self: ConfigSetT<K, V, EK, EV>,
    {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = ConfigSetMut<'a, K, V, EK, EV>;
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
            let mut idx2remove = None;
            let mut idx2swap = None;
            let len = self.e_len();
            *self.deref_mut() =
                D::e_from_iter(self.e_drain().enumerate().map(|(idx, (mut key, mut val))| {
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
                            response |= IK::show_primitive(
                                &mut key,
                                config.mutable_key,
                                ui,
                                &config.inner_config.0,
                                reset.map(|i| i.0),
                            );
                        }
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
                        if typeid::of::<V>() == typeid::of::<()>() {
                            IK::show_primitive(
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
                    response |= ui.maybe_collapsing_rows(has_childs, header).body_simple(
                        |ui: &mut ExUi| {
                            if typeid::of::<V>() == typeid::of::<()>() {
                                IK::show_childs(
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
                        },
                    );
                    (key, val)
                }));
            if let Some(idx) = idx2remove {
                self.e_remove(idx);
            }
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
            D: SetWrapperT<K, V>,
            EK: ConfigSetExpandableT<K>,
            EV: ConfigSetExpandableT<V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        > EguiStructEq for SetWrapper<'_, K, V, D, EK, EV, IK, IV>
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
            D: SetWrapperT<K, V>,
            EK: ConfigSetExpandableT<K>,
            EV: ConfigSetExpandableT<V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        > EguiStructClone for SetWrapper<'_, K, V, D, EK, EV, IK, IV>
    {
        fn eguis_clone(&mut self, source: &Self) {
            self.e_truncate(source.e_len());
            *self.deref_mut() =
                D::e_from_iter(self.e_drain().zip(source.e_iter()).map(|(mut s, r)| {
                    s.0.eguis_clone(r.0);
                    s.1.eguis_clone(r.1);
                    s
                }));
            for i in self.e_len()..source.e_len() {
                let s = source.e_get(i).unwrap();
                if let Some(val) = s.0.eguis_clone_full().zip(s.1.eguis_clone_full()) {
                    self.e_push(val)
                }
            }
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            if self.e_len() == 0 {
                return Some(SetWrapper::new(D::e_new()));
            }
            let mut cloned: Vec<_> = self
                .e_iter()
                .map(|s| s.0.eguis_clone_full().zip(s.1.eguis_clone_full()))
                .collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(SetWrapper::new(D::e_from_iter(
                    cloned.into_iter().map(|x| x.unwrap()),
                )))
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
            config: &Self::ConfigTypeMut<'_>,
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
