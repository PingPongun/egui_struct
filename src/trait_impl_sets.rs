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
                    config: &mut Self::ConfigTypeImut<'_>,
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

    impl<T: EguiStructMut + Eq + Hash + EguiStructImut + Default + Send + Any> EguiStructMut
        for std::collections::HashSet<T>
    {
        const SIMPLE_MUT: bool = false;
        type ConfigTypeMut<'a> = ConfigSetMut<'a, T, ()>;
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
                        .unwrap_or_else(|| Box::new(add.default_value()));
                    let mut add_elem = false;
                    response |= ui
                        .maybe_collapsing_rows(val.has_childs_mut(), |ui| {
                            let bresp = ui.button("+");
                            let presp = val.show_primitive_mut(ui, &mut config.inner_config);
                            add_elem = bresp.clicked();
                            bresp | presp
                        })
                        .body_simple(|ui| val.show_childs_mut(ui, &mut config.inner_config, None));
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
}
mod impl_from_wrapper {
    #[cfg(feature = "indexmap")]
    use indexmap::IndexSet;

    use super::*;
    macro_rules! impl_from_wrapper {
        ($typ:ident, [$($bound:ident),*]) => {
            impl<T: EguiStructMut + EguiStructImut + Default + Send + Any $(+$bound)*> EguiStructMut
                for $typ<T>
            {
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
}
mod vec_wrapper {
    use std::ops::DerefMut;

    use super::*;
    impl<
            'b,
            T: EguiStructMut,
            D: SetWrapperT<T>,
            E: ConfigSetExpandableT<T>,
            I: ConfigSetImutT<T>,
        > EguiStructMut for SetWrapper<'b, T, D, E, I>
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
            *self.deref_mut() = self
                .drain()
                .enumerate()
                .map(|(idx, mut x)| {
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
                                &mut x,
                                ui,
                                &mut config.inner_config,
                                reset,
                            )
                        } else {
                            I::_show_primitive_imut(&mut x, ui)
                        }
                    };
                    response |= ui.maybe_collapsing_rows(has_childs, header).body_simple(
                        |ui: &mut ExUi| {
                            if config.mutable_value {
                                x.show_childs_mut(ui, &mut config.inner_config, reset)
                            } else {
                                I::_show_childs_imut(&mut x, ui)
                            }
                        },
                    );
                    x
                })
                .collect();
            if let Some(idx) = idx2remove {
                self.remove(idx);
            }
            if let Some(idx) = idx2swap {
                self.swap(idx);
            }
            self._add_elements(ui, config);
            response
        }
        fn start_collapsed_mut(&self) -> bool {
            self.len() > 16
        }
    }
    impl<T: EguiStructMut, D: SetWrapperT<T>, E: ConfigSetExpandableT<T>, I: ConfigSetImutT<T>>
        EguiStructEq for SetWrapper<'_, T, D, E, I>
    {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            let mut ret = self.len() == rhs.len();
            self.iter()
                .zip(rhs.iter())
                .for_each(|(s, r)| ret &= s.eguis_eq(r));
            ret
        }
    }

    impl<T: EguiStructMut, D: SetWrapperT<T>, E: ConfigSetExpandableT<T>, I: ConfigSetImutT<T>>
        EguiStructClone for SetWrapper<'_, T, D, E, I>
    {
        fn eguis_clone(&mut self, source: &Self) {
            self.truncate(source.len());
            *self.deref_mut() = self
                .drain()
                .zip(source.iter())
                .map(|(mut s, r)| {
                    s.eguis_clone(r);
                    s
                })
                .collect();
            for i in self.len()..source.len() {
                if let Some(val) = source.get(i).unwrap().eguis_clone_full() {
                    self.push(val)
                }
            }
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            if self.len() == 0 {
                return Some(SetWrapper::new(D::new()));
            }
            let mut cloned: Vec<_> = self.iter().map(|s| s.eguis_clone_full()).collect();
            cloned.retain(|x| x.is_some());
            if cloned.len() == 0 {
                None
            } else {
                Some(SetWrapper::new(
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
