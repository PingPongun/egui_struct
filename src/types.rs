use crate::egui;
use egui::{Response, Ui};
use exgrid::ExUi;
use std::hash::Hash;
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

///Config structure for immutable view of many simple types like str, String & numerics
#[derive(Default)]
pub enum ConfigStrImut {
    ///`egui::Label`
    NonSelectable,

    ///Default: imutable `egui::TextEdit`
    #[default]
    Selectable,
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
            config: Self::ConfigTypeImut<'_>,
            _id: impl Hash + Clone,
        ) -> Response {
            self.0.to_string().show_primitive_imut(ui, config, ())
        }
    }
    // impl<T: Clone + PartialEq> EguiStructResetable for Combobox<T> {
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
            config: Self::ConfigTypeMut<'_>,
            id: impl Hash + Clone,
        ) -> Response {
            show_combobox(&mut self.0, ui, config, ui.id())
        }
    }

    pub(crate) fn show_combobox<'a, T: Clone + ToString + PartialEq>(
        sel: &mut T,
        ui: &mut ExUi,
        config: Option<&'a mut dyn Iterator<Item = T>>,
        id: impl Hash + Clone,
    ) -> Response {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            let mut inner_response =
                ui.allocate_response(egui::vec2(0.0, 0.0), egui::Sense::hover());
            let ret = egui::ComboBox::from_id_source((id, "__EguiStruct_combobox"))
                .selected_text(sel.to_string())
                .show_ui(ui, |ui| {
                    if let Some(config) = config {
                        for i in config {
                            let s = i.to_string();
                            inner_response |= ui.selectable_value(sel, i, s);
                        }
                    }
                })
                .response;
            ret | inner_response
        })
        .inner
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
