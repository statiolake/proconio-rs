extern crate proc_macro;

use proc_macro::TokenStream;

mod derive_readable;
mod fastout;

#[proc_macro_attribute]
pub fn derive_readable(attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_readable::main(attr, input)
}

#[proc_macro_attribute]
pub fn fastout(attr: TokenStream, input: TokenStream) -> TokenStream {
    fastout::main(attr, input)
}
