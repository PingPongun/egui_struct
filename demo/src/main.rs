#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
use app::DemoApp;

rust_i18n::i18n!("./translate", fallback = "en");
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_struct demo",
        native_options,
        Box::new(|_creation_context| Ok(Box::<DemoApp>::default())),
    )
}
