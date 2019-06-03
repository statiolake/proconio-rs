extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn derive_readable(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("no extra attribute is supported.");
    }

    let mut ast: DeriveInput = syn::parse(input).expect("failed to parse the struct definition.");

    // derive actually Readable
    let derive = derive_readable_impl(&ast);

    // modify AST to use actual Readable::Output type
    replace_type(&mut ast);

    quote!(#ast #derive).into()
}

fn replace_type(ast: &mut DeriveInput) {
    use quote::ToTokens;

    let data = match &mut ast.data {
        Data::Struct(data) => data,
        _ => panic!("Readable is only derivable for structs."),
    };

    for field in data.fields.iter_mut() {
        let new_ty: syn::Type = {
            let ty = field.ty.clone().into_token_stream();
            syn::parse_quote!(<#ty as proconio::source::Readable>::Output)
        };

        std::mem::replace(&mut field.ty, new_ty);
    }
}

fn derive_readable_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = get_name(ast);
    let fields = &get_data(ast).fields;

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

    res
}

fn get_name(ast: &syn::DeriveInput) -> syn::Ident {
    ast.ident.clone()
}

fn get_data(ast: &syn::DeriveInput) -> &syn::DataStruct {
    let data = &ast.data;

    match data {
        Data::Struct(data) => data,
        _ => panic!("Readable can only derivable for structs."),
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
        let ident = ident.expect("internal error: named field doesn't have name");
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
