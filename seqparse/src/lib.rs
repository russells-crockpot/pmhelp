#![allow(unused_variables, unused_macros, unused_imports, dead_code, unused_mut)]
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate pmhelp_internal;

extern crate pmhelp_internal as pmhelp;

//mod expanded;

mod attr;
mod base;

#[proc_macro_derive(SeqParse, attributes(parse, consume))]
pub fn attribute(stream: TokenStream) -> TokenStream {
    TokenStream::from(base::derive_seqparse(parse_macro_input!(stream)).unwrap())
}
