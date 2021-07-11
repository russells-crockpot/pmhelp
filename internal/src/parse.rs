use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
};

struct Parenthesized<V: Parse>(V);
impl<V: Parse> Parse for Parenthesized<V> {
    fn parse(stream: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in stream);
        Ok(Self(V::parse(&content)?))
    }
}
macro_rules! make_seperated_func {
    ($name:ident, $token:path, $type:ident) => {
        #[inline]
        pub fn $name<P: Parse>(stream: paste! {[<$type Stream>]}) -> Result<Punctuated<P, $token>> {
            separated::<P, $token>(stream)
        }
    };
}

/// Functions to parse `proc_macro2::TokenStream`s.
pub mod token_stream {
    use super::Parenthesized;
    use proc_macro2::TokenStream;
    use syn::{
        parse::{Parse, Parser, Result},
        punctuated::Punctuated,
        token::Token,
    };

    #[inline]
    pub fn parenthesized<V: Parse>(stream: TokenStream) -> Result<V> {
        syn::parse2::<Parenthesized<V>>(stream).map(|v| v.0)
    }

    #[inline]
    pub fn separated<P, T>(stream: TokenStream) -> Result<Punctuated<P, T>>
    where
        P: Parse,
        T: Parse + Token,
    {
        Parser::parse2(Punctuated::<P, T>::parse_separated_nonempty, stream)
    }

    make_seperated_func! {
        comma_separated, ::syn::token::Comma, Token
    }
}

/// Functions to parse `syn::parse::ParseStream`s.
pub mod parse_stream {
    use super::Parenthesized;
    use syn::{
        parse::{Parse, ParseStream, Result},
        punctuated::Punctuated,
    };

    #[inline]
    pub fn parenthesized<V: Parse>(stream: ParseStream) -> Result<V> {
        stream.parse::<Parenthesized<V>>().map(|v| v.0)
    }

    #[inline]
    pub fn separated<P: Parse, T: Parse>(stream: ParseStream) -> Result<Punctuated<P, T>> {
        <Punctuated<P, T>>::parse_terminated(stream)
    }

    make_seperated_func! {
        comma_separated, ::syn::token::Comma, Parse
    }
}
