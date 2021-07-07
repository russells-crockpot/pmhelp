use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result as ParseResult},
};

struct Parenthesized<V: Parse>(V);
impl<V: Parse> Parse for Parenthesized<V> {
    fn parse(stream: ParseStream) -> ParseResult<Self> {
        let content;
        let _ = parenthesized!(content in stream);
        Ok(Self(V::parse(&content)?))
    }
}

/// Functions to parse `proc_macro2::TokenStream`s.
pub mod token_stream {
    use super::Parenthesized;
    use proc_macro2::TokenStream;
    use syn::{
        parse::{Parse, Parser, Result as ParseResult},
        punctuated::Punctuated,
        token::Token,
    };

    #[inline]
    pub fn parenthesized<V: Parse>(stream: TokenStream) -> ParseResult<V> {
        syn::parse2::<Parenthesized<V>>(stream).map(|v| v.0)
    }

    #[inline]
    pub fn separated<P, T>(stream: TokenStream) -> ParseResult<Punctuated<P, T>>
    where
        P: Parse,
        T: Parse + Token,
    {
        Parser::parse2(Punctuated::<P, T>::parse_separated_nonempty, stream)
    }

    macro_rules! make_seperated_func {
        ($name:ident, $token:path) => {
            #[inline]
            pub fn $name<P: Parse>(stream: TokenStream) -> ParseResult<Punctuated<P, $token>> {
                separated::<P, $token>(stream)
            }
        };
    }

    make_seperated_func! {
        comma_separated, ::syn::token::Comma
    }
}

/// Functions to parse `syn::parse::ParseStream`s.
pub mod parse_stream {
    use super::Parenthesized;
    use syn::{
        parse::{Parse, ParseStream, Result as ParseResult},
        punctuated::Punctuated,
    };

    #[inline]
    pub fn parenthesized<V: Parse>(stream: ParseStream) -> ParseResult<V> {
        stream.parse::<Parenthesized<V>>().map(|v| v.0)
    }

    #[inline]
    pub fn separated<P: Parse, T: Parse>(stream: ParseStream) -> ParseResult<Punctuated<P, T>> {
        <Punctuated<P, T>>::parse_terminated(stream)
    }

    macro_rules! make_seperated_func {
        ($name:ident, $token:path) => {
            #[inline]
            pub fn $name<P: Parse>(stream: ParseStream) -> ParseResult<Punctuated<P, $token>> {
                separated::<P, $token>(stream)
            }
        };
    }

    make_seperated_func! {
        comma_separated, ::syn::token::Comma
    }
}
