use crate::pmhelp::parse::token_stream::{comma_separated, parenthesized};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, ExprPath, Field, Ident, Path, Token, Type, TypePath,
};

fn for_int(ident: Ident) -> Result<TokenStream> {
    todo!();
}

fn for_ident(ident: &Ident) -> Result<TokenStream> {
    if ident == "u8"
        || ident == "u16"
        || ident == "u32"
        || ident == "u64"
        || ident == "u128"
        || ident == "i8"
        || ident == "i16"
        || ident == "i32"
        || ident == "i64"
        || ident == "i128"
    {
        // Not sure if this will work with things like hex numbers...
        Ok(quote! {stream.parse::<::syn::LitInt>()?.base10_parse::<#ident>().unwrap()})
    } else if ident == "str" {
        // The returned value is always a String, and any references to it won't last long enough
        panic!("Due to the way syn parses, strs are not supported. Use String instead.")
        //Err(
        //stream.error("Due to the way syn parses, strs are not supported. Use String instead.")
        //)
    } else if ident == "String" {
        Ok(quote! {stream.parse::<::syn::LitStr>()?.value()})
    } else if ident == "char" {
        Ok(quote! {stream.parse::<::syn::LitChar>()?.value()})
    } else {
        Ok(quote! {stream.parse::<#ident>()})
    }
}

fn for_path(path: Path) -> Result<TokenStream> {
    if let Some(ident) = path.get_ident() {
        for_ident(ident)
    } else {
        Ok(quote! {stream.parse::<#path>()})
    }
}

pub fn get_conversion(ty: Type) -> Result<TokenStream> {
    match ty {
        Type::Path(TypePath { path, .. }) => for_path(path),
        _ => todo!(),
    }
}
