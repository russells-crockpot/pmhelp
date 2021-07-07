//! Handles parsing the `pmhelp` attribute on fields when deriving an Attribute.
use super::Style;
use crate::pmhelp::parse::{parse_stream::parenthesized, token_stream::comma_separated};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, ExprPath, Fields, Ident, Path, Token, Type,
};

pub enum ParserValue {
    Path(Path),
    Expr(Box<Expr>),
}

impl Parse for ParserValue {
    fn parse(stream: ParseStream) -> Result<Self> {
        match stream.parse::<Expr>()? {
            Expr::Path(ExprPath { path, .. }) => Ok(Self::Path(path)),
            expr => Ok(Self::Expr(Box::new(expr))),
        }
    }
}

enum Entry {
    If(Box<Expr>),
    Skip,
    DefaultOnError,
    Style(Style),
    Parser(ParserValue),
    Default(Box<Expr>),
    ConsumeBefore(Punctuated<Type, Token![,]>),
    ConsumeAfter(Punctuated<Type, Token![,]>),
    //TODO add unwrap_or
}

impl Entry {
    fn apply(self, attr: &mut FieldAttr) {
        todo!()
    }
}

impl Parse for Entry {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.peek(Token![if]) {
            Ok(Self::If(Box::new(stream.parse()?)))
        } else {
            let ident = stream.parse::<Ident>()?;
            if ident == "skip" {
                Ok(Self::Skip)
            } else if ident == "default_on_error" {
                Ok(Self::DefaultOnError)
            } else if ident == "style" {
                Ok(Self::Style(stream.parse()?))
            } else if ident == "parser" {
                Ok(Self::Parser(stream.parse()?))
            } else if ident == "Default" {
                Ok(Self::Default(stream.parse()?))
            } else if ident == "consume_before" {
                Ok(Self::ConsumeBefore(comma_separated::<Type>(
                    parenthesized::<TokenStream>(stream)?,
                )?))
            } else if ident == "consume_after" {
                Ok(Self::ConsumeAfter(comma_separated::<Type>(
                    parenthesized::<TokenStream>(stream)?,
                )?))
            } else {
                Err(stream.error(format!("Invalid pmhelp attribute entry: {}", ident)))
            }
        }
    }
}

pub struct FieldAttr {
    predicate: Option<Expr>,
    skip: bool,
    style: Option<Style>,
    parser: Option<ParserValue>,
    default: Option<Expr>,
    consume_before: Option<Entry>,
    consume_after: Option<Entry>,
}
