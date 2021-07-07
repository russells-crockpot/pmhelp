use pmhelp::parse::token_stream::comma_separated;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    punctuated::Punctuated,
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Token, Type,
};

enum Implementation {
    ParseStream,
    TokenStream,
}
impl Implementation {
    fn get_type(&self) -> TokenStream {
        match self {
            Self::ParseStream => quote! {::syn::parse::ParseStream<'_>},
            Self::TokenStream => quote! {::proc_macro2::TokenStream},
        }
    }

    fn get_error(&self) -> TokenStream {
        match self {
            Self::ParseStream => quote! {::syn::parse::Error},
            Self::TokenStream => quote! {::syn::parse::Error},
        }
    }

    fn get_method_body(&self) -> TokenStream {
        match self {
            Self::ParseStream => quote! {from.parse()},
            Self::TokenStream => quote! {::syn::parse2(from)},
        }
    }

    #[inline]
    fn parts(&self) -> (TokenStream, TokenStream, TokenStream) {
        (self.get_type(), self.get_error(), self.get_method_body())
    }

    #[inline]
    fn all() -> Vec<Self> {
        vec![Self::TokenStream, Self::ParseStream]
    }
}

impl Parse for Implementation {
    fn parse(stream: ParseStream) -> ParseResult<Self> {
        Ok(Self::from(stream.parse::<Ident>()?))
    }
}

impl From<Ident> for Implementation {
    fn from(ident: Ident) -> Self {
        if ident == "TokenStream" {
            Self::TokenStream
        } else if ident == "ParseStream" {
            Self::ParseStream
        } else {
            panic!("Invalid TryFrom Implementation: {}", ident)
        }
    }
}

fn get_types_to_implement(attrs: Vec<Attribute>) -> Vec<Implementation> {
    for attr in attrs {
        if attr.path.is_ident("try_from_parsable") && !attr.tokens.is_empty() {
            return comma_separated::<Implementation>(attr.tokens)
                .unwrap()
                .into_iter()
                .collect();
        }
    }
    Implementation::all()
}

pub fn derive_try_from_parsable(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_g, type_g, maybe_where) = input.generics.split_for_impl();
    let mut out = TokenStream::new();
    for (ty, error, body) in get_types_to_implement(input.attrs)
        .iter()
        .map(Implementation::parts)
    {
        out.extend(quote! {
            impl #impl_g ::std::convert::TryFrom<#ty> for #name {
                type Error = #error;

                fn try_from(from: #ty) -> ::std::result::Result<Self, Self::Error> {
                    #body
                }
            }
        });
    }
    out
}
