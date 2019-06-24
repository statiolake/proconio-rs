// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        let mut attr = attr.into_iter();
        let start = attr
            .next()
            .expect("Attribute is empty.  This is a bug.")
            .span();
        let end = attr.fold(start, |_, item| item.span());
        let compile_error = crate::compile_error_at(
            quote!("no extra attribute is suppported."),
            Span::from(start),
            Span::from(end),
        );

        return compile_error.into_token_stream().into();
    }

    let mut ast = parse_macro_input!(input as DeriveInput);

    // derive actually Readable
    let derive = match derive_readable_impl(&ast) {
        Ok(derive) => derive,
        Err(error) => return error,
    };

    // modify AST to use actual Readable::Output type
    if let Err(error) = replace_type(&mut ast) {
        return error;
    }

    quote!(#ast #derive).into()
}

fn replace_type(ast: &mut DeriveInput) -> Result<(), TokenStream> {
    let data = get_data_mut(ast)?;

    for field in data.fields.iter_mut() {
        let new_ty: syn::Type = {
            let ty = field.ty.clone().into_token_stream();
            syn::parse_quote!(<#ty as proconio::source::Readable>::Output)
        };

        std::mem::replace(&mut field.ty, new_ty);
    }

    Ok(())
}

fn derive_readable_impl(ast: &DeriveInput) -> Result<proc_macro2::TokenStream, TokenStream> {
    let name = get_name(ast);
    let fields = &get_data(ast)?.fields;

    let field_info = field_info(&fields);
    let generate = generate(&fields, &name, &field_info);
    let reads = field_info.iter().map(|f| &f.read);

    let res = quote! {
        impl proconio::source::Readable for #name {
            type Output = #name;
            fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> #name {
                #(#reads)*
                #generate
            }
        }
    };

    Ok(res)
}

fn get_name(ast: &syn::DeriveInput) -> syn::Ident {
    ast.ident.clone()
}

fn get_data(ast: &syn::DeriveInput) -> Result<&syn::DataStruct, TokenStream> {
    let start = ast.span();
    let end = ast.ident.span();

    let data = &ast.data;

    match data {
        Data::Struct(data) => Ok(data),
        _ => Err(crate::compile_error_at(
            quote!("Readable can only derivable for structs."),
            start,
            end,
        )
        .into_token_stream()
        .into()),
    }
}

fn get_data_mut(ast: &mut syn::DeriveInput) -> Result<&mut syn::DataStruct, TokenStream> {
    let start = ast.span();
    let end = ast.ident.span();
    let data = &mut ast.data;

    match data {
        Data::Struct(data) => Ok(data),
        _ => Err(crate::compile_error_at(
            quote!("Readable can only derivable for structs."),
            start,
            end,
        )
        .into_token_stream()
        .into()),
    }
}

struct FieldInfo {
    ident: syn::Ident,
    read: proc_macro2::TokenStream,
}

fn field_info(fields: &syn::Fields) -> Vec<FieldInfo> {
    match fields {
        Fields::Named(_) => field_named(fields),
        Fields::Unnamed(_) => field_unnamed(fields),
        Fields::Unit => Vec::new(),
    }
}

fn field_named(fields: &syn::Fields) -> Vec<FieldInfo> {
    let mut res = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().cloned();
        let ident = ident.expect("Named field doesn't have name.  This is a bug.");
        let ty = field.ty.clone();
        let read = quote! {
            let #ident = <#ty as proconio::source::Readable>::read(source);
        };

        res.push(FieldInfo { ident, read });
    }

    res
}

fn field_unnamed(fields: &syn::Fields) -> Vec<FieldInfo> {
    let mut res = Vec::new();

    for (idx, field) in fields.iter().enumerate() {
        let ident = format!("field{}", idx);
        let ident = syn::Ident::new(&ident, Span::call_site());
        let ty = field.ty.clone();
        let read = quote! {
            let #ident = <#ty as proconio::source::Readable>::read(source);
        };

        res.push(FieldInfo { ident, read });
    }

    res
}

fn generate(
    fields: &syn::Fields,
    name: &syn::Ident,
    field_info: &[FieldInfo],
) -> proc_macro2::TokenStream {
    let idents = field_info.iter().map(|f| &f.ident);

    match fields {
        Fields::Named(_) => {
            quote! {
                #name {
                    #(#idents,)*
                }
            }
        }

        Fields::Unnamed(_) => {
            quote! {
                #name(#(#idents,)*)
            }
        }

        Fields::Unit => {
            quote! {
                #name
            }
        }
    }
}
