//! Module provides wrappers that allow to use alternative EguiStruct* traits implementations (eg. with loosen bounds or changed functionality)
//! or use types that does not impl them
//!
//! # Collection (Vec/Set/Map) wrappers
//!
//! Collection wrappers that allow to get [EguiStructMut] implementation for [Vec]/[IndexSet]/[IndexMap] with looser bounds
//!
//! There are available generic wrapper [`CollWrapper<K, V, D, EK, EV, IK, IV>`](CollWrapper),
//! wrapper around it for Sets/Vec ([`SetWrapper<K, D, EK, IK>`](SetWrapper)),
//! and simple wrappers for specific requirements (eg. [`SetWrapperFull<K>`](SetWrapperFull), [`SetWrapperSD<K>`](SetWrapperSD), [`CollWrapperFull<K,V>`](CollWrapperFull)).
//! Last group should be preferred one.
//!
//! ## Simple SetWrapper naming:
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
//! | ✅ | ❌ | [ConfigCollExpandable]                  |
//! | ❌ | ❌ | [ConfigCollExpandableNStore]            |
//!
//! [EguiStructMut] for [Vec]/[IndexSet] is implemented using [SetWrapperFull]
//!
//! ## Usage:
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

use crate::config::config_coll_expandable::*;
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

pub use collection::*;
mod collection {
    use super::*;
    pub mod internal_impl {
        //! Module contains structs & traits that underlie [CollWrapper] implementation.
        //! It is only required if You intend to use [CollWrapper] with our own Collections
        pub use super::coll_wrapper_t::*;
        pub use super::config_coll_imut_t::*;
        #[cfg(doc)]
        use super::*;
    }
    pub(crate) use config_coll_imut_t::*;
    mod config_coll_imut_t {
        use super::*;
        pub struct ConfigCollMutTrueImut<T>(PhantomData<T>);
        pub struct ConfigCollMutDisableMut<T>(PhantomData<T>);
        pub trait ConfigCollImutT<T: EguiStructMut> {
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
        impl<T: EguiStructMut + EguiStructImut> ConfigCollImutT<T> for () {
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
        impl<T: EguiStructMut + EguiStructImut> ConfigCollImutT<T> for ConfigCollMutTrueImut<T> {
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
        impl<T: EguiStructMut> ConfigCollImutT<T> for ConfigCollMutDisableMut<T> {
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

    pub(crate) use coll_wrapper_t::*;
    mod coll_wrapper_t {

        use std::collections::{HashMap, HashSet};

        use super::*;
        macro_rules! impl_show_key {
            () => {
                fn key_show_primitive<IK: ConfigCollImutT<K>>(
                    val: &mut Self::KeyRef<'_>,
                    mutable: bool,
                    ui: &mut ExUi,
                    config: &K::ConfigTypeMut<'_>,
                    reset2: Option<&K>,
                ) -> Response
                where
                    K: EguiStructMut,
                {
                    if mutable {
                        crate::trait_implementor_set::primitive_w_reset(*val, ui, config, reset2)
                    } else {
                        IK::_show_primitive_imut(val, ui, &Default::default())
                    }
                }
                fn key_show_childs<IK: ConfigCollImutT<K>>(
                    val: &mut Self::KeyRef<'_>,
                    mutable: bool,
                    ui: &mut ExUi,
                    config: &K::ConfigTypeMut<'_>,
                    reset2: Option<&K>,
                ) -> Response
                where
                    K: EguiStructMut,
                {
                    if mutable {
                        val.show_childs_mut(ui, config, reset2)
                    } else {
                        IK::_show_childs_imut(val, ui, &Default::default())
                    }
                }
            };
        }
        macro_rules! impl_show_key_imut {
            () => {
                fn key_show_primitive<IK: ConfigCollImutT<K>>(
                    val: &mut Self::KeyRef<'_>,
                    _mutable: bool,
                    ui: &mut ExUi,
                    _config: &K::ConfigTypeMut<'_>,
                    _reset2: Option<&K>,
                ) -> Response
                where
                    K: EguiStructMut,
                {
                    val.show_primitive_imut(ui, &Default::default())
                }
                fn key_show_childs<IK: ConfigCollImutT<K>>(
                    val: &mut Self::KeyRef<'_>,
                    _mutable: bool,
                    ui: &mut ExUi,
                    _config: &K::ConfigTypeMut<'_>,
                    _reset2: Option<&K>,
                ) -> Response
                where
                    K: EguiStructMut,
                {
                    val.show_childs_imut(ui, &Default::default(), None)
                }
            };
        }
        pub trait CollWrapperT<K, V> {
            const REORDERABLE: bool = true;
            type KeyRef<'a>: Deref<Target = K>
            where
                K: 'a;
            fn e_len(&self) -> usize;
            fn e_new() -> Self;
            fn e_get(&self, idx: usize) -> Option<(&K, &V)>;
            fn e_truncate(&mut self, len: usize);
            fn e_swap(&mut self, idx: (usize, usize));
            fn e_push(&mut self, val: (K, V));
            fn e_drain(&mut self) -> impl Iterator<Item = (K, V)>;
            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
            where
                K: 'a,
                V: 'a;

            fn e_map<'a>(
                &'a mut self,
                op: impl for<'b> FnMut(usize, (Self::KeyRef<'b>, &'b mut V)) -> bool,
            ) where
                K: 'a,
                V: 'a;

            fn e_from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self;

            fn key_show_primitive<IK: ConfigCollImutT<K>>(
                val: &mut Self::KeyRef<'_>,
                mutable: bool,
                ui: &mut ExUi,
                config: &K::ConfigTypeMut<'_>,
                reset2: Option<&K>,
            ) -> Response
            where
                K: EguiStructMut;

            fn key_show_childs<IK: ConfigCollImutT<K>>(
                val: &mut Self::KeyRef<'_>,
                mutable: bool,
                ui: &mut ExUi,
                config: &K::ConfigTypeMut<'_>,
                reset2: Option<&K>,
            ) -> Response
            where
                K: EguiStructMut;
        }

        impl<K> CollWrapperT<K, ()> for Vec<K> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&K, &())> {
                self.deref().get(idx).map(|x| (x, &()))
            }

            fn e_truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn e_swap(&mut self, idx: (usize, usize)) {
                self.deref_mut().swap(idx.0, idx.1);
            }

            fn e_push(&mut self, value: (K, ())) {
                self.push(value.0);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (K, ())> {
                self.drain(..).map(|x| (x, ()))
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a ())>
            where
                K: 'a,
            {
                self.deref().iter().map(|x| (x, &()))
            }

            fn e_from_iter<I: IntoIterator<Item = (K, ())>>(iterable: I) -> Self {
                iterable.into_iter().map(|(x, _)| x).collect()
            }

            type KeyRef<'a> = &'a mut K where K: 'a;

            fn e_map<'a>(
                &'a mut self,
                mut op: impl for<'b> FnMut(usize, (&'b mut K, &'b mut ())) -> bool,
            ) where
                K: 'a,
            {
                let mut idx = 0;
                self.retain_mut(|x| {
                    let o = op(idx, (x, &mut ()));
                    idx += 1;
                    o
                })
            }
            impl_show_key! {}
        }

        #[cfg(feature = "indexmap")]
        impl<K: Hash + Eq> CollWrapperT<K, ()> for indexmap::IndexSet<K> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&K, &())> {
                self.get_index(idx).map(|x| (x, &()))
            }

            fn e_truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn e_swap(&mut self, idx: (usize, usize)) {
                self.swap_indices(idx.0, idx.1);
            }

            fn e_push(&mut self, value: (K, ())) {
                self.insert(value.0);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (K, ())> {
                self.drain(..).map(|x| (x, ()))
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a ())>
            where
                K: 'a,
            {
                self.iter().map(|x| (x, &()))
            }
            fn e_from_iter<I: IntoIterator<Item = (K, ())>>(iterable: I) -> Self {
                iterable.into_iter().map(|(x, _)| x).collect()
            }
            type KeyRef<'a> = &'a mut K where K: 'a;

            fn e_map<'a>(
                &'a mut self,
                mut op: impl for<'b> FnMut(usize, (&'b mut K, &'b mut ())) -> bool,
            ) where
                K: 'a,
            {
                let mut idx = 0;
                *self = self
                    .drain(..)
                    .filter_map(|mut x| {
                        let o = op(idx, (&mut x, &mut ()));
                        idx += 1;
                        if o {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            impl_show_key! {}
        }
        #[cfg(feature = "indexmap")]
        impl<K: Hash + Eq, V> CollWrapperT<K, V> for indexmap::IndexMap<K, V> {
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, idx: usize) -> Option<(&K, &V)> {
                self.get_index(idx)
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
            type KeyRef<'a> = &'a mut K where K: 'a;

            fn e_map<'a>(
                &'a mut self,
                mut op: impl for<'b> FnMut(usize, (&'b mut K, &'b mut V)) -> bool,
            ) where
                K: 'a,
            {
                let mut idx = 0;
                *self = self
                    .drain(..)
                    .filter_map(|(mut key, mut val)| {
                        let o = op(idx, (&mut key, &mut val));
                        idx += 1;
                        if o {
                            Some((key, val))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            impl_show_key! {}
        }
        impl<K: Hash + Eq + EguiStructImut, V> CollWrapperT<K, V> for HashMap<K, V> {
            const REORDERABLE: bool = false;
            type KeyRef<'a> = &'a K where K: 'a;
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, _idx: usize) -> Option<(&K, &V)> {
                None
            }

            fn e_truncate(&mut self, len: usize) {
                let mut i = 0;
                self.retain(|_, _| {
                    i += 1;
                    i <= len
                });
            }

            fn e_swap(&mut self, _idx: (usize, usize)) {
                panic!("HashMap does not support indexing!");
            }

            fn e_push(&mut self, value: (K, V)) {
                self.insert(value.0, value.1);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (K, V)> {
                self.drain()
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

            fn e_map<'a>(
                &'a mut self,
                mut op: impl for<'b> FnMut(usize, (&'b K, &'b mut V)) -> bool,
            ) where
                K: 'a,
            {
                let mut idx = 0;
                self.retain(|key, mut val| {
                    let o = op(idx, (&key, &mut val));
                    idx += 1;
                    if o {
                        true
                    } else {
                        false
                    }
                });
            }
            impl_show_key_imut! {}
        }
        impl<K: Hash + Eq + EguiStructImut> CollWrapperT<K, ()> for HashSet<K> {
            const REORDERABLE: bool = false;
            type KeyRef<'a> = &'a K where K: 'a;
            fn e_len(&self) -> usize {
                self.len()
            }

            fn e_new() -> Self {
                Self::new()
            }

            fn e_get(&self, _idx: usize) -> Option<(&K, &())> {
                None
            }

            fn e_truncate(&mut self, len: usize) {
                let mut i = 0;
                self.retain(|_| {
                    i += 1;
                    i <= len
                });
            }

            fn e_swap(&mut self, _idx: (usize, usize)) {
                panic!("HashMap does not support indexing!");
            }

            fn e_push(&mut self, value: (K, ())) {
                self.insert(value.0);
            }

            fn e_drain(&mut self) -> impl Iterator<Item = (K, ())> {
                self.drain().map(|x| (x, ()))
            }

            fn e_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a ())>
            where
                K: 'a,
            {
                self.iter().map(|x| (x, &()))
            }
            fn e_from_iter<I: IntoIterator<Item = (K, ())>>(iterable: I) -> Self {
                iterable.into_iter().map(|x| x.0).collect()
            }

            fn e_map<'a>(
                &'a mut self,
                mut op: impl for<'b> FnMut(usize, (&'b K, &'b mut ())) -> bool,
            ) where
                K: 'a,
            {
                let mut idx = 0;
                self.retain(|key| {
                    let o = op(idx, (&key, &mut ()));
                    idx += 1;
                    if o {
                        true
                    } else {
                        false
                    }
                });
            }
            impl_show_key_imut! {}
        }
    }

    mod _coll_wrapper {
        use super::*;
        #[allow(private_interfaces, private_bounds)]
        /// Thin wrapper around Collections (Vec, HashMaps/HashSets), that provides generic configured [EguiStructMut] implementation for Collections ([Vec], [indexmap::IndexSet], ..).
        ///
        /// Different generics combination provide slightly different feature set, but allows to loosen bounds on `T`
        ///
        /// Generally use aliases to this type ([SetWrapperFull], [SetWrapperI], ..), instead using this type directly.
        ///
        /// See [crate::wrappers] module description.
        pub struct CollWrapper<
            'a,
            K: EguiStructMut,
            V: EguiStructMut,
            D: CollWrapperT<K, V>,
            EK: ConfigCollExpandableT<K>,
            EV: ConfigCollExpandableT<V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        >(pub MaybeOwned<'a, D>, PhantomData<(K, V, EK, EV, IK, IV)>);

        #[allow(private_bounds)]
        impl<
                'a,
                K: EguiStructMut,
                V: EguiStructMut,
                D: CollWrapperT<K, V>,
                EK: ConfigCollExpandableT<K>,
                EV: ConfigCollExpandableT<V>,
                IK: ConfigCollImutT<K>,
                IV: ConfigCollImutT<V>,
            > CollWrapper<'a, K, V, D, EK, EV, IK, IV>
        {
            pub fn new(inner: D) -> Self {
                CollWrapper(MaybeOwned::Owned(inner), PhantomData)
            }
            pub fn new_mut(inner: &'a mut D) -> Self {
                CollWrapper(MaybeOwned::BorrowedMut(inner), PhantomData)
            }
            pub fn new_ref(inner: &'a D) -> Self {
                CollWrapper(MaybeOwned::Borrowed(inner), PhantomData)
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
            > Deref for CollWrapper<'_, K, V, D, EK, EV, IK, IV>
        {
            type Target = D;

            fn deref(&self) -> &Self::Target {
                &self.0
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
            > DerefMut for CollWrapper<'_, K, V, D, EK, EV, IK, IV>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
    pub(crate) use config_coll_t::*;
    mod config_coll_t {
        use super::*;
        use crate::wrappers::collection::_coll_wrapper::CollWrapper;
        pub(crate) trait ConfigCollT<
            K: EguiStructMut,
            V: EguiStructMut,
            EK: ConfigCollExpandableT<K>,
            EV: ConfigCollExpandableT<V>,
        >
        {
            fn _add_elements(
                &mut self,
                ui: &mut ExUi,
                config: &ConfigCollMut<'_, K, V, EK, EV>,
            ) -> Response;
        }
        macro_rules! _add_elements_send {
($typ:ty, [$($bound:ident),*], $typV:ty, [$($boundV:ident),*]) => {
    impl<
            K: EguiStructMut $(+ $bound)*,
            V: EguiStructMut $(+ $boundV)*,
            D: CollWrapperT<K,V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        > ConfigCollT<K,V, $typ, $typV> for CollWrapper<'_, K, V, D, $typ,$typV, IK,IV>
    {
        fn _add_elements(
            &mut self,
            ui: &mut ExUi,
            config: &ConfigCollMut<'_, K,V, $typ, $typV>,
        ) -> Response {
            let mut response = ui.dummy_response();
            if let Some(add) = &config.expandable {
                if config.max_len.is_none() || self.0.e_len() < config.max_len.unwrap() {
                    if <$typ as ConfigCollExpandableT<K>>::mutable(&add.0) {
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
    D: CollWrapperT<K,V>,
    IK: ConfigCollImutT<K>,
    IV: ConfigCollImutT<V>,
> ConfigCollT<K,V, $typ, $typV> for CollWrapper<'_, K, V, D, $typ,$typV, IK,IV>
{
fn _add_elements(
    &mut self,
    ui: &mut ExUi,
    config: &ConfigCollMut<'_, K,V, $typ, $typV>,
) -> Response {
    let mut response = ui.dummy_response();
    if let Some(add) = &config.expandable {
        if config.max_len.is_none() || self.0.e_len() < config.max_len.unwrap() {
            let mut_key=<$typV as ConfigCollExpandableT<V>>::mutable(&add.1);
            let mut_val=<$typ as ConfigCollExpandableT<K>>::mutable(&add.0);
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
            D: CollWrapperT<K,V>,
            IK: ConfigCollImutT<K>,
            IV: ConfigCollImutT<V>,
        > ConfigCollT<K,V, $typ, $typV> for CollWrapper<'_, K, V, D, $typ,$typV, IK,IV>
    {
        fn _add_elements(
            &mut self,
            ui: &mut ExUi,
            config: &ConfigCollMut<'_, K,V, $typ, $typV>,
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
        _add_elements_nsend! { ConfigCollExpandableNStore<'_, K>, [], (), [Default]}
        _add_elements_nsend! { (), [Default], ConfigCollExpandableNStore<'_, V>, []}
        _add_elements_nsend! { (), [Default], (), [Default]}
        _add_elements_nsend! { ConfigCollExpandableNStore<'_, K>, [], ConfigCollExpandableNStore<'_, V>, []}
        _add_elements_send! { ConfigCollExpandable<'_, K>, [Send,Any], (), [Default]}
        _add_elements_send! { ConfigCollExpandable<'_, K>, [Send,Any], ConfigCollExpandableNStore<'_, V>, []}
        _add_elements_send! { bool, [Default,Send,Any], (), [Default]}
        _add_elements_send! { bool, [Default,Send,Any], ConfigCollExpandableNStore<'_, V>, []}
        _add_elements_sendsend! { ConfigCollExpandable<'_, K>, [Send,Any], bool, [Default,Send,Any]}
        _add_elements_sendsend! { bool, [Default,Send,Any],  ConfigCollExpandable<'_, V>, [Send,Any]}
        _add_elements_sendsend! { bool, [Default,Send,Any],  bool, [Default,Send,Any]}
        _add_elements_sendsend! { ConfigCollExpandable<'_, K>, [Send,Any], ConfigCollExpandable<'_, V>, [Send,Any]}
    }

    pub use coll_wrappers::*;
    mod coll_wrappers {
        use super::*;
        pub use _coll_wrapper::CollWrapper;

        /// Generic wrapper around simple collections (Vec&Sets)
        pub type SetWrapper<'a, T, D, E, I> = CollWrapper<'a, T, (), D, E, (), I, ()>;

        /// Requires `T`: [EguiStructMut]
        #[allow(private_interfaces)]
        pub type SetWrapperMinimal<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigCollExpandableNStore<'b, T>, ConfigCollMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperS<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigCollExpandable<'b, T>, ConfigCollMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperD<'a, 'b, T, D> = SetWrapper<'a, T, D, (), ConfigCollMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSD<'a, 'b, T, D> =
            SetWrapper<'a, T, D, bool, ConfigCollMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut]
        #[allow(private_interfaces)]
        pub type SetWrapperI<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigCollExpandableNStore<'b, T>, ConfigCollMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSI<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigCollExpandable<'b, T>, ConfigCollMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperDI<'a, 'b, T, D> = SetWrapper<'a, T, D, (), ConfigCollMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperFull<'a, 'b, T, D> =
            SetWrapper<'a, T, D, bool, ConfigCollMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type CollWrapperFull<'a, 'b, K, V, D> = CollWrapper<
            'a,
            K,
            V,
            D,
            bool,
            bool,
            ConfigCollMutTrueImut<K>,
            ConfigCollMutTrueImut<V>,
        >;
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
