use convert_case::{Case, Converter, Pattern};
use darling::{ast, FromDeriveInput, FromVariant};
use darling::{FromField, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Index};
use syn::{Ident, Type};

#[derive(Debug, FromField)]
#[darling(attributes(eguis))]
struct EField {
    ident: Option<Ident>,
    ty: Type,

    /// Name of the field to be displayed on UI labels
    rename: Option<String>,
    /// Doesn't generate code for the given field
    #[darling(default)]
    skip: bool,
    ///hint to be displayed on hover
    #[darling(default)]
    hint: String,
    ///field is always imutable
    #[darling(default)]
    imut: bool,
    /// Override i18n key (key will not contain prefix)
    i18n: Option<String>,
    /// Use function callback (when value has been changed; signature: fn(&field_type) )
    on_change: Option<String>,
}
#[derive(Debug, FromVariant)]
#[darling(attributes(eguis))]
struct EVariant {
    ident: Ident,
    fields: ast::Fields<EField>,

    /// Name of the field to be displayed on UI labels
    rename: Option<String>,
    /// Don't generate code for the given variant
    #[darling(default)]
    skip: bool,
    ///hint to be displayed on hover
    #[darling(default)]
    hint: String,
    ///variant is always imutable
    #[darling(default)]
    imut: bool,
    /// Override i18n key (key will not contain prefix)
    i18n: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(eguis))]
struct EStruct {
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<EVariant, EField>,

    ///rename all variant names to selected case
    rename_all: Option<String>,
    ///prefix to be added to i18n keys
    prefix: Option<String>,
    ///generate only EguiStructImut (and not EguiStruct)
    #[darling(default)]
    imut: bool,
}

fn handle_enum(
    variants: &Vec<EVariant>,
    prefix: String,
    case: &Option<Converter>,
    input: &EStruct,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ty = input.ident.clone();
    let mut simple: bool = true;
    let mut has_childs_arm = Vec::new();
    let mut show_childs_arm = Vec::new();
    let mut show_childs_mut_arm = Vec::new();
    let mut show_combobox = Vec::new();
    let mut to_name_arm = Vec::new();
    let mut to_hint_arm = Vec::new();
    let mut show_primitive_arm = Vec::new();
    let mut show_primitive_mut_arm = Vec::new();
    for variant in variants {
        if variant.skip {
            continue;
        }
        let vident = &variant.ident;
        let mut vname_str = vident.to_string();
        if let Some(rename) = &variant.rename {
            vname_str = rename.clone();
        } else if let Some(ref case) = case {
            vname_str = case.convert(vname_str);
        }
        let vhint = &variant.hint;
        let hint = if vhint.is_empty() {
            quote! {}
        } else if cfg!(feature = "i18n") {
            let label = prefix.clone() + &vname_str + ".hint." + &vhint;
            quote! { .on_hover_text(::rust_i18n::t!(#label ))}
        } else {
            quote! { .on_hover_text(#vhint) }
        };
        let hint_top = if hint.is_empty() {
            quote! {()}
        } else {
            quote! { response=response #hint }
        };
        let vlabel = if cfg!(feature = "i18n") {
            let la = if let Some(n) = &variant.i18n {
                n.clone()
            } else {
                prefix.clone() + &vname_str
            };
            quote! { ::rust_i18n::t!(#la )}
        } else {
            quote! { #vname_str .to_string() }
        };

        match variant.fields.style {
            ast::Style::Tuple => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                for (idx, field) in variant.fields.fields.iter().enumerate() {
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_type::default(), });
                    fields_names.push(format_ident!("_field_{}", idx));
                }
                let (fields_code, mut fields_code_mut, fty, fidx) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "_field_",
                );
                if fields_code.len() == 1 {
                    let fident = format_ident!("_field_{}", fidx);
                    has_childs_arm.push(quote! { Self:: #vident(..) => ! #fty::SIMPLE,});
                    show_primitive_arm.push(quote! { Self :: #vident(#(#fields_names),*) => response |= #fident.show_primitive(ui),});
                    if variant.imut {
                        show_primitive_mut_arm.push(quote! { Self :: #vident(#(#fields_names),*) => response |= #fident.show_primitive(ui),});
                    } else {
                        show_primitive_mut_arm.push(quote! { Self :: #vident(#(#fields_names),*) => response |= #fident.show_primitive_mut(ui),});
                    }
                } else {
                    has_childs_arm.push(quote! { Self:: #vident(..) => true,});
                }
                to_name_arm.push(quote! { #ty :: #vident(..) => #vlabel,});
                to_hint_arm.push(quote! { Self :: #vident(..) => #hint_top,});

                show_childs_arm
                    .push(quote! { Self:: #vident(#(#fields_names),*)=>{#(#fields_code)*},});
                if variant.imut {
                    fields_code_mut = fields_code
                }
                show_childs_mut_arm
                    .push(quote! { Self:: #vident(#(#fields_names),*)=>{#(#fields_code_mut)*},});
                show_combobox.push(quote! {
                    let mut tresp=ui.selectable_label(matches!(self,  Self:: #vident(..)), #vlabel)#hint;
                    if tresp.clicked()
                    {
                        *self = Self:: #vident(#(#fields_default)*);
                        tresp.mark_changed()
                    }
                    response |=tresp;
                });
            }
            ast::Style::Struct => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                for field in &variant.fields.fields {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_name: #field_type::default(), });
                    fields_names.push(field_name);
                }
                let (fields_code, mut fields_code_mut, _, _) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "",
                );

                has_childs_arm.push(quote! { Self:: #vident{..} => true,});
                to_name_arm.push(quote! { #ty :: #vident{..} => #vlabel,});
                to_hint_arm.push(quote! { Self :: #vident{..} => #hint_top,});
                show_childs_arm
                    .push(quote! { Self :: #vident{#(#fields_names),*} => {#(#fields_code)*},});
                if variant.imut {
                    fields_code_mut = fields_code
                }
                show_childs_mut_arm
                    .push(quote! { Self :: #vident{#(#fields_names),*} => {#(#fields_code_mut)*},});
                show_combobox.push(quote! {
                    let mut tresp=ui.selectable_label(matches!(self,  Self:: #vident{..}), #vlabel)#hint;
                    if tresp.clicked()
                    {
                        *self = Self:: #vident{#(#fields_default)*};
                        tresp.mark_changed()
                    }
                    response |=tresp;
                });
            }
            ast::Style::Unit => {
                to_name_arm.push(quote! { #ty :: #vident => #vlabel,});
                to_hint_arm.push(quote! { Self :: #vident => #hint_top,});
                show_combobox.push(quote! {
                    let mut tresp=ui.selectable_label(matches!(self,  Self:: #vident), #vlabel)#hint;
                    if tresp.clicked()
                    {
                        *self = Self:: #vident;
                        tresp.mark_changed()
                    }
                    response |=tresp;
                });
            }
        }
    }

    let egui_struct_imut = quote! {
        impl #impl_generics EguiStructImut for #ty #ty_generics #where_clause {
            const SIMPLE: bool = #simple;//is c-like enum
            fn has_childs(&self) -> bool {
                match self{
                    #(#has_childs_arm)* //variant1=>false,
                    _=> false,
                }
            }
            fn has_primitive(&self) -> bool {
                true
            }
            fn show_childs(&self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response) -> ::egui::Response {
                match self{
                    #(#show_childs_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive(&self, ui: &mut ::egui::Ui) -> ::egui::Response {
                fn to_text(s:& #ty)-> String{
                    match s{
                        #(#to_name_arm)*
                        _=>"".to_string()}
                }
                ui.horizontal(|ui|{
                    let mut response =ui.label(to_text(self));
                    match self{
                        #(#to_hint_arm)*
                        _=>(),
                    }
                    match self{
                        #(#show_primitive_arm)*
                        _=>(),
                    }
                    response
                }).inner
            }
        }
    };

    let egui_struct_mut = quote! {
        impl #impl_generics EguiStruct for #ty #ty_generics #where_clause {
            fn show_childs_mut(&mut self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response) -> ::egui::Response {
                match self{
                    #(#show_childs_mut_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive_mut(&mut self, ui: &mut ::egui::Ui) -> ::egui::Response {
                fn to_text(s:& #ty)-> String{
                    match s{
                        #(#to_name_arm)*
                        _=>"".to_string()}
                }
                ui.horizontal(|ui|{
                    let mut response=ui.allocate_response(egui::vec2(0.0,0.0), egui::Sense::hover());
                    ::egui::ComboBox::from_id_source(ui.next_auto_id()).wrap(false)
                    .selected_text(to_text(self))
                    .show_ui(ui,|ui|{
                        #(#show_combobox)* //ui.selectable_value(&mut selected, Enum::First, "First").on_hover_text("hint");
                    });
                    match self{
                        #(#to_hint_arm)*
                        _=>(),
                    }
                    match self{
                        #(#show_primitive_mut_arm)*
                        _=>(),
                    }
                    response
                }).inner
            }
        }
    };
    if input.imut {
        egui_struct_imut
    } else {
        quote! {#egui_struct_imut #egui_struct_mut}
    }
}

fn handle_fields(
    fields: &Vec<EField>,
    prefix: String,
    case: &Option<Converter>,
    prefix_code: TokenStream,
    prefix_ident: &str,
) -> (Vec<TokenStream>, Vec<TokenStream>, Option<Type>, Index) {
    let mut fields_code = Vec::new();
    let mut fields_code_mut = Vec::new();
    let mut index = syn::Index::from(0);
    let mut ty = None;
    for (idx, field) in fields.iter().enumerate() {
        if field.skip {
            continue;
        }
        let lab;
        let field_name;
        let name_tt;
        if let Some(field_ident) = &field.ident {
            field_name = field_ident.to_string();
            name_tt = field_ident.to_token_stream();
            let label;
            if let Some(rename) = &field.rename {
                label = rename.clone();
            } else if let Some(ref case) = case {
                label = case.convert(field_name.clone());
            } else {
                label = field_name.clone();
            }

            if cfg!(feature = "i18n") {
                let label = if let Some(n) = &field.i18n {
                    n.clone()
                } else {
                    prefix.clone() + &label
                };
                lab = quote! { ::rust_i18n::t!(#label )};
            } else {
                lab = quote! { #label };
            }
        } else {
            ty = Some(field.ty.clone());
            index = syn::Index::from(idx);
            name_tt = index.to_token_stream();
            field_name = idx.to_string();
            let label = "[".to_string() + &field_name + "]";
            lab = quote! { #label};
        }
        let hint = &field.hint;
        let hint = if cfg!(feature = "i18n") && !hint.is_empty() {
            let label = prefix.clone() + &field_name + ".hint." + &hint;
            quote! { ::rust_i18n::t!(#label )}
        } else {
            quote! { #hint }
        };
        let mut whole_ident = quote! { #name_tt};
        if !prefix_ident.is_empty() {
            whole_ident = format_ident!("{}{}", prefix_ident, field_name).into_token_stream();
        };
        fields_code.push(
            quote! { response |= #prefix_code #whole_ident .show_collapsing( ui, #lab, #hint, indent_level); },
        );
        let mut on_change = quote! {};
        if let Some(custom_func) = &field.on_change {
            let ident = syn::Path::from_string(custom_func)
                .expect(format!("Could not find function: {}", custom_func).as_str());
            on_change = quote! {
                if response.changed(){
                    #ident(& #prefix_code #whole_ident);
                }
            };
        }
        if field.imut {
            fields_code_mut.push( quote! { response |= #prefix_code #whole_ident .show_collapsing( ui, #lab, #hint, indent_level); #on_change } );
        } else {
            fields_code_mut.push( quote! { response |= #prefix_code #whole_ident .show_collapsing_mut( ui, #lab, #hint, indent_level); #on_change} );
        }
    }
    (fields_code, fields_code_mut, ty, index)
}

fn handle_struct(
    fields: &ast::Fields<EField>,
    prefix: String,
    case: &Option<Converter>,
    input: &EStruct,
) -> TokenStream {
    let (fields_code, fields_code_mut, ty, index) =
        handle_fields(&fields.fields, prefix, case, quote! { self.}, "");
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident.clone();
    let simple = fields.style == ast::Style::Tuple && fields_code.len() == 1;
    let simple = if let Some(ty) = ty {
        quote! {#ty :: SIMPLE && #simple}
    } else {
        quote! { #simple}
    };
    macro_rules! show_primitive {
        ($name:ident) => {
            let $name = if fields.style == ast::Style::Tuple {
                quote! {
                    if Self::SIMPLE {
                        self. #index .$name(ui)
                    } else {
                        ui.label("")
                    }
                }
            } else {
                quote! {ui.label("")}
            };
        };
    }
    show_primitive!(show_primitive);
    show_primitive!(show_primitive_mut);

    let egui_struct_imut = quote! {
        impl #impl_generics EguiStructImut for #name #ty_generics #where_clause {
            const SIMPLE: bool = #simple;
            fn has_childs(&self) -> bool {
               !Self::SIMPLE
            }
            fn has_primitive(&self) -> bool {
                !self.has_childs()
            }
            fn show_childs(&self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response) -> ::egui::Response {
                #(#fields_code)*
                response
            }
            fn show_primitive(&self, ui: &mut ::egui::Ui) -> ::egui::Response {
                #show_primitive
            }
        }
    };
    let egui_struct_mut = quote! {
        impl #impl_generics EguiStruct for #name #ty_generics #where_clause {
            fn show_childs_mut(&mut self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response) -> ::egui::Response {
                #(#fields_code_mut)*
                response
            }
            fn show_primitive_mut(&mut self, ui: &mut ::egui::Ui) -> ::egui::Response {
                #show_primitive_mut
            }
        }
    };
    if input.imut {
        egui_struct_imut
    } else {
        quote! {#egui_struct_imut #egui_struct_mut}
    }
}
#[proc_macro_derive(EguiStruct, attributes(eguis))]
pub fn display_gui(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let input = EStruct::from_derive_input(&ast).unwrap();
    let mut prefix = String::new();
    let case = input.rename_all.as_ref().map(|x| parse_case_name(&x));

    if cfg!(feature = "i18n") {
        if let Some(p) = &input.prefix {
            prefix += &p;
            prefix += "."
        }
        prefix += &input.ident.to_string();
        prefix += ".";
    }

    let toks = match &input.data {
        ast::Data::Enum(variants) => handle_enum(variants, prefix, &case, &input),
        ast::Data::Struct(fields) => handle_struct(fields, prefix, &case, &input),
    };

    // let toks = display_gui_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

////////////////////////////////////////

fn debug_print_generated(ast: &DeriveInput, toks: &TokenStream) {
    let debug = std::env::var("STRUM_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if ast.ident == s {
            println!("{}", toks);
        }
    }
}

fn parse_case_name(case_name: &str) -> Converter {
    let conv = Converter::new();
    match case_name {
        "Upper" => conv.to_case(Case::Upper),
        "Lower" => conv.to_case(Case::Lower),
        "Title" => conv.to_case(Case::Title),
        "Toggle" => conv.to_case(Case::Toggle),
        "Camel" => conv.to_case(Case::Camel),
        "Pascal" => conv.to_case(Case::Pascal),
        "UpperCamel" => conv.to_case(Case::UpperCamel),
        "Snake" => conv.to_case(Case::Snake),
        "UpperSnake" => conv.to_case(Case::UpperSnake),
        "ScreamingSnake" => conv.to_case(Case::ScreamingSnake),
        "Kebab" => conv.to_case(Case::Kebab),
        "Cobol" => conv.to_case(Case::Cobol),
        "UpperKebab" => conv.to_case(Case::UpperKebab),
        "Train" => conv.to_case(Case::Train),
        "Flat" => conv.to_case(Case::Flat),
        "UpperFlat" => conv.to_case(Case::UpperFlat),
        "Alternating" => conv.to_case(Case::Alternating),
        "Sentence" => conv.set_pattern(Pattern::Sentence).set_delim(" "),
        _ => panic!("Unrecognized case name: {}", case_name),
    }
}
