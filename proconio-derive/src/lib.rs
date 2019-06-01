extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(ReadSource)]
pub fn read_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse input.");

    let name = &ast.ident;
    let data = ast.data;
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("ReadSource can only derivable for structs."),
    };
    let fields = data.fields;

    let (idents, read) = match fields {
        Fields::Named(_) => fields
            .iter()
            .map(|field| {
                let ident = field
                    .ident
                    .as_ref()
                    .cloned()
                    .expect("internal error: named field doesn't have name");
                let ty = &field.ty;
                (
                    ident.clone(),
                    quote! {
                        let #ident = <#ty as proconio::source::ReadSource>::read(source);
                    },
                )
            })
            .unzip(),
        Fields::Unnamed(_) => fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let ident = format!("field{}", idx);
                let ident = syn::Ident::new(&ident, Span::call_site());
                let ty = &field.ty;
                (
                    ident.clone(),
                    quote! {
                        let #ident = <#ty as proconio::source::ReadSource>::read(source);
                    },
                )
            })
            .unzip(),
        Fields::Unit => (Vec::new(), Vec::new()),
    };

    let generate = match fields {
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
    };

    let gen = quote! {
        impl proconio::source::ReadSource for #name {
            type Output = #name;
            fn read<R: std::io::BufRead>(source: &mut proconio::source::Source<R>) -> #name {
                #(#read)*
                #generate
            }
        }
    };

    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
