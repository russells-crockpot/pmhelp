use super::SeqParseAttribute;
use crate::pmhelp::parse::token_stream::{comma_separated, parenthesized};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::convert::TryFrom;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, ExprPath, Field, Ident, Path, Token, Type,
};

pub enum ExprOrPath {
    Path(Path),
    Expr(Box<Expr>),
}

impl Parse for ExprOrPath {
    fn parse(stream: ParseStream) -> Result<Self> {
        match stream.parse::<Expr>()? {
            Expr::Path(ExprPath { path, .. }) => Ok(Self::Path(path)),
            expr => Ok(Self::Expr(Box::new(expr))),
        }
    }
}

enum Entry {
    If(ExprOrPath),
    Skip,
    DefaultOnError,
    Parser(ExprOrPath),
    Default(Box<Expr>),
}

impl Entry {
    fn apply(self, attr: &mut ParseAttr) {
        match self {
            Self::If(value) => attr.predicate = Some(value),
            Self::Skip => attr.skip = true,
            Self::DefaultOnError => attr.default_on_error = true,
            Self::Parser(value) => attr.parser = Some(value),
            Self::Default(value) => attr.default = Some(value),
        }
    }
}

impl Parse for Entry {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.peek(Token![if]) {
            Ok(Self::If(stream.parse()?))
        } else {
            let ident = stream.parse::<Ident>()?;
            if ident == "skip" {
                Ok(Self::Skip)
            } else if ident == "default_on_error" {
                Ok(Self::DefaultOnError)
            } else if ident == "parser" {
                Ok(Self::Parser(stream.parse()?))
            } else if ident == "default" {
                Ok(Self::Default(stream.parse()?))
            } else {
                Err(stream.error(format!("Invalid parse attribute entry: {}", ident)))
            }
        }
    }
}

pub struct ParseAttr {
    ident: Ident,
    ty: Type,
    skip: bool,
    default_on_error: bool,
    predicate: Option<ExprOrPath>,
    parser: Option<ExprOrPath>,
    default: Option<Box<Expr>>,
}

impl ParseAttr {}

impl TryFrom<(usize, Field)> for ParseAttr {
    type Error = Error;
    fn try_from((idx, field): (usize, Field)) -> Result<Self> {
        let attr = field
            .attrs
            .into_iter()
            .find(|attr| attr.path.is_ident("parse"))
            .unwrap();
        let mut me = Self {
            ident: field.ident.unwrap_or(format_ident!("tmp_{}", idx)),
            ty: field.ty,
            skip: false,
            default_on_error: false,
            predicate: None,
            parser: None,
            default: None,
        };
        comma_separated::<Entry>(parenthesized::<TokenStream>(attr.tokens)?)?
            .into_iter()
            .for_each(|e| e.apply(&mut me));
        Ok(me)
    }
}

impl SeqParseAttribute for ParseAttr {
    fn get_ident(&self) -> &Ident {
        &self.ident
    }

    fn get_parse_expr(&self) -> TokenStream {
        todo!();
    }
}
