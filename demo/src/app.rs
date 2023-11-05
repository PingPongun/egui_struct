#![allow(dead_code)]
use egui::RichText;
use egui_struct::*;
use rust_i18n::set_locale;
use std::collections::HashMap;
use ConfigNum::*;

#[derive(EguiStruct, Default)]
enum Language {
    #[default]
    English,
    Polish,
}

impl Language {
    fn set_locale(&self) {
        match self {
            Language::English => set_locale("en"),
            Language::Polish => set_locale("pl"),
        }
    }
}
#[derive(EguiStruct, Default)]
pub enum Color {
    #[default]
    Red,

    Green,

    Named(String),

    Named2 {
        name: String,
    },

    #[eguis(resetable(with_expr = ||Color::Custom(255,13,17) ))]
    Custom(u8, u8, #[eguis(config = "DragValue(1,111)")] u8),

    #[eguis(skip, rename = "Skipped Custom")]
    SkippedCustom(u8, u8, u8),

    #[eguis(skip)]
    SkippedNamedCustom {
        red: u8,
        blue: u8,
        green: u8,
        metadata: Metadata,
    },

    #[eguis(rename = "Renamed Custom", hint = "This is named custom")]
    NamedCustom {
        red: u8,
        blue: u8,
        green: u8,
        metadata: Metadata,
    },
}

#[derive(EguiStruct)]
#[eguis(rename_all = "Sentence", resetable = "struct_default")]
pub struct Data {
    #[eguis(skip)]
    skipped_data: u32,

    #[eguis(on_change = "Language::set_locale")]
    app_language: Language,

    hashmap: std::collections::HashMap<String, String>,

    #[eguis(resetable(with_expr = "Resetable with expr".to_string()))]
    string: String,

    #[eguis(resetable = "not_resetable")]
    not_resetable_string: String,

    #[eguis(resetable = "field_default")]
    i8: i8,

    i16: i16,

    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,

    #[eguis(imut, hint = "This is isize but immutable")]
    isize_imut: isize,

    #[eguis(
        hint = "This is also isize but limited to range <5,11>",
        config = "Slider(5,11)"
    )]
    limited_isize: isize,

    bool: bool,
    u8: u8,
    u16: u16,
    u32: u32,
    f32: f32,
    f64: f64,
    u128: u128,
    usize: usize,
    usize_boxed: Box<usize>,
    nested_struct: SubData,
    unnamed_struct: TupleStruct,
    primary_color: Color,
    secondary_color: Color,

    #[eguis(hint = "This is Option<_>")]
    optional: Option<SubData>,

    #[eguis(hint = "This is also Option, but as inner value is simple it is presented inline")]
    optional_string: Option<String>,

    list: Vec<Color>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            app_language: Default::default(),
            hashmap: {
                let mut map = HashMap::default();
                map.insert("Key".to_string(), "Value".to_string());
                map
            },
            skipped_data: 0,
            string: "Hello!".to_string(),
            not_resetable_string: "Hello!".to_string(),
            i8: 42,
            i16: 1555,
            i32: -242522,
            i64: 23425259,
            i128: i128::MAX,
            isize: -14,
            isize_imut: -333,
            limited_isize: 6,
            bool: true,
            u8: 94,
            u16: 14029,
            u32: 3025844,
            f32: std::f32::consts::PI,
            f64: std::f64::consts::PI,
            u128: u128::MAX,
            usize: usize::MAX,
            usize_boxed: Box::new(usize::MAX),
            nested_struct: SubData::default(),
            unnamed_struct: TupleStruct::default(),
            primary_color: Color::default(),
            secondary_color: Color::default(),
            optional: Some(SubData::default()),
            optional_string: Some("<- hover label".to_string()),
            list: vec![
                Color::Red,
                Color::Green,
                Color::Custom(3, 2, 1),
                Color::NamedCustom {
                    red: 23,
                    blue: 100,
                    green: 30,
                    metadata: Metadata {
                        message: "Hello!".to_string(),
                    },
                },
            ],
        }
    }
}

#[derive(EguiStruct)]
pub struct TupleStruct(
    #[eguis(resetable = "struct_default")] u8,
    #[eguis(on_change_struct = "self.2=format!(\"Wololo!: {}\", self.1)")] u32,
    String,
    SubData,
);

impl Default for TupleStruct {
    fn default() -> Self {
        Self(3, 24, "Hello!".to_string(), SubData::default())
    }
}

#[derive(EguiStruct, Default)]
pub struct Metadata {
    message: String,
}

#[derive(EguiStruct, Default)]
#[eguis(resetable = "struct_default")]
pub struct SubData {
    value: String,
    number: u32,
}

#[derive(Default)]
pub struct DemoApp {
    data: Data,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            data.show_top_mut(ui, RichText::new("Data").heading(), None);
        });
    }
}
