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
    /// Use function callback (when value has been changed; signature: fn(&mut field_type) )
    on_change: Option<String>,
    /// When field value has been changed, call this expr
    on_change_struct: Option<String>,
    /// pass format/config object to customise how field is displayed
    imconfig: Option<String>,
    /// pass format/config object to customise how field is displayed (when mutable)
    config: Option<String>,
    ///add reset(to default) button (what is called default depends on selected Resetable::*; overrides resetable setting for parrent struct)
    resetable: Option<Resetable>,
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
    ///add reset(to default) button to all inner fields (overrides resetable enum-level setting)
    resetable: Option<Resetable>,
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
            static STRUCT_DEFAULT_EXPR: std::sync::OnceLock<#ty> = std::sync::OnceLock::new();
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
                static #static_name: std::sync::OnceLock<#ty> = std::sync::OnceLock::new();
                _=#static_name.get_or_init(#expr);
            })
        };

        match variant.fields.style {
            ast::Style::Tuple => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                let mut fields_names2 = Vec::new();
                let mut fields_names_nskipped2 = Vec::new();
                for (idx, field) in variant.fields.fields.iter().enumerate() {
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_type::default(), });
                    fields_names.push(format_ident!("_field_{}", idx));
                    fields_names2.push(format_ident!("_field_{}_2", idx));
                    if !field.skip {
                        fields_names_nskipped2.push(format_ident!("_field_{}_2", idx));
                    }
                }
                let vident_w_inner = quote! { Self :: #vident(#(#fields_names),*)};
                let (
                    _reset_to_struct_default,
                    fields_code,
                    mut fields_code_mut,
                    fields_names_nskipped,
                    single_field,
                    fidx,
                ) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "_field_",
                    vresetable,
                    Some(vident_w_inner.clone()),
                );
                reset_to_struct_default |= _reset_to_struct_default;
                if fields_code.len() == 1 {
                    let fident = format_ident!("_field_{}", fidx);
                    let single_field = single_field.unwrap();
                    let fty = single_field.ty;
                    let imconfig = get_config(single_field.imconfig);
                    let config = get_config(single_field.config);
                    has_childs_arm.push(quote! { Self:: #vident(..) => ! #fty::SIMPLE,});
                    let primitive_imut = quote! {#vident_w_inner => response |= #fident.show_primitive(ui,#imconfig),};
                    let primitive_mut = quote! { #vident_w_inner => response |= #fident.show_primitive_mut(ui,#config),};
                    show_primitive_arm.push(primitive_imut.clone());
                    if variant.imut {
                        show_primitive_mut_arm.push(primitive_imut);
                    } else {
                        show_primitive_mut_arm.push(primitive_mut);
                    }
                } else {
                    has_childs_arm.push(quote! { Self:: #vident(..) => true,});
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
                        if let Self::#vident(#(#fields_names2),*)=rhs{
                            #( ret &= #fields_names_nskipped.eguis_eq( #fields_names_nskipped2); )*
                        } else {ret= false;}
                    },
                });
                eclone_arm.push(quote! {
                    #vident_w_inner=>{
                        if let Self::#vident(#(#fields_names2),*)=self{
                            #( #fields_names_nskipped2.eguis_clone(#fields_names_nskipped); )*
                        } else {
                            *self = Self:: #vident(#(#fields_default)*);
                            if let Self::#vident(#(#fields_names2),*)=self{
                                #( #fields_names_nskipped2.eguis_clone( #fields_names_nskipped); )*
                            } else {unreachable!()}
                        }
                    },
                });
            }
            ast::Style::Struct => {
                simple = false;
                let mut fields_default = Vec::new();
                let mut fields_names = Vec::new();
                let mut fields_names2 = Vec::new();
                let mut fields_names_nskipped2 = Vec::new();
                for field in &variant.fields.fields {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    fields_default.push(quote! { #field_name: #field_type::default(), });
                    fields_names.push(field_name);
                    let fname2 = format_ident!("{}_2", field_name);
                    fields_names2.push(quote! { #field_name: #fname2 });
                    if !field.skip {
                        fields_names_nskipped2.push(format_ident!("{}_2", field_name));
                    }
                }
                let vident_w_inner = quote! { Self :: #vident{#(#fields_names),*}};
                let (
                    _reset_to_struct_default,
                    fields_code,
                    mut fields_code_mut,
                    fields_names_nskipped,
                    _,
                    _,
                ) = handle_fields(
                    &variant.fields.fields,
                    prefix.clone() + &vident.to_string() + ".",
                    case,
                    quote! {},
                    "",
                    vresetable,
                    Some(vident_w_inner.clone()),
                );
                reset_to_struct_default |= _reset_to_struct_default;

                has_childs_arm.push(quote! { Self:: #vident{..} => true,});
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
                            #( ret &= #fields_names_nskipped.eguis_eq( #fields_names_nskipped2); )*
                        } else {ret= false;}
                    },
                });
                eclone_arm.push(quote! {
                    #vident_w_inner=>{
                        if let Self::#vident{#(#fields_names2),*}=self{
                            #( #fields_names_nskipped2.eguis_clone( #fields_names_nskipped); )*
                        } else {
                            *self = Self:: #vident{#(#fields_default)*};
                            if let Self::#vident{#(#fields_names2),*}=self{
                                #( #fields_names_nskipped2.eguis_clone( #fields_names_nskipped); )*
                            } else {unreachable!()}
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
            static STRUCT_DEFAULT: std::sync::OnceLock<#ty> = std::sync::OnceLock::new();
            _=STRUCT_DEFAULT.get_or_init(#ty::default);
        }
    } else {
        quote! {}
    };

    let egui_struct_imut = quote! {
        impl #impl_generics EguiStructImut for #ty #ty_generics #where_clause {
            const SIMPLE: bool = #simple;//is c-like enum
            type ConfigTypeImut = ();
            fn has_childs(&self) -> bool {
                match self{
                    #(#has_childs_arm)* //variant1=>false,
                    _=> false,
                }
            }
            fn has_primitive(&self) -> bool {
                true
            }
            fn show_childs(&self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response, _reset2: Option<&Self>) -> ::egui::Response {
                match self{
                    #(#show_childs_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive(&self, ui: &mut ::egui::Ui, _config: Self::ConfigTypeImut) -> ::egui::Response {
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
            type ConfigType = ();
            fn show_childs_mut(&mut self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response, reset2: Option<&Self>) -> ::egui::Response {
                #![allow(unused)]
                #reset_to_struct_default
                #(#reset_to_struct_expr)*
                match self{
                    #(#show_childs_mut_arm)*
                    _=>(),
                }
                response
            }
            fn show_primitive_mut(&mut self, ui: &mut ::egui::Ui, _config: Self::ConfigType) -> ::egui::Response {
                #![allow(unused)]
                fn to_text(s:& #ty)-> String{
                    match s{
                        #(#to_name_arm)*
                        _=>"".to_string()}
                }
                ui.horizontal(|ui|{
                    let defspacing=ui.spacing().item_spacing.clone();
                    ui.spacing_mut().item_spacing=egui::vec2(0.0, 0.0);
                    let mut inner_response=ui.allocate_response(egui::vec2(0.0,0.0), egui::Sense::hover());
                    let mut response=::egui::ComboBox::from_id_source(ui.next_auto_id()).wrap(false)
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
        }
    };

    let eclone = quote! {
        impl #impl_generics EguiStructClone for #ty #ty_generics #where_clause {
            fn eguis_clone(&mut self, source: &Self) {
                match source{
                    #(#eclone_arm)*
                    _=>(),
                }
            }
        }
    };
    let eeq = quote! {
        impl #impl_generics EguiStructEq for #ty #ty_generics #where_clause {
            fn eguis_eq(&self, rhs: &Self) -> bool {
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
    resetable: Resetable,
    variant: Option<TokenStream>,
) -> (
    bool,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Option<EField>,
    Index,
) {
    let mut fields_code = Vec::new();
    let mut fields_code_mut = Vec::new();
    let mut fields_names_nskipped = Vec::new();
    let mut index = syn::Index::from(0);
    let mut single_field = None;
    let mut reset_to_struct_default = false;
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
            single_field = Some(field.clone());
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
        if !prefix_ident.is_empty() {
            whole_ident = format_ident!("{}{}", prefix_ident, field_name).into_token_stream();
        };

        let imconfig = get_config(field.imconfig.clone());
        let config = get_config(field.config.clone());

        let mut on_change = quote! {};
        if let Some(custom_func) = &field.on_change {
            let ident = syn::Path::from_string(custom_func)
                .expect(format!("Could not find function: {}", custom_func).as_str());
            on_change = quote! {
                if response.changed(){
                    #ident(&mut #prefix_code #whole_ident);
                }
            };
        }
        let mut on_change_struct = quote! {};
        if let Some(custom_expr) = &field.on_change_struct {
            let expr: TokenStream = custom_expr
                .parse()
                .expect(format!("Could parse expr from: {}", custom_expr).as_str());
            on_change_struct = quote! {
                if response.changed(){
                    #expr
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
                    quote! { reset2.and_then(|f| if let #variant=f{ Some(#whole_ident) }else{ None } ) }
                } else {
                    quote! { reset2.map(|f|&f.#name_tt) }
                }
            }
            Resetable::NotResetable => quote! { None},
            Resetable::StructDefault => unreachable!(),
            Resetable::FieldDefault => quote! { Some(&Default::default())},
            Resetable::WithExpr(expr) => quote! { Some(&#expr)},
            Resetable::WithStructExpr(expr) => {
                if let Some(variant) = &variant {
                    quote! {if let #variant=&#expr.get().unwrap(){ Some(#whole_ident) }else{ None }  }
                } else {
                    quote! { Some(&#expr.get().unwrap().#name_tt) }
                }
            }
        };
        fields_names_nskipped.push(quote! { #whole_ident});

        let field_code_imut = quote! { response |= #prefix_code #whole_ident .show_collapsing( ui, #lab, #hint, indent_level, #imconfig, None); };
        let field_code_mut = quote! { response |= #prefix_code #whole_ident .show_collapsing_mut( ui, #lab, #hint, indent_level, #config, #resetable); #on_change #on_change_struct};
        fields_code.push(field_code_imut.clone());
        if field.imut {
            fields_code_mut.push(field_code_imut)
        } else {
            fields_code_mut.push(field_code_mut)
        }
    }
    (
        reset_to_struct_default,
        fields_code,
        fields_code_mut,
        fields_names_nskipped,
        single_field,
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
            static STRUCT_DEFAULT_EXPR: std::sync::OnceLock<#name> = std::sync::OnceLock::new();
            _=STRUCT_DEFAULT_EXPR.get_or_init(#expr);
        }
    } else {
        quote! {}
    };

    let (
        reset_to_struct_default,
        fields_code,
        fields_code_mut,
        fields_names_nskipped,
        single_field,
        index,
    ) = handle_fields(
        &fields.fields,
        prefix,
        case,
        quote! { self.},
        "",
        resetable,
        None,
    );

    let reset_to_struct_default = if reset_to_struct_default {
        quote! {
            static STRUCT_DEFAULT: std::sync::OnceLock<#name> = std::sync::OnceLock::new();
            _=STRUCT_DEFAULT.get_or_init(#name::default);
        }
    } else {
        quote! {}
    };

    let simple = fields.style == ast::Style::Tuple && fields_code.len() == 1;
    let simple = if let Some(sf) = &single_field {
        let ty = &sf.ty;
        quote! {#ty :: SIMPLE && #simple}
    } else {
        quote! { #simple}
    };

    macro_rules! show_primitive {
        ($name:ident, $config:ident) => {
            let config = get_config(single_field.as_ref().and_then(|x| x.$config.clone()));
            let $name = if fields.style == ast::Style::Tuple {
                quote! {
                    if Self::SIMPLE {
                        self. #index .$name(ui,#config)
                    } else {
                        ui.label("")
                    }
                }
            } else {
                quote! {ui.label("")}
            };
        };
    }
    show_primitive!(show_primitive, imconfig);
    show_primitive!(show_primitive_mut, config);

    let egui_struct_imut = quote! {
        impl #impl_generics EguiStructImut for #name #ty_generics #where_clause {
            const SIMPLE: bool = #simple;
            type ConfigTypeImut = ();
            fn has_childs(&self) -> bool {
               !Self::SIMPLE
            }
            fn has_primitive(&self) -> bool {
                !self.has_childs()
            }
            fn show_childs(&self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response, _reset2: Option<&Self>) -> ::egui::Response {
                #(#fields_code)*
                response
            }
            fn show_primitive(&self, ui: &mut ::egui::Ui, _config: Self::ConfigTypeImut) -> ::egui::Response {
                #show_primitive
            }
        }
    };
    let egui_struct_mut = quote! {
        impl #impl_generics EguiStruct for #name #ty_generics #where_clause {
            type ConfigType = ();
            fn show_childs_mut(&mut self, ui: &mut ::egui::Ui, indent_level: isize, mut response: ::egui::Response, reset2: Option<&Self>) -> ::egui::Response {
                #reset_to_struct_default
                #reset_to_struct_expr
                #(#fields_code_mut)*
                response
            }
            fn show_primitive_mut(&mut self, ui: &mut ::egui::Ui, _config: Self::ConfigType) -> ::egui::Response {
                #show_primitive_mut
            }
        }
    };

    let eclone = quote! {
        impl #impl_generics EguiStructClone for #name #ty_generics #where_clause {
            fn eguis_clone(&mut self, source: &Self) {
                #(self.#fields_names_nskipped.eguis_clone(&source.#fields_names_nskipped);)*
            }
        }
    };
    let eeq = quote! {
        impl #impl_generics EguiStructEq for #name #ty_generics #where_clause {
            fn eguis_eq(&self, rhs: &Self) -> bool {
                let mut ret =true;
                #( ret &= self.#fields_names_nskipped.eguis_eq(&rhs.#fields_names_nskipped); )*
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
        .unwrap_or("Default::default()".to_string())
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
