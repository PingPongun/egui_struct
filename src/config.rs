use crate::traits::EguiStructMut;
use crate::wrappers::combobox::IteratorClone;
use std::any::Any;

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
    ComboBox(&'a dyn IteratorClone<T>),
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
    ComboBox(&'a dyn IteratorClone<&'a str>),
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

/// Configuration options for mutable sets (IndexSet, Vec, ..)
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
