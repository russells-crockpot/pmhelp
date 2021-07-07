#![allow(unused_variables, unused_macros, unused_imports, dead_code, unused_mut)]
use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate pmhelp_internal as pmhelp;

mod derive;

#[proc_macro_derive(TryFromParsable, attributes(try_from_parsable))]
pub fn try_from_parsable(stream: TokenStream) -> TokenStream {
    TokenStream::from(derive::derive_try_from_parsable(parse_macro_input!(stream)))
}
