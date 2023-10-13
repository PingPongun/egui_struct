cargo expand --all-features --bin egui_struct_demo | out-file translate/expanded.rs -encoding utf8
cargo i18n ./translate
