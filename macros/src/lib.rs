use convert_case::{Case, Converter, Pattern};
use darling::{ast, FromDeriveInput, FromVariant};
use darling::{FromField, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Expr, Index};
use syn::{Ident, Type};

#[derive(Debug, Default, Clone, FromMeta, PartialEq)]
enum Resetable {
    #[default]
    ///Field will be resetable to $r if called with reset2 == Some($r)
    FollowArg,
    ///Marked field will not be resetable
    NotResetable,
    ///Whole struct needs to implement Default
    StructDefault,
    ///all fields that will be resetable need to implement Default
    FieldDefault,
    ///reset button will reset to contained custom value (value of field)
    WithExpr(Expr),
    ///INTERNAL USE ONLY! reset button will reset to value stored by oncelock named by contained ident
    WithStructExpr(Ident),
}
impl Resetable {
    fn mask(&self, mask: &Option<Self>) -> Self {
        if let Some(mask) = mask {
            mask.clone()
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(eguis, eguisM, eguisI))]
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
    /// Use function callback (when value has been changed; signature: fn(&mut field_type) )
    on_change: Option<Expr>,
    /// Use function callback (when value has been changed; signature: fn(&mut self) )
    on_change_struct: Option<Expr>,
    /// pass format/config object to customise how field is displayed
    imconfig: Option<String>,
    /// pass format/config object to customise how field is displayed (when mutable)
    config: Option<String>,
    /// add reset(to default) button (what is called default depends on selected Resetable::*; overrides resetable setting for parrent struct)
    resetable: Option<Resetable>,
    /// Expression (closure surounded by `()` OR function path) called to map field to another type before displaying
    /// - this allows displaying fields that does not implement EguiStruct or overiding how field is shown
    /// - function shall take `& field_type` or `&mut field_type` AND return either mutable reference or owned value of selected type
    /// - ! beware, becouse(if `map_pre_ref` is not set) this will make field work only with resetable values: {NonResetable, WithExpr, FieldDefault}
    /// - defaults to `map_pre_ref` (so if `&mut` is not needed for map, can be left unused)
    map_pre: Option<Expr>,
    /// similar to `map_pre`, but takes immutable reference (signature:` fn(&field_type)->mapped` ),
    /// - used for EguiStructImut, converting default/reset2 and inside eguis_eq (if eeq not specified)
    map_pre_ref: Option<Expr>,
    /// Expression (closure surounded by `()` OR function path) called to map mapped field back to field_type after displaying
    /// - only used if `map_pre` is set AND not for EguiStructImut
    /// - signature: `fn(&mut field_type, &mapped)` (with `mapped` type matching return from `map_pre`)
    /// - expresion should assign new value to `&mut field_type`
    map_post: Option<Expr>,
    /// override `eguis_eq` function for field (signature fn(&field_type, &field_type))
    /// - if either `field_type : EguiStructEq` OR `map_pre_ref` is specified can be unused
    eeq: Option<Expr>,
    /// override `eguis_eclone` function for field (signature fn(&mut field_type, &field_type))
    // / - if `field_type : EguiStructClone` can be unused
    eclone: Option<Expr>,
    /// Override fields `start_collapsed()` output (if set true field will always start collapsed)
    start_collapsed: Option<bool>,
}
#[derive(Debug, FromVariant)]
#[darling(attributes(eguis, eguisM, eguisI))]
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
    ///add reset(to default) button to all inner fields (overrides resetable enum-level setting)
    resetable: Option<Resetable>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(eguis, eguisM, eguisI))]
struct EStruct {
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<EVariant, EField>,

    ///rename all variant names to selected case
    rename_all: Option<String>,
    ///prefix to be added to i18n keys
    prefix: Option<String>,
    ///do not generate EguiStruct impl
    #[darling(default)]
    no_mut: bool,
    ///do not generate EguiStructImut impl
    #[darling(default)]
    no_imut: bool,
    ///do not generate EguiStructClone
    #[darling(default)]
    no_eclone: bool,
    ///do not generate EguiStructEq
    #[darling(default)]
    no_eeq: bool,
    ///add reset(to default) button to all fields (same as marking all fields with same attribute)
    #[darling(default)]
    resetable: Resetable,
    /// Set `start_collapsed()` implementation (if not specified fn return `false`)
    #[darling(default)]
    start_collapsed: Option<Expr>,
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
    let mut reset_to_struct_default = false;
    let mut has_childs_arm = Vec::new();
    let mut has_childs_mut_arm = Vec::new();
    let mut show_childs_arm = Vec::new();
    let mut show_childs_mut_arm = Vec::new();
    let mut show_combobox = Vec::new();
    let mut to_name_arm = Vec::new();
    let mut to_hint_arm = Vec::new();
    let mut show_primitive_arm = Vec::new();
    let mut show_primitive_mut_arm = Vec::new();
    let mut eclone_arm = Vec::new();
    let mut eeq_arm = Vec::new();

    let mut resetable = input.resetable.clone();
    let mut reset_to_struct_expr = Vec::new();
    if let Resetable::WithExpr(expr) = &input.resetable {
        resetable = Resetable::WithStructExpr(format_ident!("STRUCT_DEFAULT_EXPR"));
        reset_to_struct_expr.push(quote! {
            static STRUCT_DEFAULT_EXPR: ::std::sync::OnceLock<#ty> = ::std::sync::OnceLock::new();
            _=STRUCT_DEFAULT_EXPR.get_or_init(#expr);
        })
    };

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
            let key = prefix.clone() + &vident.to_string() + ".__hint";
            quote! { .on_hover_text(::rust_i18n::t!( #[allow(unused_doc_comments)]#[doc = #vhint] #key ))}
        } else {
            quote! { .on_hover_text(#vhint) }
        };
        let hint_top = if hint.is_empty() {
            quote! {()}
        } else {
            quote! { response=response #hint }
        };
        let vlabel = if cfg!(feature = "i18n") {
            let key = if let Some(n) = &variant.i18n {
                n.clone()
            } else {
                prefix.clone() + &vident.to_string()
            };
            quote! { ::rust_i18n::t!(#[allow(unused_doc_comments)]#[doc = #vname_str] #key )}
        } else {
            quote! { #vname_str .to_string() }
        };

        let mut vresetable = resetable.mask(&variant.resetable);
        if let Resetable::WithExpr(expr) = &vresetable.clone() {
            let static_name = format_ident!("VARIANT_{}_DEFAULT_EXPR", vident);
            vresetable = Resetable::WithStructExpr(static_name.clone());
            reset_to_struct_expr.push(quote! {
                #[allow(nonstandard_style)]
                static #static_name: ::std::sync::OnceLock<#ty> = ::std::sync::OnceLock::new();
                _=#static_name.get_or_init(#expr);
            })
        };

        match variant.fields.style {
            ast::Style::Tuple => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                let mut fields_names2 = Vec::new();
                for (idx, field) in variant.fields.fields.iter().enumerate() {
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_type::default(), });
                    fields_names.push(format_ident!("_field_{}", idx));
                    fields_names2.push(format_ident!("_2_field_{}", idx));
                }
                let vident_w_inner = quote! { Self :: #vident(#(#fields_names),*)};
                let vident_w_inner2 = quote! { Self :: #vident(#( #fields_names2),*)};
                let (
                    _reset_to_struct_default,
                    fields_code,
                    mut fields_code_mut,
                    fields_map_eclone,
                    fields_map_eeq,
                    single_field,
                    on_change,
                    fidx,
                ) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "_field_",
                    quote! {},
                    "_2_field_",
                    vresetable,
                    Some(vident_w_inner.clone()),
                );
                reset_to_struct_default |= _reset_to_struct_default;
                if fields_code.len() == 1 {
                    let fident = format_ident!("_field_{}", fidx);
                    let single_field = single_field.unwrap();
                    let fty = single_field.ty;
                    let map_ref = single_field.map_pre_ref.map_or(quote! {}, |x| quote! {#x});
                    let map = single_field.map_pre.map_or(quote! {}, |x| quote! {#x});
                    let map_post = single_field.map_post.map_or(
                        quote! {},
                        |x| quote! {if r.changed() { #x(#fident, mapped); } },
                    );

                    let imconfig = get_config(single_field.imconfig);
                    let config = get_config(single_field.config);
                    has_childs_arm.push(quote! { Self:: #vident(..) => ! #fty::SIMPLE_IMUT,});
                    has_childs_mut_arm.push(quote! { Self:: #vident(..) => ! #fty::SIMPLE,});
                    let primitive_imut = quote! {#vident_w_inner => response |= #map_ref(#fident).show_primitive_imut(ui,#imconfig,id),};
                    let primitive_mut = quote! { #vident_w_inner => {let mut mapped=#map(#fident); let r= mapped.show_primitive(ui,#config,id);  #map_post; {#on_change}; response |=r;},};
                    show_primitive_arm.push(primitive_imut.clone());
                    if variant.imut {
                        show_primitive_mut_arm.push(primitive_imut);
                    } else {
                        show_primitive_mut_arm.push(primitive_mut);
                    }
                } else {
                    let childs_arm = quote! { Self:: #vident(..) => true,};
                    has_childs_arm.push(childs_arm.clone());
                    has_childs_mut_arm.push(childs_arm.clone());
                }
                to_name_arm.push(quote! { #ty :: #vident(..) => #vlabel,});
                to_hint_arm.push(quote! { Self :: #vident(..) => #hint_top,});

                show_childs_arm.push(quote! {#vident_w_inner=>{#(#fields_code)*},});
                if variant.imut {
                    fields_code_mut = fields_code
                }
                show_childs_mut_arm.push(quote! { #vident_w_inner=>{#(#fields_code_mut)*},});
                show_combobox.push(quote! {
                    let mut tresp=ui.selectable_label(matches!(self,  Self:: #vident(..)), #vlabel)#hint;
                    if tresp.clicked()
                    {
                        *self = Self:: #vident(#(#fields_default)*);
                        tresp.mark_changed()
                    }
                    inner_response |=tresp;
                });

                eeq_arm.push(quote! {
                    #vident_w_inner => {
                        if let #vident_w_inner2=rhs{
                            #( ret &= #fields_map_eeq )*
                        } else {ret= false;}
                    },
                });
                eclone_arm.push(quote! {
                    #vident_w_inner=>{
                        if let #vident_w_inner2=self{
                            #( #fields_map_eclone )*
                        } else {
                            *self = Self:: #vident(#(#fields_default)*);
                            if let #vident_w_inner2=self{
                                #( #fields_map_eclone )*
                            } else {::std::unreachable!()}
                        }
                    },
                });
            }
            ast::Style::Struct => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                let mut fields_names2 = Vec::new();
                for field in &variant.fields.fields {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_name: #field_type::default(), });
                    fields_names.push(field_name);
                    let fname2 = format_ident!("_2_{}", field_name);
                    fields_names2.push(quote! { #field_name: #fname2 });
                }
                let vident_w_inner = quote! { Self :: #vident{#(#fields_names),*}};
                let (
                    _reset_to_struct_default,
                    fields_code,
                    mut fields_code_mut,
                    fields_map_eclone,
                    fields_map_eeq,
                    _,
                    _,
                    _,
                ) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "",
                    quote! {},
                    "_2_",
                    vresetable,
                    Some(vident_w_inner.clone()),
                );
                reset_to_struct_default |= _reset_to_struct_default;

                let childs_arm = quote! { Self:: #vident{..} => true,};
                has_childs_arm.push(childs_arm.clone());
                has_childs_mut_arm.push(childs_arm.clone());
                to_name_arm.push(quote! { #ty :: #vident{..} => #vlabel,});
                to_hint_arm.push(quote! { Self :: #vident{..} => #hint_top,});
                show_childs_arm.push(quote! { #vident_w_inner => {#(#fields_code)*},});
                if variant.imut {
                    fields_code_mut = fields_code
                }
                show_childs_mut_arm.push(quote! { #vident_w_inner => {#(#fields_code_mut)*},});
                show_combobox.push(quote! {
                    let mut tresp=ui.selectable_label(matches!(self,  Self:: #vident{..}), #vlabel)#hint;
                    if tresp.clicked()
                    {
                        *self = Self:: #vident{#(#fields_default)*};
                        tresp.mark_changed()
                    }
                    inner_response |=tresp;
                });

                eeq_arm.push(quote! {
                    #vident_w_inner =>{
                        if let Self::#vident{#(#fields_names2),*}=rhs{
                            #( ret &= #fields_map_eeq )*
                        } else {ret= false;}
                    },
                });
                eclone_arm.push(quote! {
                    #vident_w_inner=>{
                        if let Self::#vident{#(#fields_names2),*}=self{
                            #( #fields_map_eclone )*
                        } else {
                            *self = Self:: #vident{#(#fields_default)*};
                            if let Self::#vident{#(#fields_names2),*}=self{
                                #( #fields_map_eclone )*
                            } else {::std::unreachable!()}
                        }
                    },
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
                    inner_response |=tresp;
                });
                eclone_arm.push(quote! {
                    Self::#vident=>{
                        *self=Self::#vident;
                    },
                });
            }
        }
    }
    let reset_to_struct_default = if reset_to_struct_default {
        quote! {
            static STRUCT_DEFAULT: ::std::sync::OnceLock<#ty> = ::std::sync::OnceLock::new();
            _=STRUCT_DEFAULT.get_or_init(#ty::default);
        }
    } else {
        quote! {}
    };

    let start_collapsed = input
        .start_collapsed
        .as_ref()
        .map(|x| quote!(#x))
        .unwrap_or(quote!(false));

    let egui_struct_imut = quote! {
        impl #impl_generics ::egui_struct::EguiStructImut for #ty #ty_generics #where_clause {
            const SIMPLE_IMUT: ::std::primitive::bool = #simple;//is c-like enum
            type ConfigTypeImut<'a> = ();
            fn has_childs_imut(&self) -> ::std::primitive::bool {
                match self{
                    #(#has_childs_arm)* //variant1=>false,
                    _=> false,
                }
            }
            fn has_primitive_imut(&self) -> ::std::primitive::bool {
                true
            }
            fn show_childs_imut(&self, ui: &mut ::egui::Ui, indent_level: ::std::primitive::isize, mut response: ::egui::Response, _reset2: ::std::option::Option<&Self>, id: ::egui::Id) -> ::egui::Response {
                match self{
                    #(#show_childs_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive_imut(&self, ui: &mut ::egui::Ui, _config: Self::ConfigTypeImut<'_>, id: impl ::std::hash::Hash + ::std::clone::Clone) -> ::egui::Response {
                fn to_text(s:& #ty)-> ::std::string::String{
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
            fn start_collapsed_imut(&self) -> bool {
                #start_collapsed
            }
        }
    };

    let egui_struct_mut = quote! {
        impl #impl_generics ::egui_struct::EguiStruct for #ty #ty_generics #where_clause {
            const SIMPLE: ::std::primitive::bool = #simple;//is c-like enum
            type ConfigType<'a> = ();
            fn has_childs(&self) -> ::std::primitive::bool {
                match self{
                    #(#has_childs_mut_arm)* //variant1=>false,
                    _=> false,
                }
            }
            fn has_primitive(&self) -> ::std::primitive::bool {
                true
            }
            fn show_childs(&mut self, ui: &mut ::egui::Ui, indent_level: ::std::primitive::isize, mut response: ::egui::Response, reset2: ::std::option::Option<&Self>, id: ::egui::Id) -> ::egui::Response {
                #![allow(unused)]
                #reset_to_struct_default
                #(#reset_to_struct_expr)*
                match self{
                    #(#show_childs_mut_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive(&mut self, ui: &mut ::egui::Ui, _config: Self::ConfigType<'_>, id: impl ::std::hash::Hash + ::std::clone::Clone) -> ::egui::Response {
                #![allow(unused)]
                fn to_text(s:& #ty)-> ::std::string::String{
                    match s{
                        #(#to_name_arm)*
                        _=>"".to_string()}
                }
                ui.horizontal(|ui|{
                    let defspacing=ui.spacing().item_spacing.clone();
                    ui.spacing_mut().item_spacing=::egui::vec2(0.0, 0.0);
                    let mut inner_response=ui.allocate_response(::egui::vec2(0.0,0.0), ::egui::Sense::hover());
                    let mut response=::egui::ComboBox::from_id_source((id.clone(), "__EguiStruct_enum_combobox")).wrap(false)
                    .selected_text(to_text(self))
                    .show_ui(ui,|ui|{
                        ui.spacing_mut().item_spacing=defspacing;
                        #(#show_combobox)* //ui.selectable_value(&mut selected, Enum::First, "First").on_hover_text("hint");
                    }).response;
                    ui.spacing_mut().item_spacing=defspacing;
                    match self{
                        #(#to_hint_arm)*
                        _=>(),
                    }
                    match self{
                        #(#show_primitive_mut_arm)*
                        _=>(),
                    }
                    response | inner_response
                }).inner
            }
            fn start_collapsed(&self) -> bool {
                #start_collapsed
            }
        }
    };

    let eclone = quote! {
        impl #impl_generics ::egui_struct::EguiStructClone for #ty #ty_generics #where_clause {
            fn eguis_clone(&mut self, source: &Self) {
                match source{
                    #(#eclone_arm)*
                    _=>(),
                }
            }
        }
    };
    let eeq = quote! {
        impl #impl_generics ::egui_struct::EguiStructEq for #ty #ty_generics #where_clause {
            fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
                let mut ret=true;
                match self{
                    #(#eeq_arm)*
                    _=>(),
                }
                ret
            }
        }
    };
    let mut ret = quote! {};
    if !input.no_imut {
        ret = quote! {#ret #egui_struct_imut};
    }
    if !input.no_mut {
        ret = quote! {#ret #egui_struct_mut};
    }
    if !input.no_eclone {
        ret = quote! {#ret #eclone};
    }
    if !input.no_eeq {
        ret = quote! {#ret #eeq};
    }
    ret
}

fn handle_fields(
    fields: &Vec<EField>,
    prefix: String,
    case: &Option<Converter>,
    prefix_code: TokenStream,
    prefix_ident: &str,
    prefix_code2: TokenStream,
    prefix_ident2: &str,
    resetable: Resetable,
    variant: Option<TokenStream>,
) -> (
    bool,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Option<EField>,
    TokenStream,
    Index,
) {
    let mut fields_code = Vec::new();
    let mut fields_code_mut = Vec::new();
    let mut fields_map_eclone = Vec::new();
    let mut fields_map_eeq = Vec::new();
    let mut index = syn::Index::from(0);
    let mut single_field = None;
    let mut reset_to_struct_default = false;
    let mut on_change = quote! {};
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
                let key = if let Some(n) = &field.i18n {
                    n.clone()
                } else {
                    prefix.clone() + &field_name.to_string()
                };
                lab = quote! { ::rust_i18n::t!(#[allow(unused_doc_comments)]#[doc = #label] #key )};
            } else {
                lab = quote! { #label };
            }
        } else {
            index = syn::Index::from(idx);
            name_tt = index.to_token_stream();
            field_name = idx.to_string();
            let label = "[".to_string() + &field_name + "]";
            lab = quote! { #label};
        }
        let hint = &field.hint;
        let hint = if cfg!(feature = "i18n") && !hint.is_empty() {
            let key = prefix.clone() + &name_tt.to_string() + ".__hint.";
            quote! { ::rust_i18n::t!(#[allow(unused_doc_comments)]#[doc = #hint] #key  )}
        } else {
            quote! { #hint }
        };
        let mut whole_ident = quote! { #name_tt};
        let mut whole_ident2 = quote! { #name_tt};
        if !prefix_ident.is_empty() {
            whole_ident = format_ident!("{}{}", prefix_ident, field_name).into_token_stream();
        };
        if !prefix_ident2.is_empty() {
            whole_ident2 = format_ident!("{}{}", prefix_ident2, field_name).into_token_stream();
        };
        whole_ident = quote! {#prefix_code #whole_ident};
        whole_ident2 = quote! {#prefix_code2 #whole_ident2};

        let imconfig = get_config(field.imconfig.clone());
        let config = get_config(field.config.clone());

        on_change = quote! {};
        if let Some(func) = &field.on_change {
            on_change = quote! {
                if response.changed(){
                    #func(&mut #whole_ident);
                }
            };
        }
        if let Some(func) = &field.on_change_struct {
            on_change = quote! {
                #on_change
                if response.changed(){
                    #func(self)
                }
            };
        }

        let mut bresetable = resetable.mask(&field.resetable);
        if bresetable == Resetable::StructDefault {
            reset_to_struct_default = true;
            bresetable = Resetable::WithStructExpr(format_ident!("STRUCT_DEFAULT"))
        };
        let resetable = match &bresetable {
            Resetable::FollowArg => {
                if let Some(variant) = &variant {
                    quote! { reset2.and_then(|f| if let #variant=f{ ::std::option::Option::Some(#whole_ident) }else{ ::std::option::Option::None } ) }
                } else {
                    quote! { reset2.map(|f|&f.#name_tt) }
                }
            }
            Resetable::NotResetable => quote! { ::std::option::Option::None},
            Resetable::StructDefault => unreachable!(),
            Resetable::FieldDefault => {
                quote! { ::std::option::Option::Some(&::std::default::Default::default())}
            }
            Resetable::WithExpr(expr) => quote! { ::std::option::Option::Some(&#expr)},
            Resetable::WithStructExpr(expr) => {
                if let Some(variant) = &variant {
                    quote! {if let #variant=&#expr.get().unwrap(){ ::std::option::Option::Some(#whole_ident) }else{ ::std::option::Option::None }  }
                } else {
                    quote! { ::std::option::Option::Some(&#expr.get().unwrap().#name_tt) }
                }
            }
        };
        let start_collapsed = if let Some(x) = field.start_collapsed {
            quote!(Some(#x))
        } else {
            quote!(None)
        };

        let mut field_code_imut = quote! { response |= #whole_ident.show_collapsing_inner_imut( ui, #lab, #hint, indent_level, #imconfig, ::std::option::Option::None, id, #start_collapsed);};
        let mut field_code_mut = quote! { response |= #whole_ident.show_collapsing_inner( ui, #lab, #hint, indent_level, #config, #resetable, id, #start_collapsed);};
        let (_ref, _ref_mut) = if variant.is_some() {
            (quote! {}, quote! {})
        } else {
            (quote! {&}, quote! {&mut})
        };
        let mut sfield = field.clone();
        let mut map_reset = quote! {};
        if let Some(map_pre_ref) = &field.map_pre_ref {
            let _ = sfield.map_pre.get_or_insert(map_pre_ref.clone());
            field_code_imut = quote! {
                #[allow(unused_mut)]
                let mut mapped = #map_pre_ref(#_ref #whole_ident);
                response |=mapped .show_collapsing_inner_imut( ui, #lab, #hint, indent_level, #imconfig, ::std::option::Option::None, id, #start_collapsed);
            };
            map_reset = quote! {#map_pre_ref};
        }

        if let Some(map_pre) = &sfield.map_pre {
            field_code_mut = quote! {
                #[allow(unused_mut)]
                let mut mapped = #map_pre(#_ref_mut #whole_ident);
                let r = mapped .show_collapsing_inner( ui, #lab, #hint, indent_level, #config, #resetable.map(|x|#map_reset(x)).as_ref(), id, #start_collapsed);
                response |= r.clone();
            };

            if let Some(map_post) = &field.map_post {
                field_code_mut = quote! { #field_code_mut if r.changed() { #map_post(#_ref_mut #whole_ident, mapped);}  };
            }
        }
        field_code_mut = quote! { #field_code_mut {#on_change}; };

        if let Some(expr) = &field.eeq {
            fields_map_eeq.push(quote! {#expr(#_ref #whole_ident,#_ref #whole_ident2);});
        } else {
            if let Some(map_pre_ref) = &field.map_pre_ref {
                fields_map_eeq.push(quote! {#map_pre_ref(#_ref #whole_ident).eguis_eq(&#map_pre_ref(#_ref #whole_ident));});
            } else {
                fields_map_eeq.push(quote! {#whole_ident.eguis_eq(#_ref #whole_ident2);});
            }
        }

        if variant.is_some() {
            (whole_ident, whole_ident2) = (whole_ident2, whole_ident); //in enum eclone self is destructed to whole_ident2
        }
        if let Some(expr) = &field.eclone {
            fields_map_eclone.push(quote! {#expr(#_ref_mut #whole_ident,#_ref #whole_ident2);});
        } else {
            fields_map_eclone.push(quote! {#whole_ident.eguis_clone(#_ref #whole_ident2);});
        }

        fields_code.push(field_code_imut.clone());
        if field.imut {
            fields_code_mut.push(field_code_imut)
        } else {
            fields_code_mut.push(field_code_mut)
        }
        single_field = Some(sfield);
    }
    (
        reset_to_struct_default,
        fields_code,
        fields_code_mut,
        fields_map_eclone,
        fields_map_eeq,
        single_field,
        on_change,
        index,
    )
}

fn handle_struct(
    fields: &ast::Fields<EField>,
    prefix: String,
    case: &Option<Converter>,
    input: &EStruct,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident.clone();
    let mut resetable = input.resetable.clone();
    let reset_to_struct_expr = if let Resetable::WithExpr(expr) = &input.resetable {
        resetable = Resetable::WithStructExpr(format_ident!("STRUCT_DEFAULT_EXPR"));
        quote! {
            static STRUCT_DEFAULT_EXPR: ::std::sync::OnceLock<#name> = ::std::sync::OnceLock::new();
            _=STRUCT_DEFAULT_EXPR.get_or_init(#expr);
        }
    } else {
        quote! {}
    };

    let (
        reset_to_struct_default,
        fields_code,
        fields_code_mut,
        fields_map_eclone,
        fields_map_eeq,
        single_field,
        on_change,
        index,
    ) = handle_fields(
        &fields.fields,
        prefix,
        case,
        quote! {self.},
        "",
        quote! {rhs.},
        "",
        resetable,
        None,
    );

    let reset_to_struct_default = if reset_to_struct_default {
        quote! {
            static STRUCT_DEFAULT: ::std::sync::OnceLock<#name> = ::std::sync::OnceLock::new();
            _=STRUCT_DEFAULT.get_or_init(#name::default);
        }
    } else {
        quote! {}
    };

    let mut show_primitive = quote! { ui.label("") };
    let mut show_primitive_imut = quote! { ui.label("") };
    let (mut simple_imut, mut simple) = (quote! {false}, quote! {false});
    if fields.style == ast::Style::Tuple && fields_code.len() == 1 {
        if let Some(single_field) = &single_field {
            let ty = &single_field.ty;
            simple_imut = quote! { #ty::SIMPLE_IMUT};
            simple = quote! { #ty::SIMPLE };

            let config_imut = get_config(single_field.imconfig.clone());
            let config = get_config(single_field.config.clone());

            let map_ref = single_field
                .map_pre_ref
                .clone()
                .map_or(quote! {}, |x| quote! {#x});
            let map = single_field
                .map_pre
                .clone()
                .map_or(quote! {}, |x| quote! {#x});
            let map_post = single_field.map_post.clone().map_or(
                quote! {},
                |x| quote! { if response.changed() {#x(&mut self.#index, mapped);} },
            );
            show_primitive_imut = quote! {
                  if Self::SIMPLE_IMUT {
                    #map_ref (&self. #index).show_primitive_imut(ui,#config_imut, id)
                  }else {
                    ui.label("")
                  }
            };
            show_primitive = quote! {
                if Self::SIMPLE {
                    let mut mapped=#map (&mut self. #index);
                    let response=mapped.show_primitive(ui, #config, id);
                    #map_post
                    {#on_change};
                    response
                }else {
                  ui.label("")
                }
            };
        }
    }

    let start_collapsed = input
        .start_collapsed
        .as_ref()
        .map(|x| quote!(#x))
        .unwrap_or(quote!(false));

    let egui_struct_imut = quote! {
        impl #impl_generics ::egui_struct::EguiStructImut for #name #ty_generics #where_clause {
            const SIMPLE_IMUT: ::std::primitive::bool = #simple_imut;
            type ConfigTypeImut<'a> = ();
            fn has_childs_imut(&self) -> ::std::primitive::bool {
               !Self::SIMPLE_IMUT
            }
            fn show_childs_imut(&self, ui: &mut ::egui::Ui, indent_level: ::std::primitive::isize, mut response: ::egui::Response, _reset2: ::std::option::Option<&Self>, id: ::egui::Id) -> ::egui::Response {
                #(#fields_code)*
                response
            }
            fn show_primitive_imut(&self, ui: &mut ::egui::Ui, _config: Self::ConfigTypeImut<'_>, id: impl ::std::hash::Hash + ::std::clone::Clone) -> ::egui::Response {
                #show_primitive_imut
            }
            fn start_collapsed_imut(&self) -> bool {
                #start_collapsed
            }
        }
    };
    let egui_struct_mut = quote! {
        impl #impl_generics ::egui_struct::EguiStruct for #name #ty_generics #where_clause {
            const SIMPLE: ::std::primitive::bool = #simple;
            type ConfigType<'a> = ();
            fn has_childs(&self) -> ::std::primitive::bool {
               !Self::SIMPLE
            }
            fn show_childs(&mut self, ui: &mut ::egui::Ui, indent_level: ::std::primitive::isize, mut response: ::egui::Response, reset2: ::std::option::Option<&Self>, id: ::egui::Id) -> ::egui::Response {
                #reset_to_struct_default
                #reset_to_struct_expr
                #(#fields_code_mut)*
                response
            }
            fn show_primitive(&mut self, ui: &mut ::egui::Ui, _config: Self::ConfigType<'_>, id: impl ::std::hash::Hash + ::std::clone::Clone) -> ::egui::Response {
                #show_primitive
            }
            fn start_collapsed(&self) -> bool {
                #start_collapsed
            }
        }
    };

    let eclone = quote! {
        impl #impl_generics ::egui_struct::EguiStructClone for #name #ty_generics #where_clause {
            fn eguis_clone(&mut self, rhs: &Self) {
                #(#fields_map_eclone)*
            }
        }
    };
    let eeq = quote! {
        impl #impl_generics ::egui_struct::EguiStructEq for #name #ty_generics #where_clause {
            fn eguis_eq(&self, rhs: &Self) -> ::std::primitive::bool {
                let mut ret =true;
                #( ret &= #fields_map_eeq )*
                ret
            }
        }
    };
    let mut ret = quote! {};
    if !input.no_imut {
        ret = quote! {#ret #egui_struct_imut};
    }
    if !input.no_mut {
        ret = quote! {#ret #egui_struct_mut};
    }
    if !input.no_eclone {
        ret = quote! {#ret #eclone};
    }
    if !input.no_eeq {
        ret = quote! {#ret #eeq};
    }
    ret
}

fn egui_struct_inner(input: EStruct) -> TokenStream {
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

    match &input.data {
        ast::Data::Enum(variants) => handle_enum(variants, prefix, &case, &input),
        ast::Data::Struct(fields) => handle_struct(fields, prefix, &case, &input),
    }
}

/// Derive `EguiStruct`, `EguiStructClone` & `EguiStructEq` for struct/enum
///
/// ```
/// #[derive(EguiStruct)]
/// #[eguis(rename_all = "Upper")]
/// struct Data{
///     #[eguis(hint = "This is field")]
///     field: usize
/// }
/// ```
///
/// Atributtes `eguis` & `eguisM` are supported on either enum/struct, field or variant level
/// to configure trait implementation and may take following values:
///
/// - enum/struct level:
///   - `rename_all = "str"`- renames all fields/variants to selected case (recognized values: `"Upper"`, `"Lower"`, `"Title"`, `"Toggle"`, `"Camel"`, `"Pascal"`, `"UpperCamel"`, `"Snake"`, `"UpperSnake"`, `"ScreamingSnake"`, `"Kebab"`, `"Cobol"`, `"UpperKebab"`, `"Train"`, `"Flat"`, `"UpperFlat"`, `"Alternating"`, `"Sentence"`)
///   - `prefix = "str"`- add this prefix when generating `rust-i18n` keys
///   - `no_mut` - do not generate `EguiStruct` implementation
///   - `no_eclone` - do not generate `EguiStructClone` implementation
///   - `no_eeq` - do not generate `EguiStructEq` implementation
///   - `start_collapsed = "Expr"` - sets `start_collapsed()` implementation (should return `bool`; can use `self`)
///   - `resetable = "val"` OR `resetable(with_expr = Expr)` - all fields/variants will be resetable according to provieded value (val: `"not_resetable"`, `"field_default"`, `"struct_default"`, `"follow_arg"`(use value passed on runtime through reset2 arg))
/// - variant level:
///   - `rename ="str"`- Name of the field to be displayed on UI labels or variantName in i18n key
///   - `skip` - Don't generate code for the given variant
///   - `hint ="str"` - add on hover hint
///   - `imut` - variant will be shown as immutable
///   - `i18n ="i18n_key"`- normally i18n keys are in format "prefix.enumName.variantName", override this with "i18n_key"
///   - `resetable`- overides enum/struct level resetable
/// - field level
///   - `rename`, `skip`, `hint`, `imut`, `i18n`- see variant level
///   - `resetable`- overides enum/struct & variant level resetable
///   - `on_change = "expr"`- Use function (`expr`: closure surounded by `()` OR function path) callback (when value has been changed; signature: `fn(&mut field_type)`)
///   - `on_change_struct = "expr"`- Similar to `on_change` but takes whole struct: signature: `fn(&mut self)`
///   - `config`- pass format/config object to customise how field is displayed
///   - `start_collapsed = true/false` - field always starts collapsed/uncollapsed (overides fields `start_collapsed()` return)
///   - `map_pre`- Expression (closure surounded by `()` OR function path) called to map field to another type before displaying
///     - this allows displaying fields that does not implement EguiStruct or overiding how field is shown
///     - function shall take `& field_type` or `&mut field_type` AND return either mutable reference or owned value of selected type (that implements `EguiStruct`)
///     - ! beware, because (if `map_pre_ref` is not set) this will make field work only with resetable values: {NonResetable, WithExpr, FieldDefault}
///     - defaults to `map_pre_ref` (so if `&mut` is not needed for map, can be left unused)
///   - `map_pre_ref`- similar to `map_pre`, but takes immutable reference (signature: `fn(&field_type)->mapped`),
///     - used to convert default/reset2 and inside eguis_eq (if eeq not specified)
///   - `map_post`- Expression (closure surounded by `()` OR function path) called to map mapped field back to field_type after displaying
///     - only used if `map_pre` is set
///     - signature: `fn(&mut field_type, &mapped)` (with `mapped` type matching return from `map_pre`)
///     - expresion should assign new value to `&mut field_type`
///   - `eeq`- override `eguis_eq` function for field (signature fn(&field_type, &field_type))
///     - if either `field_type : EguiStructEq` OR `map_pre_ref` is specified can be unused
///   - `eclone`- override `eguis_eclone` function for field (signature fn(&mut field_type, &field_type))
///     - if `field_type : EguiStructClone` can be unused
#[proc_macro_derive(EguiStruct, attributes(eguis, eguisM))]
pub fn egui_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let mut input = EStruct::from_derive_input(&ast).unwrap();
    input.no_imut = true;
    let toks = egui_struct_inner(input);
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Derive `EguiStructImut` for struct/enum
///
/// ```
/// #[derive(EguiStructImut)]
/// #[eguis(rename_all = "Upper")]
/// struct Data{
///     #[eguis(hint = "This is field")]
///     field: usize
/// }
/// ```
///
/// Atributtes `eguis` & `eguisI` are supported on either enum/struct, field or variant level
/// to configure trait implementation and may take following values:
///
/// - enum/struct level:
///   - `rename_all = "str"`- renames all fields/variants to selected case (recognized values: `"Upper"`, `"Lower"`, `"Title"`, `"Toggle"`, `"Camel"`, `"Pascal"`, `"UpperCamel"`, `"Snake"`, `"UpperSnake"`, `"ScreamingSnake"`, `"Kebab"`, `"Cobol"`, `"UpperKebab"`, `"Train"`, `"Flat"`, `"UpperFlat"`, `"Alternating"`, `"Sentence"`)
///   - `prefix = "str"`- add this prefix when generating `rust-i18n` keys
///   - `start_collapsed = "Expr"` - sets `EguiStructImut::start_collapsed_imut()` implementation (should return `bool`; can use `self`)
/// - variant level:
///   - `rename ="str"`- Name of the field to be displayed on UI labels or variantName in i18n key
///   - `skip` - Don't generate code for the given variant
///   - `hint ="str"` - add on hover hint
///   - `i18n ="i18n_key"`- normally i18n keys are in format "prefix.enumName.variantName", override this with "i18n_key"
/// - field level
///   - `rename`, `skip`, `hint`, `i18n`- see variant level
///   - `imconfig`- pass format/config object([`EguiStructImut::ConfigTypeImut`) to customise how field is displayed
///   - `start_collapsed = true/false` - field always starts collapsed/uncollapsed (overides fields `EguiStructImut::start_collapsed_imut()` return)
///   - `map_pre_ref`- Expression (closure surounded by `()` OR function path) called to map field to another type before displaying
///     - this allows displaying fields that does not implement `EguiStructImut` or overiding how field is shown
///     - function shall take `&field_type` AND return either reference or owned value of selected type (that implements `EguiStructImut`)
#[proc_macro_derive(EguiStructImut, attributes(eguis, eguisI))]
pub fn egui_struct_imut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let mut input = EStruct::from_derive_input(&ast).unwrap();
    input.no_eclone = true;
    input.no_eeq = true;
    input.no_mut = true;
    let toks = egui_struct_inner(input);
    debug_print_generated(&ast, &toks);
    toks.into()
}

////////////////////////////////////////

fn debug_print_generated(ast: &DeriveInput, toks: &TokenStream) {
    let debug = std::env::var("EGUI_STRUCT_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if ast.ident == s {
            println!("{}", toks);
        }
    }
}
fn get_config(config: Option<String>) -> TokenStream {
    config
        .unwrap_or("::std::default::Default::default()".to_string())
        .parse()
        .unwrap()
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
