#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![no_std]

include!("ogc.rs");

mod inline;
pub use inline::*;
