//! Handles parsing the `pmhelp` attribute on fields when deriving an Attribute.
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Token, Type,
};
