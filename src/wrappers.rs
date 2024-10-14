//! Module provides wrappers that allow to use alternative EguiStruct* traits implementations (eg. with loosen bounds or changed functionality)
//! or use types that does not impl them
//!
//! # Vec/Set wrappers
//!
//! [Vec]/[IndexSet] wrappers that allow to get [EguiStructMut] implementation for [Vec]/[IndexSet] with looser bounds
//!
//! There are 3 traits that characterize this wrappers (different trait combination provide slightly different feature set, but allows to loosen bounds on `T`):
//! - `S`- [Send]+[Any] - Elements can be edited prior adding
//!     (otherwise `+` button will add "default" value)
//! - `D`- [Default] - New elements (`+` button) will be generated with [Default]
//!     (otherwise function specified in config struct will be used)
//! - `I`- [EguiStructImut] -  immutable (`config.mutable_value == false`) Set elements will be using this trait
//!     (otherwise they will use disabled [egui::Ui])
//!
//! | `S` | `D` | type of `config.expandable.unwrap()`   | comment |
//! |-----|-----|----------------------------------------|---------|
//! | ✅ | ✅ | [bool]                                  | `bool` controls if value can be modified prior add |
//! | ❌ | ✅ | [()](unit)                              |
//! | ✅ | ❌ | [ConfigSetExpandable]                   |
//! | ❌ | ❌ | [ConfigSetExpandableNStore]             |
//!
//! [EguiStructMut] for [Vec]/[IndexSet] is implemented using [SetWrapperFull]
//!
//! Usage:
//!
//! ```
//! // In derive
//! #[EguiStructMut]
//! struct MyStruct<T: !Send>{
//!     #[eguis(wrapper="SetDI")]
//!     field: Vec<T>
//! }
//! // Standalone usage
//! SetWrapperFull::new_mut(&mut vec).eguis_mut().show(ui);
//! // Usage during trait implementation
//! SetWrapperFull::new_mut(&mut vec).show_collapsing_mut(
//!     ui,
//!     "name",
//!     "hint",
//!     config,
//!     reset2.map(|x| SetWrapperFull::new_ref(x)).as_ref(),
//!     None,
//! );
//! ```
//!
//! # Combobox wrappers
//!
//! Wrapper that provides `EguiStructMut` impl in form of drop-down list (possible options are passed via iterator in config struct)
//!
//! Usage:
//!
//! ```
//! // In derive
//! #[EguiStructMut]
//! struct MyStruct<T: ToString + Clone + PartialEq>{
//!     #[eguis(wrapper="ComboBox", config = "Some(&[ T(1), T(2), T(3) ].into_iter())")]
//!     field: T
//! }
//! // Usage during trait implementation
//! ComboBox::new_mut(&mut val).show_collapsing_mut(
//!     ui,
//!     "name",
//!     "hint",
//!     Some(iter),
//!     reset2.map(|x| ComboBox::new_ref(x)).as_ref(),
//!     None,
//! );
//! ```

use crate::config::config_set_expandable::*;
use crate::config::*;
use crate::egui;
use crate::traits::*;

use egui::Response;
use exgrid::ExUi;
use std::any::Any;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[cfg(doc)]
use indexmap::*;

pub use set::*;
mod set {
    use super::*;
    pub(crate) use config_set_imut_t::*;
    mod config_set_imut_t {
        use super::*;
        pub struct ConfigSetMutTrueImut<T>(PhantomData<T>);
        pub struct ConfigSetMutDisableMut<T>(PhantomData<T>);
        pub(crate) trait ConfigSetImutT<T: EguiStructMut> {
            fn _show_childs_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response;
            fn _show_primitive_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response;
            fn _has_childs_imut(val: &T) -> bool;
            fn show_childs(
                val: &mut T,
                mutable: bool,
                ui: &mut ExUi,
                config: &T::ConfigTypeMut<'_>,
                reset2: Option<&T>,
            ) -> Response {
                if mutable {
                    val.show_childs_mut(ui, config, reset2)
                } else {
                    Self::_show_childs_imut(val, ui, &Default::default())
                }
            }
            fn show_primitive(
                val: &mut T,
                mutable: bool,
                ui: &mut ExUi,
                config: &T::ConfigTypeMut<'_>,
                reset2: Option<&T>,
            ) -> Response {
                if mutable {
                    crate::trait_implementor_set::primitive_w_reset(val, ui, config, reset2)
                } else {
                    Self::_show_primitive_imut(val, ui, &Default::default())
                }
            }
        }
        impl<T: EguiStructMut + EguiStructImut> ConfigSetImutT<T> for () {
            fn _show_childs_imut(
                _val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                ("[".to_string() + &ui.get_nesting_cursor().last().unwrap().to_string() + "]")
                    .show_primitive_imut(ui, &Default::default())
            }
            fn _show_primitive_imut(
                _val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                ("[".to_string() + &ui.get_nesting_cursor().last().unwrap().to_string() + "]")
                    .show_primitive_imut(ui, &Default::default())
            }

            fn _has_childs_imut(_val: &T) -> bool {
                todo!()
            }
        }
        impl<T: EguiStructMut + EguiStructImut> ConfigSetImutT<T> for ConfigSetMutTrueImut<T> {
            fn _show_childs_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                val.show_childs_imut(ui, &mut Default::default(), None)
            }
            fn _show_primitive_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                val.show_primitive_imut(ui, &mut Default::default())
            }

            fn _has_childs_imut(val: &T) -> bool {
                val.has_childs_imut()
            }
        }
        impl<T: EguiStructMut> ConfigSetImutT<T> for ConfigSetMutDisableMut<T> {
            fn _show_childs_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                ui.start_disabled();
                let ret = val.show_childs_mut(ui, &mut Default::default(), None);
                ui.stop_disabled();
                ret
            }
            fn _show_primitive_imut(
                val: &mut T,
                ui: &mut ExUi,
                _config: &T::ConfigTypeMut<'_>,
            ) -> Response {
                ui.start_disabled();
                let ret = val.show_primitive_mut(ui, &mut Default::default());
                ui.stop_disabled();
                ret
            }
            fn _has_childs_imut(val: &T) -> bool {
                val.has_childs_mut()
            }
        }
    }
    pub(crate) use maybe_owned::*;
    mod maybe_owned {
        use super::*;
        pub(crate) enum MaybeOwned<'a, T> {
            Owned(T),
            Borrowed(&'a T),
            BorrowedMut(&'a mut T),
        }

        impl<'a, T> MaybeOwned<'a, T> {
            pub fn owned(self) -> T {
                if let MaybeOwned::Owned(i) = self {
                    i
                } else {
                    unreachable!()
                }
            }
        }

        impl<'a, T> Deref for MaybeOwned<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                match self {
                    MaybeOwned::Owned(i) => i,
                    MaybeOwned::Borrowed(i) => i,
                    MaybeOwned::BorrowedMut(i) => i,
                }
            }
        }
        impl<'a, T> DerefMut for MaybeOwned<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    MaybeOwned::Owned(i) => i,
                    MaybeOwned::Borrowed(_) => {
                        panic!("MaybeOwned storing `Borrowed` value can not mutably deref")
                    }
                    MaybeOwned::BorrowedMut(i) => i,
                }
            }
        }
    }

    pub(crate) use set_wrapper_t::*;
    mod set_wrapper_t {

        use super::*;
        pub trait SetWrapperT<K, V> {
            fn e_len(&self) -> usize;
            fn e_new() -> Self;
            fn e_get(&self, idx: usize) -> Option<(&K, &V)>;
            fn e_remove(&mut self, idx: usize);
            fn e_truncate(&mut self, len: usize);
            fn e_swap(&mut self, idx: (usize, usize));
            fn e_push(&mut self, val: (K, V));
            fn e_drain(&mut self) -> impl Iterator<Item = (K, V)>;
            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
            where
                K: 'a,
                V: 'a;
            fn e_from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self;
        }
        impl<T> SetWrapperT<T, ()> for Vec<T> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&T, &())> {
                self.deref().get(idx).map(|x| (x, &()))
            }

            fn e_remove(&mut self, idx: usize) {
                self.remove(idx);
            }

            fn e_truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn e_swap(&mut self, idx: (usize, usize)) {
                self.deref_mut().swap(idx.0, idx.1);
            }

            fn e_push(&mut self, value: (T, ())) {
                self.push(value.0);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (T, ())> {
                self.drain(..).map(|x| (x, ()))
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a ())>
            where
                T: 'a,
            {
                self.deref().iter().map(|x| (x, &()))
            }

            fn e_from_iter<I: IntoIterator<Item = (T, ())>>(iterable: I) -> Self {
                iterable.into_iter().map(|(x, _)| x).collect()
            }
        }

        #[cfg(feature = "indexmap")]
        impl<T: Hash + Eq> SetWrapperT<T, ()> for indexmap::IndexSet<T> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&T, &())> {
                self.get_index(idx).map(|x| (x, &()))
            }

            fn e_remove(&mut self, idx: usize) {
                self.shift_remove_index(idx);
            }

            fn e_truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn e_swap(&mut self, idx: (usize, usize)) {
                self.swap_indices(idx.0, idx.1);
            }

            fn e_push(&mut self, value: (T, ())) {
                self.insert(value.0);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (T, ())> {
                self.drain(..).map(|x| (x, ()))
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a ())>
            where
                T: 'a,
            {
                self.iter().map(|x| (x, &()))
            }
            fn e_from_iter<I: IntoIterator<Item = (T, ())>>(iterable: I) -> Self {
                iterable.into_iter().map(|(x, _)| x).collect()
            }
        }
        #[cfg(feature = "indexmap")]
        impl<K: Hash + Eq, V> SetWrapperT<K, V> for indexmap::IndexMap<K, V> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&K, &V)> {
                self.get_index(idx)
            }

            fn e_remove(&mut self, idx: usize) {
                self.shift_remove_index(idx);
            }

            fn e_truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn e_swap(&mut self, idx: (usize, usize)) {
                self.swap_indices(idx.0, idx.1);
            }

            fn e_push(&mut self, value: (K, V)) {
                self.insert(value.0, value.1);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (K, V)> {
                self.drain(..)
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
            where
                K: 'a,
                V: 'a,
            {
                self.iter()
            }
            fn e_from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
                iterable.into_iter().collect()
            }
        }
    }

    mod _set_wrapper {
        use super::*;
        #[allow(private_interfaces, private_bounds)]
        /// Thin wrapper around [Vec]/[indexmap::IndexSet], that provides generic configured [EguiStructMut] implementation for [Vec]/[indexmap::IndexSet].
        ///
        /// Different generics combination provide slightly different feature set, but allows to loosen bounds on `T`
        ///
        /// Generally use aliases to this type ([SetWrapperFull], [SetWrapperI], ..), instead using this type directly.
        ///
        /// See [crate::wrappers] module description.
        pub struct SetWrapper<
            'a,
            K: EguiStructMut,
            V: EguiStructMut,
            D: SetWrapperT<K, V>,
            EK: ConfigSetExpandableT<K>,
            EV: ConfigSetExpandableT<V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        >(pub MaybeOwned<'a, D>, PhantomData<(K, V, EK, EV, IK, IV)>);

        #[allow(private_bounds)]
        impl<
                'a,
                K: EguiStructMut,
                V: EguiStructMut,
                D: SetWrapperT<K, V>,
                EK: ConfigSetExpandableT<K>,
                EV: ConfigSetExpandableT<V>,
                IK: ConfigSetImutT<K>,
                IV: ConfigSetImutT<V>,
            > SetWrapper<'a, K, V, D, EK, EV, IK, IV>
        {
            pub fn new(inner: D) -> Self {
                SetWrapper(MaybeOwned::Owned(inner), PhantomData)
            }
            pub fn new_mut(inner: &'a mut D) -> Self {
                SetWrapper(MaybeOwned::BorrowedMut(inner), PhantomData)
            }
            pub fn new_ref(inner: &'a D) -> Self {
                SetWrapper(MaybeOwned::Borrowed(inner), PhantomData)
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
            > Deref for SetWrapper<'_, K, V, D, EK, EV, IK, IV>
        {
            type Target = D;

            fn deref(&self) -> &Self::Target {
                &self.0
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
            > DerefMut for SetWrapper<'_, K, V, D, EK, EV, IK, IV>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
    pub(crate) use config_set_t::*;
    mod config_set_t {
        use super::*;
        use crate::wrappers::set::_set_wrapper::SetWrapper;
        pub(crate) trait ConfigSetT<
            K: EguiStructMut,
            V: EguiStructMut,
            EK: ConfigSetExpandableT<K>,
            EV: ConfigSetExpandableT<V>,
        >
        {
            fn _add_elements(
                &mut self,
                ui: &mut ExUi,
                config: &ConfigSetMut<'_, K, V, EK, EV>,
            ) -> Response;
        }
        macro_rules! _add_elements_send {
($typ:ty, [$($bound:ident),*], $typV:ty, [$($boundV:ident),*]) => {
    impl<
            K: EguiStructMut $(+ $bound)*,
            V: EguiStructMut $(+ $boundV)*,
            D: SetWrapperT<K,V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        > ConfigSetT<K,V, $typ, $typV> for SetWrapper<'_, K, V, D, $typ,$typV, IK,IV>
    {
        fn _add_elements(
            &mut self,
            ui: &mut ExUi,
            config: &ConfigSetMut<'_, K,V, $typ, $typV>,
        ) -> Response {
            let mut response = ui.dummy_response();
            if let Some(add) = &config.expandable {
                if config.max_len.is_none() || self.0.e_len() < config.max_len.unwrap() {
                    if <$typ as ConfigSetExpandableT<K>>::mutable(&add.0) {
                        let id = ui.id();
                        let mut key: Box<K> = ui
                            .data_remove(id)
                            .unwrap_or_else(|| Box::new(add.0.default_value()));
                        let mut add_elem = false;
                        let show_childs=if typeid::of::<V>() == typeid::of::<()>() {
                            key.has_childs_mut()
                        } else {
                            false
                        };
                        let resp = ui
                            .maybe_collapsing_rows(show_childs, |ui| {
                                let bresp = ui.button("+");
                                let presp =
                                    key.show_primitive_mut(ui, &config.inner_config.0);
                                add_elem = bresp.clicked();
                                bresp | presp
                            })
                            .body_simple(|ui| {
                                key.show_childs_mut(ui, &config.inner_config.0, None)
                            });
                        response = resp.clone();
                        if add_elem {
                            self.0.e_push((*key, add.1.default_value()));
                        } else {
                            ui.data_store(id, key);
                        }
                    } else {
                        let bresp = ui.button("+");
                        ui.end_row();
                        response = bresp.clone();
                        if bresp.clicked() {
                            self.0.e_push((add.0.default_value(),add.1.default_value()));
                        }
                    };
                }
            }
            response
        }
    }
};
}
        macro_rules! _add_elements_sendsend {
($typ:ty, [$($bound:ident),*], $typV:ty, [$($boundV:ident),*]) => {
impl<
    K: EguiStructMut $(+ $bound)*,
    V: EguiStructMut $(+ $boundV)*,
    D: SetWrapperT<K,V>,
    IK: ConfigSetImutT<K>,
    IV: ConfigSetImutT<V>,
> ConfigSetT<K,V, $typ, $typV> for SetWrapper<'_, K, V, D, $typ,$typV, IK,IV>
{
fn _add_elements(
    &mut self,
    ui: &mut ExUi,
    config: &ConfigSetMut<'_, K,V, $typ, $typV>,
) -> Response {
    let mut response = ui.dummy_response();
    if let Some(add) = &config.expandable {
        if config.max_len.is_none() || self.0.e_len() < config.max_len.unwrap() {
            let mut_key=<$typV as ConfigSetExpandableT<V>>::mutable(&add.1);
            let mut_val=<$typ as ConfigSetExpandableT<K>>::mutable(&add.0);
            if mut_key||mut_val {
                let id = ui.id();
                let mut key_val: Box<(K,V)> = ui
                    .data_remove(id)
                    .unwrap_or_else(|| Box::new((add.0.default_value(),add.1.default_value())));
                let mut add_elem = false;
                let has_childs = if mut_val { key_val.1.has_childs_mut() } else { IV::_has_childs_imut(&key_val.1) };
                let resp = ui
                    .maybe_collapsing_rows(has_childs, |ui| {
                        ui.keep_cell_start();
                        let bresp = ui.button("+");
                        let mut presp = IK::show_primitive( &mut key_val.0, mut_key, ui, &config.inner_config.0, None);
                        ui.keep_cell_stop();
                        presp |= IV::show_primitive( &mut key_val.1, mut_val, ui, &config.inner_config.1, None);
                        add_elem = bresp.clicked();
                        bresp | presp
                    })
                    .body_simple(|ui| {
                        IV::show_childs( &mut key_val.1,mut_val, ui, &config.inner_config.1, None)
                    });
                response = resp.clone();
                if add_elem {
                    self.e_push(*key_val);
                } else {
                    ui.data_store(id, key_val);
                }
            } else {
                let bresp = ui.button("+");
                ui.end_row();
                response = bresp.clone();
                if bresp.clicked() {
                    self.0.e_push((add.0.default_value(),add.1.default_value()));
                }
            };
        }
    }
    response
}
}
};
}
        macro_rules! _add_elements_nsend {
($typ:ty, [$($bound:ident),*], $typV:ty, [$($boundV:ident),*]) => {
    impl<
            K: EguiStructMut $(+ $bound)*,
            V: EguiStructMut $(+ $boundV)*,
            D: SetWrapperT<K,V>,
            IK: ConfigSetImutT<K>,
            IV: ConfigSetImutT<V>,
        > ConfigSetT<K,V, $typ, $typV> for SetWrapper<'_, K, V, D, $typ,$typV, IK,IV>
    {
        fn _add_elements(
            &mut self,
            ui: &mut ExUi,
            config: &ConfigSetMut<'_, K,V, $typ, $typV>,
        ) -> Response {
            let mut response = ui.dummy_response();
            if let Some(add) = &config.expandable {
                if config.max_len.is_none() || self.0.e_len() < config.max_len.unwrap() {
                    let bresp = ui.button("+");
                    ui.end_row();
                    response = bresp.clone();
                    if bresp.clicked() {
                        self.0.e_push((add.0.default_value(),add.1.default_value()));
                    }
                }
            }
            response
        }
    }
};
}
        _add_elements_nsend! { ConfigSetExpandableNStore<'_, K>, [], (), [Default]}
        _add_elements_nsend! { (), [Default], ConfigSetExpandableNStore<'_, V>, []}
        _add_elements_nsend! { (), [Default], (), [Default]}
        _add_elements_nsend! { ConfigSetExpandableNStore<'_, K>, [], ConfigSetExpandableNStore<'_, V>, []}
        _add_elements_send! { ConfigSetExpandable<'_, K>, [Send,Any], (), [Default]}
        _add_elements_send! { ConfigSetExpandable<'_, K>, [Send,Any], ConfigSetExpandableNStore<'_, V>, []}
        _add_elements_send! { bool, [Default,Send,Any], (), [Default]}
        _add_elements_send! { bool, [Default,Send,Any], ConfigSetExpandableNStore<'_, V>, []}
        _add_elements_sendsend! { ConfigSetExpandable<'_, K>, [Send,Any], bool, [Default,Send,Any]}
        _add_elements_sendsend! { bool, [Default,Send,Any],  ConfigSetExpandable<'_, V>, [Send,Any]}
        _add_elements_sendsend! { bool, [Default,Send,Any],  bool, [Default,Send,Any]}
        _add_elements_sendsend! { ConfigSetExpandable<'_, K>, [Send,Any], ConfigSetExpandable<'_, V>, [Send,Any]}
    }

    pub use set_wrappers::*;
    mod set_wrappers {
        use super::*;
        pub use _set_wrapper::SetWrapper;
        type SimpleSetWrapper<'a, T, D, E, I> = SetWrapper<'a, T, (), D, E, (), I, ()>;
        /// Requires `T`: [EguiStructMut]
        #[allow(private_interfaces)]
        pub type SetWrapperMinimal<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, ConfigSetExpandableNStore<'b, T>, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperS<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, ConfigSetExpandable<'b, T>, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperD<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, (), ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSD<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, bool, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut]
        #[allow(private_interfaces)]
        pub type SetWrapperI<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, ConfigSetExpandableNStore<'b, T>, ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSI<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, ConfigSetExpandable<'b, T>, ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperDI<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, (), ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperFull<'a, 'b, T, D> =
            SimpleSetWrapper<'a, T, D, bool, ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type MapWrapperFull<'a, 'b, K, V, D> =
            SetWrapper<'a, K, V, D, bool, bool, ConfigSetMutTrueImut<K>, ConfigSetMutTrueImut<V>>;
    }
}

pub use combobox::ComboBox;
pub(crate) mod combobox {
    use dyn_clone::DynClone;

    use super::*;
    pub struct ComboBox<'a, T>(MaybeOwned<'a, T>);

    #[allow(private_bounds)]
    impl<'a, T> ComboBox<'a, T> {
        pub fn new(inner: T) -> Self {
            Self(MaybeOwned::Owned(inner))
        }
        pub fn new_mut(inner: &'a mut T) -> Self {
            Self(MaybeOwned::BorrowedMut(inner))
        }
        pub fn new_ref(inner: &'a T) -> Self {
            Self(MaybeOwned::Borrowed(inner))
        }
    }
    impl<T: ToString> EguiStructImut for ComboBox<'_, T> {
        type ConfigTypeImut<'a> = ConfigStrImut;

        fn show_primitive_imut(
            self: &Self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeImut<'_>,
        ) -> Response {
            self.0.to_string().show_primitive_imut(ui, config)
        }
    }
    // impl<T: Clone + PartialEq> EguiStructResettable for Combobox<T> {
    //     type Reset2 = T;

    //     fn reset2(&mut self, source: &Self::Reset2) {
    //         self.0.clone_from(&source)
    //     }

    //     fn reset_possible(&self, rhs: &Self::Reset2) -> bool {
    //         self.0.eq(&rhs)
    //     }
    // }
    impl<T: Clone> EguiStructClone for ComboBox<'_, T> {
        fn eguis_clone(&mut self, source: &Self) {
            self.0.clone_from(&source.0)
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            Some(ComboBox(MaybeOwned::Owned(self.0.clone())))
        }
    }
    impl<T: PartialEq> EguiStructEq for ComboBox<'_, T> {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            self.0.eq(&rhs.0)
        }
    }
    impl<T: Clone + ToString + PartialEq + 'static> EguiStructMut for ComboBox<'_, T> {
        type ConfigTypeMut<'a> = Option<&'a dyn IteratorClone<T>>;

        fn show_primitive_mut(
            self: &mut Self,
            ui: &mut ExUi,
            config: &Self::ConfigTypeMut<'_>,
        ) -> Response {
            show_combobox(self.0.deref_mut(), ui, &config)
        }
    }
    pub trait IteratorClone<T>: Iterator<Item = T> + DynClone {}
    impl<TI, T: Iterator<Item = TI> + DynClone> IteratorClone<TI> for T {}

    pub(crate) fn show_combobox<'a, T: Clone + ToString + PartialEq, TS: Clone + Into<T>>(
        sel: &mut T,
        ui: &mut ExUi,
        config: &Option<&'a dyn IteratorClone<TS>>,
    ) -> Response {
        let id = ui.id();
        let mut inner_response = ui.dummy_response();
        #[cfg(feature = "egui29")]
        let comb = egui::ComboBox::from_id_salt((id, "__EguiStruct_combobox"));
        #[cfg(not(feature = "egui29"))]
        let comb = egui::ComboBox::from_id_source((id, "__EguiStruct_combobox"));
        let ret = comb
            .selected_text(sel.to_string())
            .show_ui(ui, |ui| {
                inner_response.layer_id = ui.layer_id();
                if let Some(config) = config {
                    for i in dyn_clone::clone_box(*config) {
                        let s = i.clone().into().to_string();
                        inner_response |= ui.selectable_value(sel, i.into().clone(), s);
                    }
                }
            })
            .response;
        inner_response.layer_id = ui.layer_id();
        ret | inner_response
    }
    impl<T> Deref for ComboBox<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for ComboBox<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<T: Default> Default for ComboBox<'_, T> {
        fn default() -> Self {
            Self(MaybeOwned::Owned(Default::default()))
        }
    }
    impl<T: Clone> Clone for ComboBox<'_, T> {
        fn clone(&self) -> Self {
            Self(MaybeOwned::Owned(self.0.clone()))
        }
    }
    impl<T: Eq> Eq for ComboBox<'_, T> {}
    impl<T: Ord> Ord for ComboBox<'_, T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl<T: PartialEq> PartialEq for ComboBox<'_, T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: PartialOrd> PartialOrd for ComboBox<'_, T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
}
