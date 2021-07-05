//! Handles parsing the top level `pmhelp` attribute when deriving an Attribute.
use super::Style;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Token, Type,
};

enum Entry {
    DefaultStyle(Style),
}
