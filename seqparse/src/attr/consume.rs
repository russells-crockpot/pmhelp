use super::SeqParseAttribute;
use crate::pmhelp::parse::token_stream::{comma_separated, parenthesized};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::convert::TryFrom;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, ExprMacro, ExprType, Field, Ident, Macro, Path, Token, Type,
};

enum Consumeable {
    Type(Box<Type>),
    Macro(Macro),
}
impl Parse for Consumeable {
    fn parse(stream: ParseStream) -> Result<Self> {
        match stream.parse::<Expr>()? {
            Expr::Macro(ExprMacro { mac, .. }) => Ok(Self::Macro(mac)),
            Expr::Type(ExprType { ty, .. }) => Ok(Self::Type(ty)),
            other => Err(stream.error(format!("Invalid consumeable: {}", other.to_token_stream()))),
        }
    }
}
impl ToTokens for Consumeable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(val) => val.to_tokens(tokens),
            Self::Macro(val) => val.to_tokens(tokens),
        }
    }
}

pub struct ConsumeAttr {
    ident: Ident,
    types: Punctuated<Consumeable, Token![,]>,
}

impl TryFrom<(usize, Field)> for ConsumeAttr {
    type Error = Error;
    fn try_from((idx, field): (usize, Field)) -> Result<Self> {
        let ident = field.ident.unwrap_or(format_ident!("tmp_{}", idx));
        let attr = field
            .attrs
            .into_iter()
            .find(|attr| attr.path.is_ident("consume"))
            .unwrap();
        Ok(Self {
            ident,
            types: comma_separated(parenthesized::<TokenStream>(attr.tokens)?)?,
        })
    }
}

impl SeqParseAttribute for ConsumeAttr {
    fn get_ident(&self) -> &Ident {
        &self.ident
    }

    fn get_parse_expr(&self) -> TokenStream {
        let types = &self.types;
        quote! {
            {
                #( stream.parse::<#types>()?; )+
                Default::default()
            }
        }
    }
}
