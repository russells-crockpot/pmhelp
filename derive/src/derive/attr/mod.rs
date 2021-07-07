#![allow(non_upper_case_globals)]
//! Handles deriving attributes;
use crate::pmhelp::parse::parse_stream::comma_separated;
use pmhelp_internal_macros::TryFromParsable;
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Attribute, DeriveInput, Ident, Token,
};

mod conversions;
mod field_attr;
mod top_level_attr;
use top_level_attr::TopLevelAttr;

bitflags! {
    #[derive(TryFromParsable)]
    pub struct Style: u8 {
        const Exprs = 0x1;
        const Strings = 0x2;
        const Parens = 0x4;
        const Equals = 0x8;
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Exprs | Self::Parens
    }
}

impl Parse for Style {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut value = Self::empty();
        let items = comma_separated::<Ident>(from_parens!(stream))?;
        for ident in items {
            if ident == "equals" {
                value |= Self::Equals;
            } else if ident == "parens" {
                value |= Self::Parens;
            } else if ident == "strings" {
                value |= Self::Strings;
            } else if ident == "exprs" {
                value |= Self::Exprs;
            } else {
                return Err(stream.error(format!("Invalid style choice: {}", ident)));
            }
        }
        if !value.intersects(Self::Exprs | Self::Strings) {
            value |= Self::Exprs;
        }
        if !value.intersects(Self::Parens | Self::Equals) {
            value |= Self::Parens;
        }
        if value.contains(Self::Exprs | Self::Strings) {
            Err(stream.error("Only `strings` or `exprs` maybe provided, not both!"))
        } else if value.contains(Self::Parens | Self::Equals) {
            Err(stream.error("Only `parens` or `equals` maybe provided, not both!"))
        } else {
            Ok(value)
        }
    }
}

pub(crate) fn derive_attribute(input: DeriveInput) -> Result<TokenStream> {
    let specs = TopLevelAttr::from_input(&input.ident, input.attrs);
    eprintln!("{:#?}", specs);
    Ok(quote! {})
}
