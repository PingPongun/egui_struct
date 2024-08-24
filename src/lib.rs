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
    pub use crate::traits::{
        EguiStructClone, EguiStructEq, EguiStructImut, EguiStructImutInner, EguiStructMut,
        EguiStructMutInner,
    };
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
