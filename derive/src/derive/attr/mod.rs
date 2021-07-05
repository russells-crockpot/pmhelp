//! Handles deriving attributes;
use crate::pmhelp::parse::comma_separated;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    punctuated::Punctuated,
    DeriveInput, Ident, Token,
};

mod field_attr;
mod top_level_attr;

enum Style {
    EqualsStrings,
    EqualsExpressions,
    ParensStrings,
    ParensExpressions,
}

impl Parse for Style {
    fn parse(stream: ParseStream) -> ParseResult<Self> {
        let mut parens = false;
        let mut strings = false;
        //let items = comma_separated(from_parens!(&stream))?;
        todo!();
    }
}

pub(crate) fn derive_attribute(input: DeriveInput) -> TokenStream {
    todo!()
}
