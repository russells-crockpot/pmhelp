#![allow(non_upper_case_globals)]
//! Handles deriving attributes;
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Attribute, DeriveInput, Ident, Token,
};

pub(crate) fn derive_seqparse(input: DeriveInput) -> Result<TokenStream> {
    Ok(quote! {})
}
