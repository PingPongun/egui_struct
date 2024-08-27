//! Crate consists of 4 traits ([`EguiStructImut`] & [`EguiStructMut`]: [`EguiStructEq`]+[`EguiStructClone`]) and two derive macros ([`macro@EguiStructImut`] to derive [`EguiStructImut`] & [`macro@EguiStructMut`] to derive the other three).
//!
//! See [demo](https://github.com/PingPongun/egui_struct/tree/master/demo)
#[macro_use]
mod trait_impls;
mod traits;
mod types;
pub mod prelude {
    pub use crate::traits::{EguiStruct, EguiStructWrapper};
    pub use crate::types::*;
    pub use egui_struct_macros::*;
}

pub mod trait_implementor_set {
    pub use crate::traits::{EguiStructClone, EguiStructEq, EguiStructImut, EguiStructMut};

    use crate::egui::{Response, RichText};
    use exgrid::ExUi;

    pub fn primitive_label(
        ui: &mut ExUi,
        label: impl Into<RichText> + Clone,
        hint: impl Into<RichText> + Clone,
    ) {
        let lab = ui.extext(label);
        let hint = hint.into();
        if !hint.is_empty() {
            lab.on_hover_text(hint);
        }
    }

    pub fn primitive_w_reset<T: EguiStructMut + ?Sized>(
        data: &mut T,
        ui: &mut ExUi,
        config: T::ConfigTypeMut<'_>,
        reset2: Option<&T>,
    ) -> Response {
        let mut ret = data.show_primitive_mut(ui, config);
        if let Some(reset2) = reset2 {
            if !reset2.eguis_eq(data) {
                let mut r = ui.button("⟲");
                if r.clicked() {
                    data.eguis_clone(reset2);
                    r.mark_changed();
                }
                ret |= r;
            }
        }
        ret
    }
}
pub use exgrid;

#[cfg(feature = "egui21")]
use egui21 as egui;
#[cfg(feature = "egui22")]
use egui22 as egui;
#[cfg(feature = "egui23")]
use egui23 as egui;
#[cfg(feature = "egui24")]
use egui24 as egui;
#[cfg(feature = "egui25")]
use egui25 as egui;
#[cfg(feature = "egui26")]
use egui26 as egui;
#[cfg(feature = "egui27")]
use egui27 as egui;
#[cfg(feature = "egui28")]
use egui28 as egui;
