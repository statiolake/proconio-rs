extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Readable)]
pub fn read_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse input.");

    let name = get_name(&ast);
    let fields = get_data(ast).fields;

    let field_info = field_info(&fields);
    let decl = declare(&fields, &name, &field_info);
    let generate = generate(&fields, &name, &field_info);
    let reads = field_info.iter().map(|f| &f.read);

    let res = quote! {
        impl proconio::source::Readable for #name {
            type Output = #name;
            fn read<R: std::io::BufRead>(source: &mut proconio::source::Source<R>) -> #name {
                #(#reads)*
                #generate
            }
        }
    };
    res.into()
}

fn get_name(ast: &syn::DeriveInput) -> syn::Ident {
    ast.ident.clone()
}

fn get_data(ast: syn::DeriveInput) -> syn::DataStruct {
    let data = ast.data;

    match data {
        Data::Struct(data) => data,
        _ => panic!("Readable can only derivable for structs."),
    }
}

struct FieldInfo {
    ident: syn::Ident,
    ty: syn::Type,
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

        res.push(FieldInfo { ident, ty, read });
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

        res.push(FieldInfo { ident, ty, read });
    }

    res
}

fn declare(
    fields: &syn::Fields,
    name: &syn::Ident,
    field_info: &[FieldInfo],
) -> proc_macro2::TokenStream {
    let idents = field_info.iter().map(|f| &f.ident);
    let types = field_info.iter().map(|f| &f.ty);

    match fields {
        Fields::Named(_) => {
            quote! {
                #[derive(Debug)]
                struct #name {
                    #(#idents: #types,)*
                }
            }
        }

        Fields::Unnamed(_) => {
            quote! {
                #[derive(Debug)]
                struct #name(#(#idents,)*);
            }
        }

        Fields::Unit => {
            quote! {
                #[derive(Debug)]
                struct #name;
            }
        }
    }
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
