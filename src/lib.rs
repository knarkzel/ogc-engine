#![no_std]
#![feature(associated_type_defaults)]

//! `ogc-engine` is a simple engine for creating games targeting the Wii.
//! Internally it uses `ogc-rs` and `embedded-graphics`.
//!
//! # Example
//!
//! ```rust
//! #![no_std]
//! #![feature(start)]
//!
//! use ogc_engine::prelude::*;
//!
//! struct Game;
//!
//! impl State for Game {}
//!
//! #[start]
//! fn main(_argc: isize, _argv: *const *const u8) -> isize {
//!     let state = Game;
//!     Engine::run(state)
//! }
//! ```
//!
//! For more examples, see the
//! [repository](https://github.com/knarkzel/ogc-engine/tree/master/examples)

/// Contains implementation for drawing and manipulating the screen.
pub mod display;

/// Provides necessary abstractions for ergonomic game development.
pub mod engine;

/// Draw error abstraction.
pub type DrawError = core::convert::Infallible;

pub mod prelude {
    pub use super::DrawError;
    pub use crate::display::Display;
    pub use crate::engine::{Engine, State};
    pub use embedded_graphics::{self, pixelcolor::Rgb888 as Rgb, prelude::*};
    pub use ogc::{self, prelude::*};
}
