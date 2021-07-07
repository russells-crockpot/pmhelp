// Since the main crate pulls straight from this one, we have to allow for an unused attribute.
#![allow(unused_attributes)]
#![no_std]
//#![allow(unused_variables, unused_macros, unused_imports, dead_code, unused_mut)]
extern crate alloc;

pub mod parse;
pub mod util;
