#![no_std]

mod display;
mod engine;

pub mod prelude {
    pub use crate::display::Display;
    pub use crate::engine::{Engine, State};
    pub use embedded_graphics::{self, prelude::*};
    pub use ogc::{self, prelude::*};
}
