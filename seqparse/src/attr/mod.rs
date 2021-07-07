use proc_macro2::TokenStream;
use std::convert::TryFrom;
use syn::{parse::Error, Field, Ident};

pub(crate) mod consume;
pub(crate) mod conversion;
pub(crate) mod parse;

pub trait SeqParseAttribute: TryFrom<(usize, Field)> {
    fn get_ident(&self) -> &Ident;
    fn get_parse_expr(&self) -> TokenStream;
}
