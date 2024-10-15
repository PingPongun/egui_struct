use crate::traits::EguiStructMut;
use crate::wrappers::combobox::IteratorClone;
use std::any::Any;

/// Config structure for mutable view of Numerics
#[derive(Default)]
#[non_exhaustive]
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
    ComboBox(&'a dyn IteratorClone<T>),
}

///Config structure for mutable view of String
#[derive(Default)]
#[non_exhaustive]
pub enum ConfigStr<'a> {
    ///Default: single line `egui::TextEdit`
    #[default]
    SingleLine,

    ///multi line `egui::TextEdit`
    MultiLine,

    ///Combobox with available options specified by included iterator
    ComboBox(&'a dyn IteratorClone<&'a str>),
}

/// Config structure for immutable view of many simple types like str, String & numerics
#[derive(Default)]
#[non_exhaustive]
pub enum ConfigStrImut {
    /// `egui::Label`
    NonSelectable,

    /// Default: immutable `egui::TextEdit`
    #[default]
    Selectable,
}

/// Configuration options for mutable Collections (IndexSet, IndexMap, Vec, ..)
#[non_exhaustive]
pub struct ConfigCollMut<
    'a,
    K: EguiStructMut,
    V: EguiStructMut,
    EK: ConfigCollExpandableT<K>,
    EV: ConfigCollExpandableT<V>,
> {
    /// Can new elements be added to set
    pub expandable: Option<(EK, EV)>,

    /// Can elements be removed from set
    pub shrinkable: bool,

    /// Can element value be changed after adding (ignored for [std::collections::HashSet])
    pub mutable_value: bool,

    /// Can element key be changed after adding (Applicable only for [indexmap::IndexMap])
    pub mutable_key: bool,

    /// Maximum number of elements in set
    pub max_len: Option<usize>,

    /// Config how elements are shown
    pub inner_config: (K::ConfigTypeMut<'a>, V::ConfigTypeMut<'a>),

    /// Can elements be reordered? (ignored for [std::collections::HashSet]/[std::collections::HashMap])
    pub reorder: bool,
}

impl<
        'a,
        K: EguiStructMut,
        V: EguiStructMut,
        EK: ConfigCollExpandableT<K>,
        EV: ConfigCollExpandableT<V>,
    > ConfigCollMut<'a, K, V, EK, EV>
{
    /// Same as Self::default()
    pub fn new() -> Self {
        Default::default()
    }

    /// Set value of [Self::shrinkable](Self#structfield.shrinkable)
    pub fn shrinkable(mut self, shrinkable: bool) -> Self {
        self.shrinkable = shrinkable;
        self
    }
    /// Sets value of [Self::mutable_key]
    pub fn mut_key(mut self, mut_key: bool) -> Self {
        self.mutable_key = mut_key;
        if typeid::of::<V>() == typeid::of::<()>() {
            self.mutable_value = mut_key;
        }
        self
    }
    /// Sets value of [Self::mutable_value]
    pub fn mut_val(mut self, mut_val: bool) -> Self {
        self.mutable_value = mut_val;
        self
    }
    /// Set value of [Self::max_len](Self#structfield.max_len)
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }
    /// Set value of [Self::reorder](Self#structfield.reorder)
    pub fn reorder(mut self, reorder: bool) -> Self {
        self.reorder = reorder;
        self
    }

    /// Sets value of [Self::expandable](Self#structfield.expandable)
    pub fn expandable(mut self, expandable: Option<(EK, EV)>) -> Self {
        self.expandable = expandable;
        self
    }
    /// Sets value of [Self::inner_config]
    pub fn config(
        mut self,
        config_k: K::ConfigTypeMut<'a>,
        config_v: V::ConfigTypeMut<'a>,
    ) -> Self {
        self.inner_config = (config_k, config_v);
        self
    }
}
impl<'a, K: EguiStructMut, EK: ConfigCollExpandableT<K>> ConfigCollMut<'a, K, (), EK, ()> {
    /// Simpler version of [Self::expandable] for Sets/Vec (where `V == ()`)
    /// Sets value of [Self::expandable](Self#structfield.expandable)
    pub fn expandable_set(mut self, expandable: Option<EK>) -> Self {
        self.expandable = expandable.map(|x| (x, ()));
        self
    }
    /// Simpler version of [Self::config] for Sets/Vec (where `V == ()`)
    /// Sets value of [Self::inner_config]
    pub fn config_set(mut self, config_k: K::ConfigTypeMut<'a>) -> Self {
        self.inner_config = (config_k, ());
        self
    }
}

impl<
        'a,
        K: EguiStructMut,
        V: EguiStructMut,
        EK: ConfigCollExpandableT<K>,
        EV: ConfigCollExpandableT<V>,
    > Default for ConfigCollMut<'a, K, V, EK, EV>
{
    fn default() -> Self {
        Self {
            expandable: EK::default_config().zip(EV::default_config()),
            shrinkable: true,
            mutable_value: true,
            mutable_key: true,
            max_len: None,
            inner_config: Default::default(),
            reorder: true,
        }
    }
}

use config_coll_expandable::*;
pub mod config_coll_expandable {

    use super::*;
    pub trait ConfigCollExpandableT<T> {
        fn mutable(&self) -> bool {
            false
        }
        fn default_value(&self) -> T;
        fn default_config() -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }

    /// Configuration struct that controls adding new elements to set. Used for `T: Send+Any`
    pub struct ConfigCollExpandable<'a, T> {
        /// Function that generates new value that will be added to collection (on `+` button click).
        /// If self.mutable == true generates starting value that can further edited before adding
        pub default: &'a dyn for<'b> Fn() -> T,
        /// Can element be edited prior adding to collection
        pub mutable: bool,
    }
    /// Configuration struct that controls adding new elements to set. Used for `T: !(Send+Any)`
    pub struct ConfigCollExpandableNStore<'a, T> {
        /// Function that generates new value that will be added to collection (on `+` button click).
        pub default: &'a dyn for<'b> Fn() -> T,
    }

    impl<T: Send + Any> ConfigCollExpandableT<T> for ConfigCollExpandable<'_, T> {
        fn mutable(&self) -> bool {
            self.mutable
        }

        fn default_value(&self) -> T {
            (self.default)()
        }
    }
    impl<T> ConfigCollExpandableT<T> for ConfigCollExpandableNStore<'_, T> {
        fn default_value(&self) -> T {
            (self.default)()
        }
    }
    impl<T: Default + Send + Any> ConfigCollExpandableT<T> for bool {
        fn mutable(&self) -> bool {
            *self
        }

        fn default_value(&self) -> T {
            T::default()
        }
        fn default_config() -> Option<Self> {
            Some(true)
        }
    }
    impl<T: Default> ConfigCollExpandableT<T> for () {
        fn default_value(&self) -> T {
            T::default()
        }

        fn default_config() -> Option<Self> {
            Some(())
        }
    }
}
