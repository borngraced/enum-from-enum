///! Rust Derive Impl from enum
///
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use quote::ToTokens;
use quote::__private::ext::RepToTokensExt;
use quote::quote_spanned;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, DeriveInput};

/// `EnumFromEnum` is very useful for generating `From<T>` trait from one enum to another enum
/// Currently, this crate can only convert enum variant with only some basic inner type such as `String`, and `Enum`
/// type just like the example below. Can not be used for tuple, struct etc for now .
///
/// More support will be added soon
///
///
/// ### USAGE:
/// ```rust
/// use enum_from_enum::EnumFromEnum;
/// use derive_more::Display;
///
///  // E.G, this converts from whatever Bar is to Foo::Bar(String) and
/// // whatever FooBar to Foo::FooBar(FooBar)
/// #[derive(Debug, EnumFromEnum)]
/// pub enum Foo {
///     #[enum_from_enum("Bar")]
///     Bar(String),
///     #[enum_from_enum("FooBar")]
///     FooBar(FooBar),
/// }
///
/// #[derive(Debug, Display)]
/// pub enum Bar {
///     Foo(String),
/// }
///
/// #[derive(Debug, Display)]
/// pub enum FooBar {
///     Foo(String),
/// }
///
/// fn foo_fn() -> Result<(), Foo> {
///     Ok(bar_fn()?)
/// }
///
/// fn bar_fn() -> Result<(), Bar> {
///     Err(Bar::Foo("Err".to_string()))
/// }
/// ```
///
///
///
#[proc_macro_derive(EnumFromEnum, attributes(enum_from_enum))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
        variants
    } else {
        panic!("Couldn't fetch variants")
    };

    let construct_meta_binding = map_field_meta(variants);
    let construct_meta = construct_meta_binding.iter().map(|m| {
        let variant_ident = &m.variant_ident;
        if let syn::NestedMeta::Lit(syn::Lit::Str(str)) = &m.meta {
            if str.value().is_empty() {
            return Some(quote_spanned!(
                str.span() => compile_error!("Expected this to take a `type`")
                ))
            };
            let lit_ident = syn::Ident::new_raw(&str.value(), str.span());
            let inner_ident = get_inner_ident(m.inner_ident.to_owned());
            return match inner_ident {
                InnerIdentTypes::Named => Some(quote! {
                    impl From<#lit_ident> for #name {
                        fn from(err: #lit_ident) -> #name {
                            #name::#variant_ident(err)
                        }
                    }
                }),
                _ => Some(quote! {
                    impl From<#lit_ident> for #name {
                        fn from(err: #lit_ident) -> #name {
                            #name::#variant_ident(err.to_string())
                        }
                    }
                }),
            };
        }
        None
    });

    quote!(#(#construct_meta)*).into()
}

#[derive(Debug, Clone)]
pub(crate) struct MapEnumNestedIter {
    variant_ident: Ident,
    nested_meta: Punctuated<syn::NestedMeta, Comma>,
    inner_ident: Option<Ident>,
}

#[derive(Debug, Clone)]
pub(crate) struct MapEnumMeta {
    pub(crate) variant_ident: Ident,
    pub(crate) meta: syn::NestedMeta,
    pub(crate) inner_ident: Option<Ident>,
}

#[derive(Debug)]
pub(crate) enum InnerIdentTypes {
    String,
    Named,
    Unnamed,
}

pub(crate) fn get_inner_ident(ident: Option<Ident>) -> InnerIdentTypes {
    if let Some(ident) = ident {
        let n = Ident::new("String", ident.span());
        return if ident == n {
            InnerIdentTypes::String
        } else {
            InnerIdentTypes::Named
        };
    }
    InnerIdentTypes::Unnamed
}

pub(crate) fn get_attributes(variants: syn::Variant) -> Result<MapEnumNestedIter, syn::Error> {
    let variant_ident = &variants.ident;
    let fields = &variants.fields;
    let attributes = &variants.attrs; 
    for attribute in attributes {
        if let Ok(meta) = attribute.parse_meta() {
            match meta {
                syn::Meta::List(syn::MetaList { nested, .. }) => {
                    if let Some(ident) = get_variant_unnamed_ident(fields.to_owned()) {
                        return syn::Result::Ok(MapEnumNestedIter {
                            variant_ident: variant_ident.to_owned(),
                            nested_meta: nested.to_owned(),
                            inner_ident: Some(ident.to_owned()),
                        });
                    } 
                    return syn::Result::Ok(MapEnumNestedIter {
                        variant_ident: variant_ident.to_owned(),
                        nested_meta: nested.to_owned(),
                        inner_ident: None,
                    });
                }
                _ => {
                    return syn::Result::Err(syn::Error::new_spanned(
                        attribute.clone().tokens,
                        "expected #[enum_from_displaying(..)]".to_string(),
                    ));
                }
            };
        };
    }
    return syn::Result::Err(syn::Error::new_spanned(
        variant_ident.to_token_stream(),
        "Operation Error.".to_string(),
    ));
}

pub(crate) fn get_variant_unnamed_ident(fields: syn::Fields) -> Option<Ident> {
    if let syn::Fields::Unnamed(fields_unnamed) = fields {
        let syn::FieldsUnnamed { unnamed, .. } = fields_unnamed;
        if let Some(field) = unnamed.iter().next() {
            let type_path = if let Some(syn::Type::Path(type_path, ..)) = field.ty.next().cloned() {
                type_path
            } else {
                return None;
            };
            let path_segment = type_path.path.segments.iter().next().cloned()?;
            return Some(path_segment.ident);
        };
    }
    None
}

pub(crate) fn map_field_meta(variants: Punctuated<syn::Variant, Comma>) -> Vec<MapEnumMeta> {
    let mut meta_vec = vec![];
    for variant in variants.iter() {
        let _ = get_attributes(variant.to_owned()).map(|attr| {
            for meta in attr.clone().nested_meta.iter() {
                let variant_ident = attr.clone().variant_ident.to_owned();
                meta_vec.push(MapEnumMeta {
                    variant_ident,
                    meta: meta.clone(),
                    inner_ident: attr.inner_ident.clone(),
                });
            }
        });
    }
    meta_vec
}
