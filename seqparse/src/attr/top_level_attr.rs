//! Handles parsing the top level `pmhelp` attribute when deriving an Attribute.
use super::Style;
use crate::pmhelp::parse::{
    parse_stream::comma_separated as comma_separated_ps,
    token_stream::{comma_separated as comma_separated_ts, parenthesized},
};
use heck::SnakeCase as _;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Token, Type,
};

bitflags! {
    #[derive(TryFromParsable)]
    pub struct AllowOn: u8 {
        const Enums = 0x1;
        const Structs = 0x2;
        const Unions = 0x4;
    }
}

impl Default for AllowOn {
    fn default() -> Self {
        Self::all()
    }
}

impl Parse for AllowOn {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut value = Self::empty();
        let stream = from_parens!(stream);
        while !stream.is_empty() {
            if stream.peek(Token![enum]) {
                stream.parse::<Token![enum]>()?;
                value |= Self::Enums;
            } else if stream.peek(Token![struct]) {
                stream.parse::<Token![struct]>()?;
                value |= Self::Structs;
            } else if stream.peek(Token![union]) {
                stream.parse::<Token![union]>()?;
                value |= Self::Unions;
                unimplemented!();
            } else {
                return Err(stream.error("Invalid allow_on choice."));
            }
            if stream.peek(Token![,]) {
                stream.parse::<Token![,]>()?;
            }
        }
        if value.is_empty() {
            Err(stream.error("Empty 'allow_on's are, ironically, not allowed."))
        } else {
            Ok(value)
        }
    }
}

pub enum Entry {
    //Ordered,
    Style(Style),
    AllowOn(AllowOn),
    Name(Ident),
}

impl Entry {
    fn apply(self, attr: &mut TopLevelAttr) {
        match self {
            Self::Style(value) => attr.style = value,
            Self::AllowOn(value) => attr.allow_on = value,
            Self::Name(value) => attr.name = value,
            //Self::Ordered => attr.ordered = true,
        }
    }
}

impl Parse for Entry {
    fn parse(stream: ParseStream) -> Result<Self> {
        let ident = stream.parse::<Ident>()?;
        if ident == "style" {
            Ok(Self::Style(stream.parse()?))
        } else if ident == "allow_on" {
            Ok(Self::AllowOn(stream.parse()?))
        } else if ident == "name" {
            Ok(Self::Name(stream.parse()?))
        } else {
            Err(stream.error(format!("Unknown option: {}", ident)))
        }
    }
}

#[derive(Debug)]
pub struct TopLevelAttr {
    //pub ordered: bool,
    pub style: Style,
    pub allow_on: AllowOn,
    pub name: Ident,
}

impl TopLevelAttr {
    pub fn from_input(ident: &Ident, attrs: Vec<Attribute>) -> Result<Self> {
        let mut me = Self {
            //ordered: false,
            style: Style::default(),
            allow_on: AllowOn::default(),
            name: format_ident!("{}", format!("{}", ident).to_snake_case()),
        };
        if let Some(attr) = attrs.into_iter().find(|attr| attr.path.is_ident("pmhelp")) {
            comma_separated_ts::<Entry>(parenthesized::<TokenStream>(attr.tokens)?)?
                .into_iter()
                .for_each(|e| e.apply(&mut me));
        }
        Ok(me)
    }
}
