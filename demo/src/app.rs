#![allow(dead_code)]
use egui::mutex::RwLock;
use egui::RichText;
use egui_struct::exgrid::GridMode;
use egui_struct::prelude::*;
use indexmap::IndexSet;
use rust_i18n::set_locale;
use std::collections::{HashMap, HashSet};
use ConfigNum::*;

#[derive(EguiStructMut, Default)]
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
#[derive(EguiStructMut, EguiStructImut, Default)]
#[eguis(start_collapsed = "if let Self::NamedCustom{..} = self {true} else {false}")]
pub enum Color {
    #[default]
    Red,

    Green,

    Named(String),

    Named2 {
        name: String,
    },

    #[eguis(resettable(with_expr = ||Color::Custom(255,13,17) ))]
    Custom(
        #[eguis(
        resettable = "not_resettable",
        map_pre = (|field: &mut u8| field.to_string()),
        map_post = (|field: &mut u8, mapped: String| { use std::str::FromStr; if let Ok(new_val)=u8::from_str(mapped.as_str()) {*field=new_val;} })
    )]
        u8,
        u8,
        #[eguis(config = "DragValue(1,111)")] u8,
    ),

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
lazy_static::lazy_static! {
    pub static ref STATIC_COMBOBOX: RwLock<HashSet<String>> =  RwLock::new(
        HashSet::from(["Einar".to_string(), "Olaf".to_string(), "Harald".to_string()])
    );
}
#[derive(EguiStructMut)]
#[eguis(rename_all = "Sentence", resettable = "struct_default")]
pub struct Data {
    #[eguis(skip)]
    skipped_data: u32,

    #[eguis(on_change = Language::set_locale)]
    app_language: Language,

    hashmap: std::collections::HashMap<String, String>,

    #[eguis(resettable(with_expr = "Resettable with expr".to_string()))]
    string: String,

    #[eguis(resettable = "not_resettable")]
    not_resettable_string: String,

    #[eguis(resettable = "field_default")]
    i8: i8,

    #[eguisM(resettable = "field_default")]
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

    #[eguis(config = "SliderStep(5,110,5)")]
    stepped_isize: isize,

    bool: bool,
    u8: u8,
    u16: u16,

    #[eguis(
        map_pre_ref = u32::to_string, //Same as: map_pre_ref = (|field: &u32| field.to_string()),
        map_post = (|field: &mut u32, mapped: String| { use std::str::FromStr; if let Ok(new_val)=u32::from_str(mapped.as_str()) {*field=new_val;} })
    )]
    u32: u32,

    #[eguis(
        resettable = "not_resettable",
        map_pre = RwLock::write, //more elegant would be to use sth like: map_pre_ref = RwLock::read, map_post= (|field, mapped|*field.write()=mapped;),
        eeq = (|field: &RwLock<u32>, rhs: &RwLock<u32>| field.read().eguis_eq(&*rhs.read()) ),
        eclone = (|field: &mut RwLock<u32>, rhs: &RwLock<u32>| field.write().eguis_clone(&*rhs.read()) )
    )]
    u32_rwlock: RwLock<u32>,
    f32: f32,
    f64: f64,
    u128: u128,
    usize: usize,

    #[eguis(
        hint = "fields in derived struct needs to implement EguiStructMut or deref to type that implements it"
    )]
    usize_boxed: Box<usize>,

    #[eguis(config = "Some(&mut [2,3,5,7,11,13,17,19].into_iter())")]
    u8_combobox_wrapper: Combobox<u8>,

    #[eguis(config = "ComboBox(&mut [2,3,5,7,11,13,17,19].into_iter())")]
    u8_combobox_config: u8,

    //this(cloning) is not elegant but for most cases would work well enough
    #[eguis(config = "Some(&mut STATIC_COMBOBOX.read().clone().into_iter())")]
    static_combobox: Combobox<String>,

    nested_struct: SubData,
    unnamed_struct: TupleStruct,
    primary_color: Color,
    secondary_color: Color,

    #[eguis(hint = "This is Option<_>", start_collapsed = true)]
    optional: Option<SubData>,

    #[eguis(hint = "This is also Option, but as inner value is simple it is presented inline")]
    optional_string: Option<String>,

    #[eguis(config = " ConfigSetMut{
            expandable: Some(ConfigSetExpandable{default: &Default::default, mutable:false}),
            shrinkable: true,
            mutable_data: false,
            max_len: Some(5),
            inner_config: Default::default(),
        }")]
    list: Vec<Color>,
    set: IndexSet<i32>,
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
            not_resettable_string: "Hello!".to_string(),
            i8: 42,
            i16: 1555,
            i32: -242522,
            i64: 23425259,
            i128: i128::MAX,
            isize: -14,
            isize_imut: -333,
            limited_isize: 6,
            stepped_isize: 50,
            bool: true,
            u8: 94,
            u16: 14029,
            u32: 3025844,
            u32_rwlock: RwLock::new(9999),
            f32: std::f32::consts::PI,
            f64: std::f64::consts::PI,
            u128: u128::MAX,
            usize: usize::MAX,
            usize_boxed: Box::new(usize::MAX),
            u8_combobox_wrapper: Combobox(3),
            u8_combobox_config: 3,
            static_combobox: Combobox("default name".to_string()),
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
            set: IndexSet::from([2, 4, 8]),
        }
    }
}

#[derive(EguiStructMut)]
pub struct TupleStruct(
    #[eguis(resettable = "struct_default")] u8,
    #[eguis(on_change_struct = (|s: &mut TupleStruct|s.2=format!("Wololo!: {}", s.1)))] u32,
    String,
    SubData,
);

impl Default for TupleStruct {
    fn default() -> Self {
        Self(3, 24, "Hello!".to_string(), SubData::default())
    }
}

#[derive(EguiStructMut, EguiStructImut, Default)]
pub struct Metadata {
    message: String,
}

#[derive(EguiStructMut, Default)]
#[eguis(resettable = "struct_default")]
pub struct SubData {
    value: String,
    number: u32,
}

#[derive(Default)]
pub struct DemoApp {
    data: Data,
    exgrid_mode: GridMode,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data, exgrid_mode } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals.striped = true;
            exgrid_mode.eguis_mut().label("ExGrid Mode").show(ui);
            data.eguis_mut()
                .label(RichText::new("Data").heading())
                .view_mode(exgrid_mode.clone())
                .show(ui);
        });
    }
}
