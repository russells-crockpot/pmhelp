#![cfg_attr(all(not(feature = "from-base"), not(test)), no_std)]
#![allow(unused_variables, unused_macros, unused_imports, dead_code, unused_mut)]
extern crate alloc;

#[macro_use]
extern crate paste;

#[cfg(not(feature = "from-base"))]
mod macros;

pub mod exts;
pub mod parse;
pub mod util;
