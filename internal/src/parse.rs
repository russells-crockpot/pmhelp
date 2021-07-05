use proc_macro2::{Span, TokenStream};
use std::{convert::TryFrom, iter::Cycle, ops::RangeInclusive};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result as ParseResult},
    punctuated::Punctuated,
    GenericParam, Generics, Lifetime, LifetimeDef, Token,
};

struct Parenthesized<V: Parse>(V);
impl<V: Parse> Parse for Parenthesized<V> {
    fn parse(stream: ParseStream) -> ParseResult<Self> {
        let content;
        let _ = parenthesized!(content in stream);
        Ok(Self(V::parse(&content)?))
    }
}

struct CommaDelim<P: Parse>(Punctuated<P, Token![,]>);
impl<P: Parse> Parse for CommaDelim<P> {
    fn parse(stream: ParseStream) -> ParseResult<Self> {
        let parser = Punctuated::<P, Token![,]>::parse_separated_nonempty;
        Ok(Self(parser(stream)?))
    }
}

#[inline]
pub fn parenthesized<V: Parse>(stream: TokenStream) -> ParseResult<V> {
    syn::parse2::<Parenthesized<V>>(stream).map(|v| v.0)
}

#[inline]
pub fn parenthesized2<V: Parse>(stream: ParseStream) -> ParseResult<V> {
    stream.parse::<Parenthesized<V>>().map(|v| v.0)
}

#[inline]
pub fn separated<P: Parse, T: Parse>(stream: ParseStream) -> ParseResult<Punctuated<P, T>> {
    <Punctuated<P, T>>::parse_terminated(stream)
}

#[inline]
pub fn comma_separated<P: Parse>(stream: ParseStream) -> ParseResult<Punctuated<P, Token![,]>> {
    separated::<P, Token![,]>(stream)
}
