#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    Engine::new().run()
}
