#![no_std]

//! `ogc-engine` is a simple engine for creating games targeting the Wii.
//! Internally it uses `ogc-rs` and `embedded-graphics`.
//!
//! # Minimal example
//!
//! ```rust
//! #![no_std]
//! #![feature(start)]
//!
//! use ogc_engine::prelude::*;
//!
//! #[start]
//! fn main(_argc: isize, _argv: *const *const u8) -> isize {
//!     Engine::new().run()
//! }
//! ```
//!
//! For more examples, see the
//! [repository](https://github.com/knarkzel/ogc-engine/tree/master/examples)

/// Contains implementation for drawing and manipulating the screen.
pub mod display;

/// Provides necessary abstractions for ergonomic game development.
pub mod engine;

pub mod prelude {
    pub use crate::display::Display;
    pub use crate::engine::{Engine, State};
    pub use embedded_graphics::{self, pixelcolor::Rgb888 as Rgb, prelude::*};
    pub use ogc::{self, prelude::*};
}
