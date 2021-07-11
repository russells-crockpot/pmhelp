/// Extension traits to existing types.
use alloc::vec::Vec;
use syn::{
    parse::{Parse, ParseBuffer, Peek},
    GenericArgument, Ident, Path, PathArguments, PathSegment, Type, TypePath,
};

mod is_exts;
pub use is_exts::*;

pub trait ParseBufferExt {
    /// Peeks to see if a token is present. If it is, then it consumes the token and returns true.
    fn peek_and_consume<T: Peek<Token = P>, P: Parse>(&self, token: T) -> bool;
}

impl<'a> ParseBufferExt for ParseBuffer<'a> {
    fn peek_and_consume<T: Peek<Token = P>, P: Parse>(&self, token: T) -> bool {
        if self.peek(token) {
            // Since we KNOW it's the next token, we can just unwrap it
            self.parse::<T::Token>().unwrap();
            true
        } else {
            false
        }
    }
}

pub trait GetBaseTypes {
    fn get_base_types(&self) -> Vec<&Type>;
}

impl GetBaseTypes for PathSegment {
    fn get_base_types(&self) -> Vec<&Type> {
        if let PathArguments::AngleBracketed(generics) = &self.arguments {
            generics
                .args
                .iter()
                .filter_map(|arg| {
                    if let GenericArgument::Type(ty) = arg {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl GetBaseTypes for Path {
    fn get_base_types(&self) -> Vec<&Type> {
        if let Some(last) = self.segments.last() {
            last.get_base_types()
        } else {
            Vec::new()
        }
    }
}

impl GetBaseTypes for Type {
    fn get_base_types(&self) -> Vec<&Type> {
        if let Self::Path(TypePath { path, qself: None }) = &self {
            path.get_base_types()
        } else {
            Vec::new()
        }
    }
}
