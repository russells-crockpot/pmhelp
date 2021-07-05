#![allow(unused_variables, unused_macros, unused_imports, dead_code, unused_mut)]
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[macro_use]
extern crate pmhelp_internal;

extern crate pmhelp_internal as pmhelp;

#[macro_use]
extern crate pmhelp_derive_macros;

#[macro_use]

mod attr;
mod derive;
mod shared;
mod util;

#[proc_macro_derive(Attribute, attributes(pmhelp))]
pub fn attribute(stream: TokenStream) -> TokenStream {
    TokenStream::from(derive::attr::derive_attribute(parse_macro_input!(stream)))
}
