use crate::egui;
use crate::traits::{EguiStructImut, EguiStructMut};
use egui::Response;
use exgrid::ExUi;
use std::any::Any;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
/// Config structure for mutable view of Numerics
#[derive(Default)]
pub enum ConfigNum<'a, T: 'a> {
    /// Default: DragValue (without limits)
    #[default]
    NumDefault,

    /// DragValue(min, max)
    DragValue(T, T),

    /// Slider(min, max)
    Slider(T, T),

    /// Slider(min, max, step)
    SliderStep(T, T, T),

    /// Combobox with available options specified by included iterator
    ComboBox(&'a mut dyn Iterator<Item = T>),
}

///Config structure for mutable view of String
#[derive(Default)]
pub enum ConfigStr<'a> {
    ///Default: single line `egui::TextEdit`
    #[default]
    SingleLine,

    ///multi line `egui::TextEdit`
    MultiLine,

    ///Combobox with available options specified by included iterator
    ComboBox(&'a mut dyn Iterator<Item = String>),
}

/// Config structure for immutable view of many simple types like str, String & numerics
#[derive(Default)]
pub enum ConfigStrImut {
    /// `egui::Label`
    NonSelectable,

    /// Default: immutable `egui::TextEdit`
    #[default]
    Selectable,
}

pub mod set {
    use super::*;
    /// Configuration options for mutable sets (IndexSet, Vec, ..)
    pub struct ConfigSetMut<'a, T: EguiStructMut, E> {
        /// Can new elements be added to set
        pub expandable: Option<E>,

        /// Can elements be removed from set
        pub shrinkable: bool,

        /// Can element value be changed after adding (ignored for [std::collections::HashSet])
        pub mutable_value: bool,

        /// Can element key be changed after adding (Applicable only for [indexmap::IndexMap])
        pub mutable_key: bool,

        /// Maximum number of elements in set
        pub max_len: Option<usize>,

        /// Config how elements are shown
        pub inner_config: T::ConfigTypeMut<'a>,

        /// Can elements be reordered? (ignored for [std::collections::HashSet]/[std::collections::HashMap])
        pub reorder: bool,
    }

    impl<'a, T: EguiStructMut, E> Default for ConfigSetMut<'a, T, E> {
        fn default() -> Self {
            Self {
                expandable: None,
                shrinkable: true,
                mutable_value: true,
                mutable_key: true,
                max_len: None,
                inner_config: Default::default(),
                reorder: true,
            }
        }
    }

    pub(crate) use config_set_imut_t::*;
    mod config_set_imut_t {
        use super::*;
        pub struct ConfigSetMutTrueImut<T>(PhantomData<T>);
        pub struct ConfigSetMutDisableMut<T>(PhantomData<T>);
        pub(crate) trait ConfigSetImutT<T: EguiStructMut> {
            fn _show_childs_imut(val: &mut T, ui: &mut ExUi) -> Response;
            fn _show_primitive_imut(val: &mut T, ui: &mut ExUi) -> Response;
        }
        impl<T: EguiStructMut + EguiStructImut> ConfigSetImutT<T> for ConfigSetMutTrueImut<T> {
            fn _show_childs_imut(val: &mut T, ui: &mut ExUi) -> Response {
                val.show_childs_imut(ui, &mut Default::default(), None)
            }
            fn _show_primitive_imut(val: &mut T, ui: &mut ExUi) -> Response {
                val.show_primitive_imut(ui, &mut Default::default())
            }
        }
        impl<T: EguiStructMut> ConfigSetImutT<T> for ConfigSetMutDisableMut<T> {
            fn _show_childs_imut(val: &mut T, ui: &mut ExUi) -> Response {
                ui.start_disabled();
                let ret = val.show_childs_mut(ui, &mut Default::default(), None);
                ui.stop_disabled();
                ret
            }
            fn _show_primitive_imut(val: &mut T, ui: &mut ExUi) -> Response {
                ui.start_disabled();
                let ret = val.show_primitive_mut(ui, &mut Default::default());
                ui.stop_disabled();
                ret
            }
        }
    }
    pub use config_set_expandable_t::*;
    mod config_set_expandable_t {
        use super::*;
        pub(crate) trait ConfigSetExpandableT<T> {
            fn mutable(&self) -> bool;
            fn default_value(&self) -> T;
        }
        /// Configuration struct that controls adding new elements to set. Used for `T: Send+Any`
        pub struct ConfigSetExpandable<'a, T> {
            /// Function that generates new value that will be added to collection (on `+` button click).
            /// If self.mutable == true generates starting value that can further edited before adding
            pub default: &'a dyn for<'b> Fn() -> T,
            /// Can element be edited prior adding to collection
            pub mutable: bool,
        }
        /// Configuration struct that controls adding new elements to set. Used for `T: !(Send+Any)`
        pub struct ConfigSetExpandableNStore<'a, T> {
            /// Function that generates new value that will be added to collection (on `+` button click).
            pub default: &'a dyn for<'b> Fn() -> T,
        }
        impl<T: Send + Any> ConfigSetExpandableT<T> for ConfigSetExpandable<'_, T> {
            fn mutable(&self) -> bool {
                self.mutable
            }

            fn default_value(&self) -> T {
                (self.default)()
            }
        }
        impl<T> ConfigSetExpandableT<T> for ConfigSetExpandableNStore<'_, T> {
            fn mutable(&self) -> bool {
                false
            }

            fn default_value(&self) -> T {
                (self.default)()
            }
        }
        impl<T: Default + Send + Any> ConfigSetExpandableT<T> for bool {
            fn mutable(&self) -> bool {
                *self
            }

            fn default_value(&self) -> T {
                T::default()
            }
        }
        impl<T: Default> ConfigSetExpandableT<T> for () {
            fn mutable(&self) -> bool {
                false
            }

            fn default_value(&self) -> T {
                T::default()
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
        pub trait SetWrapperT<T>: FromIterator<T> {
            fn len(&self) -> usize;
            fn new() -> Self;
            fn get(&self, idx: usize) -> Option<&T>;
            fn remove(&mut self, idx: usize);
            fn truncate(&mut self, len: usize);
            fn swap(&mut self, idx: (usize, usize));
            fn push(&mut self, val: T);
            fn drain(&mut self) -> impl Iterator<Item = T>;
            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
            where
                T: 'a;
        }
        impl<T> SetWrapperT<T> for Vec<T> {
            fn len(&self) -> usize {
                self.len()
            }

            fn new() -> Self {
                Self::new()
            }

            fn get(&self, idx: usize) -> Option<&T> {
                self.deref().get(idx)
            }

            fn remove(&mut self, idx: usize) {
                self.remove(idx);
            }

            fn truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn swap(&mut self, idx: (usize, usize)) {
                self.deref_mut().swap(idx.0, idx.1);
            }

            fn push(&mut self, value: T) {
                self.push(value);
            }

            fn drain(&mut self) -> impl Iterator<Item = T> {
                self.drain(..)
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
            where
                T: 'a,
            {
                self.deref().iter()
            }
        }

        #[cfg(feature = "indexmap")]
        impl<T: Hash + Eq> SetWrapperT<T> for indexmap::IndexSet<T> {
            fn len(&self) -> usize {
                self.len()
            }

            fn new() -> Self {
                Self::new()
            }

            fn get(&self, idx: usize) -> Option<&T> {
                self.get_index(idx)
            }

            fn remove(&mut self, idx: usize) {
                self.shift_remove_index(idx);
            }

            fn truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn swap(&mut self, idx: (usize, usize)) {
                self.swap_indices(idx.0, idx.1);
            }

            fn push(&mut self, value: T) {
                self.insert(value);
            }

            fn drain(&mut self) -> impl Iterator<Item = T> {
                self.drain(..)
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
            where
                T: 'a,
            {
                self.iter()
            }
        }
    }
    pub(crate) use _set_wrapper::*;
    mod _set_wrapper {
        use super::*;
        #[allow(private_interfaces, private_bounds)]
        /// Thin wrapper around [Vec]/[indexmap::IndexSet], that provides generic configured [EguiStructMut] implementation for [Vec]/[indexmap::IndexSet].
        ///
        /// Different generics combination provide slightly different feature set, but allows to loosen bounds on `T`
        ///
        /// See [set_wrappers].
        pub struct SetWrapper<
            'a,
            T: EguiStructMut,
            D: SetWrapperT<T>,
            E: ConfigSetExpandableT<T>,
            I: ConfigSetImutT<T>,
        >(pub MaybeOwned<'a, D>, PhantomData<(T, E, I)>);

        #[allow(private_bounds)]
        impl<
                'a,
                T: EguiStructMut,
                D: SetWrapperT<T>,
                E: ConfigSetExpandableT<T>,
                I: ConfigSetImutT<T>,
            > SetWrapper<'a, T, D, E, I>
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
                T: EguiStructMut,
                D: SetWrapperT<T>,
                E: ConfigSetExpandableT<T>,
                I: ConfigSetImutT<T>,
            > Deref for SetWrapper<'_, T, D, E, I>
        {
            type Target = D;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<
                T: EguiStructMut,
                D: SetWrapperT<T>,
                E: ConfigSetExpandableT<T>,
                I: ConfigSetImutT<T>,
            > DerefMut for SetWrapper<'_, T, D, E, I>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
    pub(crate) use config_set_t::*;
    mod config_set_t {
        use super::*;
        pub(crate) trait ConfigSetT<T: EguiStructMut, E> {
            fn _add_elements(
                &mut self,
                ui: &mut ExUi,
                config: &mut ConfigSetMut<'_, T, E>,
            ) -> Response;
        }
        macro_rules! _add_elements_send {
    ($typ:ty, [$($bound:ident),*]) => {
        impl<T: EguiStructMut $(+ $bound)*, D: SetWrapperT<T>, I: ConfigSetImutT<T>> ConfigSetT<T,$typ>
            for SetWrapper<'_, T, D, $typ, I>
        {
            fn _add_elements(
                &mut self,
                ui: &mut ExUi,
                config: &mut ConfigSetMut<'_, T,  $typ>,
            ) -> Response {
                let mut response = ui.dummy_response();
                if let Some(add) = &config.expandable {
                    if config.max_len.is_none() || self.0.len() < config.max_len.unwrap() {
                        if <$typ as ConfigSetExpandableT<T>>::mutable(add) {
                            let id = ui.id();
                            let mut val: Box<T> = ui
                                .data_remove(id)
                                .unwrap_or_else(|| Box::new(add.default_value()));
                            let mut add_elem = false;
                            let resp = ui
                                .maybe_collapsing_rows(val.has_childs_mut(), |ui| {
                                    let bresp = ui.button("+");
                                    let presp =
                                        val.show_primitive_mut(ui, &mut config.inner_config);
                                    add_elem = bresp.clicked();
                                    bresp | presp
                                })
                                .body_simple(|ui| {
                                    val.show_childs_mut(ui, &mut config.inner_config, None)
                                });
                            response = resp.clone();
                            if add_elem {
                                self.0.push(*val);
                            } else {
                                ui.data_store(id, val);
                            }
                        } else {
                            let bresp = ui.button("+");
                            ui.end_row();
                            response = bresp.clone();
                            if bresp.clicked() {
                                let new_val = add.default_value();
                                self.0.push(new_val);
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
    ($typ:ty, [$($bound:ident),*]) => {
        impl<T: EguiStructMut $(+ $bound)*, D: SetWrapperT<T>, I: ConfigSetImutT<T>> ConfigSetT<T, $typ>
            for SetWrapper<'_, T, D, $typ, I>
        {
            fn _add_elements(
                &mut self,
                ui: &mut ExUi,
                config: &mut ConfigSetMut<'_, T,  $typ>,
            ) -> Response {
                let mut response = ui.dummy_response();
                if let Some(add) = &config.expandable {
                    if config.max_len.is_none() || self.0.len() < config.max_len.unwrap() {
                        let bresp = ui.button("+");
                        ui.end_row();
                        response = bresp.clone();
                        if bresp.clicked() {
                            let new_val = add.default_value();
                            self.0.push(new_val);
                        }
                    }
                }
                response
            }
        }
    };
}
        _add_elements_nsend! { (),  [Default]}
        _add_elements_nsend! { ConfigSetExpandableNStore<'_, T>, []}
        _add_elements_send! { ConfigSetExpandable<'_, T>, [Send,Any]}
        _add_elements_send! { bool, [Default,Send,Any]}
    }

    pub(crate) use set_wrappers::*;
    pub mod set_wrappers {
        //! [Vec]/[indexmap::IndexSet] wrappers that allow to get [EguiStructMut] implementation for [Vec]/[indexmap::IndexSet] with looser bounds
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
        //! [EguiStructMut] for [Vec]/[indexmap::IndexSet] is implemented using [SetWrapperFull]
        //!
        use super::*;
        pub use _set_wrapper::SetWrapper;
        /// Requires `T`: [EguiStructMut]
        #[allow(private_interfaces)]
        pub type SetWrapperMinimal<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigSetExpandableNStore<'b, T>, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperS<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigSetExpandable<'b, T>, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperD<'a, 'b, T, D> = SetWrapper<'a, T, D, (), ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSD<'a, 'b, T, D> = SetWrapper<'a, T, D, bool, ConfigSetMutDisableMut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut]
        #[allow(private_interfaces)]
        pub type SetWrapperI<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigSetExpandableNStore<'b, T>, ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperSI<'a, 'b, T, D> =
            SetWrapper<'a, T, D, ConfigSetExpandable<'b, T>, ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default]
        #[allow(private_interfaces)]
        pub type SetWrapperDI<'a, 'b, T, D> = SetWrapper<'a, T, D, (), ConfigSetMutTrueImut<T>>;

        /// Requires `T`: [EguiStructMut] + [EguiStructImut] + [Default] + [Any] + [Send]
        #[allow(private_interfaces)]
        pub type SetWrapperFull<'a, 'b, T, D> = SetWrapper<'a, T, D, bool, ConfigSetMutTrueImut<T>>;
    }
}
//////////////////////////////////////////////////////////////
pub use combobox::Combobox;
pub(crate) mod combobox {
    use crate::traits::*;
    use crate::types::*;
    pub struct Combobox<T>(pub T);

    impl<T: ToString> EguiStructImut for Combobox<T> {
        type ConfigTypeImut<'a> = ConfigStrImut;

        fn show_primitive_imut(
            self: &Self,
            ui: &mut ExUi,
            config: &mut Self::ConfigTypeImut<'_>,
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
    impl<T: Clone> EguiStructClone for Combobox<T> {
        fn eguis_clone(&mut self, source: &Self) {
            self.0.clone_from(&source.0)
        }

        fn eguis_clone_full(&self) -> Option<Self> {
            Some(Combobox(self.0.clone()))
        }
    }
    impl<T: PartialEq> EguiStructEq for Combobox<T> {
        fn eguis_eq(&self, rhs: &Self) -> bool {
            self.0.eq(&rhs.0)
        }
    }
    impl<T: Clone + ToString + PartialEq + 'static> EguiStructMut for Combobox<T> {
        type ConfigTypeMut<'a> = Option<&'a mut dyn Iterator<Item = T>>;

        fn show_primitive_mut(
            self: &mut Self,
            ui: &mut ExUi,
            config: &mut Self::ConfigTypeMut<'_>,
        ) -> Response {
            show_combobox(&mut self.0, ui, config)
        }
    }

    pub(crate) fn show_combobox<'a, T: Clone + ToString + PartialEq>(
        sel: &mut T,
        ui: &mut ExUi,
        config: &mut Option<&'a mut dyn Iterator<Item = T>>,
    ) -> Response {
        let id = ui.id();
        let mut inner_response = ui.dummy_response();
        let ret = egui::ComboBox::from_id_source((id, "__EguiStruct_combobox"))
            .selected_text(sel.to_string())
            .show_ui(ui, |ui| {
                inner_response.layer_id = ui.layer_id();
                if let Some(config) = config {
                    for i in config {
                        let s = i.to_string();
                        inner_response |= ui.selectable_value(sel, i, s);
                    }
                }
            })
            .response;
        inner_response.layer_id = ui.layer_id();
        ret | inner_response
    }
    impl<T> Deref for Combobox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for Combobox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<T: Default> Default for Combobox<T> {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl<T: Clone> Clone for Combobox<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: Copy> Copy for Combobox<T> {}
    impl<T: Eq> Eq for Combobox<T> {}
    impl<T: Ord> Ord for Combobox<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl<T: PartialEq> PartialEq for Combobox<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<T: PartialOrd> PartialOrd for Combobox<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
}
